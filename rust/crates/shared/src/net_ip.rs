//! IP address parsing and classification — mirrors src/shared/net/ip.ts
//!
//! Uses `std::net` for base parsing plus manual range classification
//! to match the behaviour of the TypeScript `ipaddr.js` library.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Parsed IP address (either v4 or v6).
pub type ParsedIpAddress = IpAddr;

// ---------------------------------------------------------------------------
// IPv4 range classification
// ---------------------------------------------------------------------------

/// IPv4 special-use range categories (matching ipaddr.js naming).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ipv4Range {
    Unspecified,
    Broadcast,
    Multicast,
    LinkLocal,
    Loopback,
    CarrierGradeNat,
    Private,
    Reserved,
    Unicast,
}

/// Classify an IPv4 address into its special-use range.
pub fn ipv4_range(addr: Ipv4Addr) -> Ipv4Range {
    let octets = addr.octets();
    if addr.is_unspecified() {
        return Ipv4Range::Unspecified;
    }
    if octets == [255, 255, 255, 255] {
        return Ipv4Range::Broadcast;
    }
    if addr.is_multicast() {
        return Ipv4Range::Multicast;
    }
    if addr.is_link_local() {
        return Ipv4Range::LinkLocal;
    }
    if addr.is_loopback() {
        return Ipv4Range::Loopback;
    }
    // Carrier-grade NAT: 100.64.0.0/10
    if octets[0] == 100 && (octets[1] & 0xC0) == 64 {
        return Ipv4Range::CarrierGradeNat;
    }
    if addr.is_private() {
        return Ipv4Range::Private;
    }
    // Reserved ranges not covered by std (240.0.0.0/4 minus broadcast, and others)
    if octets[0] >= 240 {
        return Ipv4Range::Reserved;
    }
    // 192.0.0.0/24 (IETF protocol assignments)
    if octets[0] == 192 && octets[1] == 0 && octets[2] == 0 {
        return Ipv4Range::Reserved;
    }
    // 192.0.2.0/24 (TEST-NET-1)
    if octets[0] == 192 && octets[1] == 0 && octets[2] == 2 {
        return Ipv4Range::Reserved;
    }
    // 198.51.100.0/24 (TEST-NET-2)
    if octets[0] == 198 && octets[1] == 51 && octets[2] == 100 {
        return Ipv4Range::Reserved;
    }
    // 203.0.113.0/24 (TEST-NET-3)
    if octets[0] == 203 && octets[1] == 0 && octets[2] == 113 {
        return Ipv4Range::Reserved;
    }
    Ipv4Range::Unicast
}

/// Set of IPv4 ranges that are blocked for special use.
fn is_blocked_ipv4_range(range: Ipv4Range) -> bool {
    matches!(
        range,
        Ipv4Range::Unspecified
            | Ipv4Range::Broadcast
            | Ipv4Range::Multicast
            | Ipv4Range::LinkLocal
            | Ipv4Range::Loopback
            | Ipv4Range::CarrierGradeNat
            | Ipv4Range::Private
            | Ipv4Range::Reserved
    )
}

fn is_private_or_loopback_ipv4_range(range: Ipv4Range) -> bool {
    matches!(
        range,
        Ipv4Range::Loopback | Ipv4Range::Private | Ipv4Range::LinkLocal | Ipv4Range::CarrierGradeNat
    )
}

// ---------------------------------------------------------------------------
// IPv6 range classification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ipv6Range {
    Unspecified,
    Loopback,
    LinkLocal,
    UniqueLocal,
    Multicast,
    Unicast,
}

pub fn ipv6_range(addr: Ipv6Addr) -> Ipv6Range {
    if addr.is_unspecified() {
        return Ipv6Range::Unspecified;
    }
    if addr.is_loopback() {
        return Ipv6Range::Loopback;
    }
    let segments = addr.segments();
    // Link-local: fe80::/10
    if (segments[0] & 0xFFC0) == 0xFE80 {
        return Ipv6Range::LinkLocal;
    }
    // Unique local: fc00::/7
    if (segments[0] & 0xFE00) == 0xFC00 {
        return Ipv6Range::UniqueLocal;
    }
    if addr.is_multicast() {
        return Ipv6Range::Multicast;
    }
    Ipv6Range::Unicast
}

fn is_blocked_ipv6_range(range: Ipv6Range) -> bool {
    matches!(
        range,
        Ipv6Range::Unspecified
            | Ipv6Range::Loopback
            | Ipv6Range::LinkLocal
            | Ipv6Range::UniqueLocal
            | Ipv6Range::Multicast
    )
}

// ---------------------------------------------------------------------------
// RFC 2544 benchmark prefix: 198.18.0.0/15
// ---------------------------------------------------------------------------

fn is_in_rfc2544_benchmark_range(addr: Ipv4Addr) -> bool {
    let octets = addr.octets();
    octets[0] == 198 && (octets[1] & 0xFE) == 18
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

fn strip_ipv6_brackets(value: &str) -> &str {
    if value.starts_with('[') && value.ends_with(']') {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

fn is_numeric_ipv4_literal_part(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
        return !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit());
    }
    value.chars().all(|c| c.is_ascii_digit())
}

/// Check if a string is a canonical four-part dotted decimal IPv4 address.
fn is_valid_four_part_decimal(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    for part in &parts {
        if part.is_empty() || part.len() > 3 {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        // No leading zeros (except "0" itself)
        if part.len() > 1 && part.starts_with('0') {
            return false;
        }
        let val: u32 = match part.parse() {
            Ok(v) => v,
            Err(_) => return false,
        };
        if val > 255 {
            return false;
        }
    }
    true
}

/// Normalize IPv4-mapped IPv6 to plain IPv4.
fn normalize_ipv4_mapped(addr: IpAddr) -> IpAddr {
    match addr {
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                IpAddr::V4(v4)
            } else {
                addr
            }
        }
        _ => addr,
    }
}

fn normalize_ip_parse_input(raw: Option<&str>) -> Option<&str> {
    let trimmed = raw?.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(strip_ipv6_brackets(trimmed))
}

// ---------------------------------------------------------------------------
// Public parsing API
// ---------------------------------------------------------------------------

/// Parse an IP address in canonical form (only four-part decimal for IPv4).
pub fn parse_canonical_ip_address(raw: Option<&str>) -> Option<IpAddr> {
    let normalized = normalize_ip_parse_input(raw)?;

    // Try IPv4 first — only accept canonical four-part decimal
    if is_valid_four_part_decimal(normalized) {
        return normalized.parse::<Ipv4Addr>().ok().map(IpAddr::V4);
    }

    // Try IPv6
    if let Ok(v6) = normalized.parse::<Ipv6Addr>() {
        return Some(IpAddr::V6(v6));
    }

    // Try IPv6 with embedded IPv4 (e.g. ::ffff:192.168.1.1)
    parse_ipv6_with_embedded_ipv4(normalized).map(IpAddr::V6)
}

/// Parse an IP address loosely (accept legacy IPv4 forms too via std).
pub fn parse_loose_ip_address(raw: Option<&str>) -> Option<IpAddr> {
    let normalized = normalize_ip_parse_input(raw)?;
    if let Ok(addr) = normalized.parse::<IpAddr>() {
        return Some(addr);
    }
    parse_ipv6_with_embedded_ipv4(normalized).map(IpAddr::V6)
}

fn parse_ipv6_with_embedded_ipv4(raw: &str) -> Option<Ipv6Addr> {
    if !raw.contains(':') || !raw.contains('.') {
        return None;
    }
    // Match prefix:w.x.y.z pattern
    let re = regex::Regex::new(r"^(.*:)([^:%]+(?:\.[^:%]+){3})(%[0-9A-Za-z]+)?$").ok()?;
    let caps = re.captures(raw)?;
    let prefix = caps.get(1)?.as_str();
    let embedded_ipv4 = caps.get(2)?.as_str();
    let zone_suffix = caps.get(3).map(|m| m.as_str()).unwrap_or("");

    if !is_valid_four_part_decimal(embedded_ipv4) {
        return None;
    }

    let octets: Vec<u8> = embedded_ipv4
        .split('.')
        .filter_map(|p| p.parse::<u8>().ok())
        .collect();
    if octets.len() != 4 {
        return None;
    }

    let high = (u16::from(octets[0]) << 8) | u16::from(octets[1]);
    let low = (u16::from(octets[2]) << 8) | u16::from(octets[3]);
    let normalized = format!("{}{:x}:{:x}{}", prefix, high, low, zone_suffix);
    normalized.parse::<Ipv6Addr>().ok()
}

/// Normalize an IP address to its canonical string form.
pub fn normalize_ip_address(raw: Option<&str>) -> Option<String> {
    let parsed = parse_canonical_ip_address(raw)?;
    let normalized = normalize_ipv4_mapped(parsed);
    Some(normalized.to_string())
}

/// Check if a string is a canonical dotted-decimal IPv4 address.
pub fn is_canonical_dotted_decimal_ipv4(raw: Option<&str>) -> bool {
    let Some(trimmed) = raw.map(|s| s.trim()) else {
        return false;
    };
    if trimmed.is_empty() {
        return false;
    }
    let normalized = strip_ipv6_brackets(trimmed);
    is_valid_four_part_decimal(normalized)
}

/// Check if a string is a legacy (non-canonical) IPv4 literal.
pub fn is_legacy_ipv4_literal(raw: Option<&str>) -> bool {
    let Some(trimmed) = raw.map(|s| s.trim()) else {
        return false;
    };
    if trimmed.is_empty() {
        return false;
    }
    let normalized = strip_ipv6_brackets(trimmed);
    if normalized.is_empty() || normalized.contains(':') {
        return false;
    }
    if is_canonical_dotted_decimal_ipv4(Some(normalized)) {
        return false;
    }
    let parts: Vec<&str> = normalized.split('.').collect();
    if parts.is_empty() || parts.len() > 4 {
        return false;
    }
    if parts.iter().any(|p| p.is_empty()) {
        return false;
    }
    parts.iter().all(|p| is_numeric_ipv4_literal_part(p))
}

/// Check if an IP address is a loopback address.
pub fn is_loopback_ip_address(raw: Option<&str>) -> bool {
    let Some(parsed) = parse_canonical_ip_address(raw) else {
        return false;
    };
    let normalized = normalize_ipv4_mapped(parsed);
    normalized.is_loopback()
}

/// Check if an IP address is private or loopback.
pub fn is_private_or_loopback_ip_address(raw: Option<&str>) -> bool {
    let Some(parsed) = parse_canonical_ip_address(raw) else {
        return false;
    };
    let normalized = normalize_ipv4_mapped(parsed);
    match normalized {
        IpAddr::V4(v4) => is_private_or_loopback_ipv4_range(ipv4_range(v4)),
        IpAddr::V6(v6) => is_blocked_special_use_ipv6_address(v6),
    }
}

/// Check if an IPv6 address is in a blocked special-use range.
pub fn is_blocked_special_use_ipv6_address(addr: Ipv6Addr) -> bool {
    if is_blocked_ipv6_range(ipv6_range(addr)) {
        return true;
    }
    // Deprecated site-local fec0::/10
    (addr.segments()[0] & 0xFFC0) == 0xFEC0
}

/// Check if an IPv4 address is in RFC 1918 private range.
pub fn is_rfc1918_ipv4_address(raw: Option<&str>) -> bool {
    let Some(parsed) = parse_canonical_ip_address(raw) else {
        return false;
    };
    match parsed {
        IpAddr::V4(v4) => ipv4_range(v4) == Ipv4Range::Private,
        _ => false,
    }
}

/// Check if an IPv4 address is in the carrier-grade NAT range.
pub fn is_carrier_grade_nat_ipv4_address(raw: Option<&str>) -> bool {
    let Some(parsed) = parse_canonical_ip_address(raw) else {
        return false;
    };
    match parsed {
        IpAddr::V4(v4) => ipv4_range(v4) == Ipv4Range::CarrierGradeNat,
        _ => false,
    }
}

/// Options for IPv4 special-use blocking.
#[derive(Debug, Clone, Default)]
pub struct Ipv4SpecialUseBlockOptions {
    pub allow_rfc2544_benchmark_range: bool,
}

/// Check if an IPv4 address is in a blocked special-use range.
pub fn is_blocked_special_use_ipv4_address(
    addr: Ipv4Addr,
    options: &Ipv4SpecialUseBlockOptions,
) -> bool {
    let in_rfc2544 = is_in_rfc2544_benchmark_range(addr);
    if in_rfc2544 && options.allow_rfc2544_benchmark_range {
        return false;
    }
    is_blocked_ipv4_range(ipv4_range(addr)) || in_rfc2544
}

/// Decode an IPv4 address from two 16-bit hextets.
fn decode_ipv4_from_hextets(high: u16, low: u16) -> Ipv4Addr {
    Ipv4Addr::new(
        ((high >> 8) & 0xFF) as u8,
        (high & 0xFF) as u8,
        ((low >> 8) & 0xFF) as u8,
        (low & 0xFF) as u8,
    )
}

/// Extract an embedded IPv4 address from an IPv6 address (mapped, 6to4, Teredo, etc.).
pub fn extract_embedded_ipv4_from_ipv6(addr: Ipv6Addr) -> Option<Ipv4Addr> {
    // IPv4-mapped ::ffff:w.x.y.z
    if let Some(v4) = addr.to_ipv4_mapped() {
        return Some(v4);
    }

    let parts = addr.segments();

    // RFC 6145 / RFC 6052 (well-known prefix 64:ff9b::/96)
    if parts[0] == 0x0064 && parts[1] == 0xFF9B {
        if parts[2] == 0 && parts[3] == 0 && parts[4] == 0 && parts[5] == 0 {
            return Some(decode_ipv4_from_hextets(parts[6], parts[7]));
        }
    }

    // Sentinel rules (matching TS EMBEDDED_IPV4_SENTINEL_RULES)

    // IPv4-compatible ::w.x.y.z
    if parts[0] == 0 && parts[1] == 0 && parts[2] == 0 && parts[3] == 0
        && parts[4] == 0 && parts[5] == 0
    {
        return Some(decode_ipv4_from_hextets(parts[6], parts[7]));
    }

    // NAT64 local-use prefix 64:ff9b:1::/48
    if parts[0] == 0x0064 && parts[1] == 0xFF9B && parts[2] == 0x0001
        && parts[3] == 0 && parts[4] == 0 && parts[5] == 0
    {
        return Some(decode_ipv4_from_hextets(parts[6], parts[7]));
    }

    // 6to4: 2002::/16 — IPv4 in hextets 1..2
    if parts[0] == 0x2002 {
        return Some(decode_ipv4_from_hextets(parts[1], parts[2]));
    }

    // Teredo: 2001:0000::/32 — client IPv4 XOR 0xffff in hextets 6..7
    if parts[0] == 0x2001 && parts[1] == 0x0000 {
        return Some(decode_ipv4_from_hextets(parts[6] ^ 0xFFFF, parts[7] ^ 0xFFFF));
    }

    // ISATAP: ....:0000:5efe:w.x.y.z (u/g bits allowed in hextet 4)
    if (parts[4] & 0xFCFF) == 0 && parts[5] == 0x5EFE {
        return Some(decode_ipv4_from_hextets(parts[6], parts[7]));
    }

    None
}

/// Check if an IP is within a CIDR range (or exact match if no prefix).
pub fn is_ip_in_cidr(ip: &str, cidr: &str) -> bool {
    let Some(normalized_ip) = parse_canonical_ip_address(Some(ip)) else {
        return false;
    };
    let candidate = cidr.trim();
    if candidate.is_empty() {
        return false;
    }
    let comparable_ip = normalize_ipv4_mapped(normalized_ip);

    if !candidate.contains('/') {
        // Exact match
        let Some(exact) = parse_canonical_ip_address(Some(candidate)) else {
            return false;
        };
        let comparable_exact = normalize_ipv4_mapped(exact);
        return std::mem::discriminant(&comparable_ip) == std::mem::discriminant(&comparable_exact)
            && comparable_ip.to_string() == comparable_exact.to_string();
    }

    // CIDR match
    let parts: Vec<&str> = candidate.splitn(2, '/').collect();
    if parts.len() != 2 {
        return false;
    }
    let Some(base_addr) = parse_canonical_ip_address(Some(parts[0])) else {
        return false;
    };
    let Ok(prefix_len) = parts[1].parse::<u32>() else {
        return false;
    };

    let comparable_base = normalize_ipv4_mapped(base_addr);

    match (comparable_ip, comparable_base) {
        (IpAddr::V4(ip4), IpAddr::V4(base4)) => ip_in_cidr_v4(ip4, base4, prefix_len),
        (IpAddr::V6(ip6), IpAddr::V6(base6)) => ip_in_cidr_v6(ip6, base6, prefix_len),
        _ => false,
    }
}

fn ip_in_cidr_v4(ip: Ipv4Addr, base: Ipv4Addr, prefix_len: u32) -> bool {
    if prefix_len > 32 {
        return false;
    }
    if prefix_len == 0 {
        return true;
    }
    let mask = !0u32 << (32 - prefix_len);
    let ip_bits = u32::from(ip);
    let base_bits = u32::from(base);
    (ip_bits & mask) == (base_bits & mask)
}

fn ip_in_cidr_v6(ip: Ipv6Addr, base: Ipv6Addr, prefix_len: u32) -> bool {
    if prefix_len > 128 {
        return false;
    }
    if prefix_len == 0 {
        return true;
    }
    let mask = !0u128 << (128 - prefix_len);
    let ip_bits = u128::from(ip);
    let base_bits = u128::from(base);
    (ip_bits & mask) == (base_bits & mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Parsing ----

    #[test]
    fn parse_canonical_ipv4() {
        let addr = parse_canonical_ip_address(Some("192.168.1.1")).unwrap();
        assert_eq!(addr, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn parse_canonical_rejects_non_decimal() {
        assert!(parse_canonical_ip_address(Some("0x7f.0.0.1")).is_none());
    }

    #[test]
    fn parse_canonical_ipv6() {
        let addr = parse_canonical_ip_address(Some("::1")).unwrap();
        assert_eq!(addr, IpAddr::V6(Ipv6Addr::LOCALHOST));
    }

    #[test]
    fn parse_strips_brackets() {
        let addr = parse_canonical_ip_address(Some("[::1]")).unwrap();
        assert_eq!(addr, IpAddr::V6(Ipv6Addr::LOCALHOST));
    }

    #[test]
    fn parse_none_input() {
        assert!(parse_canonical_ip_address(None).is_none());
    }

    #[test]
    fn parse_empty_input() {
        assert!(parse_canonical_ip_address(Some("")).is_none());
        assert!(parse_canonical_ip_address(Some("  ")).is_none());
    }

    // ---- Normalization ----

    #[test]
    fn normalize_ipv4() {
        assert_eq!(normalize_ip_address(Some("  192.168.1.1  ")), Some("192.168.1.1".to_string()));
    }

    #[test]
    fn normalize_ipv4_mapped_v6() {
        assert_eq!(normalize_ip_address(Some("::ffff:192.168.1.1")), Some("192.168.1.1".to_string()));
    }

    // ---- Canonical dotted decimal ----

    #[test]
    fn canonical_dotted_decimal() {
        assert!(is_canonical_dotted_decimal_ipv4(Some("192.168.1.1")));
        assert!(is_canonical_dotted_decimal_ipv4(Some("0.0.0.0")));
        assert!(!is_canonical_dotted_decimal_ipv4(Some("192.168.1")));
        assert!(!is_canonical_dotted_decimal_ipv4(Some("192.168.01.1")));
        assert!(!is_canonical_dotted_decimal_ipv4(Some("")));
        assert!(!is_canonical_dotted_decimal_ipv4(None));
    }

    // ---- Legacy IPv4 literal ----

    #[test]
    fn legacy_ipv4() {
        assert!(is_legacy_ipv4_literal(Some("0x7f000001")));
        assert!(is_legacy_ipv4_literal(Some("192.168.1")));
        assert!(!is_legacy_ipv4_literal(Some("192.168.1.1")));
        assert!(!is_legacy_ipv4_literal(Some("::1")));
        assert!(!is_legacy_ipv4_literal(None));
    }

    // ---- Range checks ----

    #[test]
    fn loopback_check() {
        assert!(is_loopback_ip_address(Some("127.0.0.1")));
        assert!(is_loopback_ip_address(Some("::1")));
        assert!(!is_loopback_ip_address(Some("192.168.1.1")));
    }

    #[test]
    fn private_or_loopback() {
        assert!(is_private_or_loopback_ip_address(Some("127.0.0.1")));
        assert!(is_private_or_loopback_ip_address(Some("10.0.0.1")));
        assert!(is_private_or_loopback_ip_address(Some("192.168.1.1")));
        assert!(is_private_or_loopback_ip_address(Some("172.16.0.1")));
        assert!(is_private_or_loopback_ip_address(Some("100.64.0.1")));
        assert!(!is_private_or_loopback_ip_address(Some("8.8.8.8")));
    }

    #[test]
    fn rfc1918_check() {
        assert!(is_rfc1918_ipv4_address(Some("10.0.0.1")));
        assert!(is_rfc1918_ipv4_address(Some("172.16.0.1")));
        assert!(is_rfc1918_ipv4_address(Some("192.168.1.1")));
        assert!(!is_rfc1918_ipv4_address(Some("100.64.0.1")));
        assert!(!is_rfc1918_ipv4_address(Some("8.8.8.8")));
    }

    #[test]
    fn carrier_grade_nat() {
        assert!(is_carrier_grade_nat_ipv4_address(Some("100.64.0.1")));
        assert!(is_carrier_grade_nat_ipv4_address(Some("100.127.255.255")));
        assert!(!is_carrier_grade_nat_ipv4_address(Some("100.128.0.0")));
    }

    // ---- Blocked special use ----

    #[test]
    fn blocked_ipv4_special_use() {
        let opts = Ipv4SpecialUseBlockOptions::default();
        assert!(is_blocked_special_use_ipv4_address(Ipv4Addr::LOCALHOST, &opts));
        assert!(is_blocked_special_use_ipv4_address(Ipv4Addr::new(10, 0, 0, 1), &opts));
        assert!(!is_blocked_special_use_ipv4_address(Ipv4Addr::new(8, 8, 8, 8), &opts));
    }

    #[test]
    fn rfc2544_benchmark_range() {
        let addr = Ipv4Addr::new(198, 18, 0, 1);
        let block_opts = Ipv4SpecialUseBlockOptions::default();
        assert!(is_blocked_special_use_ipv4_address(addr, &block_opts));

        let allow_opts = Ipv4SpecialUseBlockOptions {
            allow_rfc2544_benchmark_range: true,
        };
        assert!(!is_blocked_special_use_ipv4_address(addr, &allow_opts));
    }

    #[test]
    fn blocked_ipv6() {
        assert!(is_blocked_special_use_ipv6_address(Ipv6Addr::LOCALHOST));
        assert!(is_blocked_special_use_ipv6_address(Ipv6Addr::UNSPECIFIED));
        // Site-local fec0::1
        assert!(is_blocked_special_use_ipv6_address(
            "fec0::1".parse().unwrap()
        ));
    }

    // ---- Embedded IPv4 extraction ----

    #[test]
    fn extract_ipv4_mapped() {
        let v6: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        let v4 = extract_embedded_ipv4_from_ipv6(v6).unwrap();
        assert_eq!(v4, Ipv4Addr::new(192, 168, 1, 1));
    }

    #[test]
    fn extract_6to4() {
        let v6: Ipv6Addr = "2002:c0a8:0101::".parse().unwrap();
        let v4 = extract_embedded_ipv4_from_ipv6(v6).unwrap();
        assert_eq!(v4, Ipv4Addr::new(192, 168, 1, 1));
    }

    #[test]
    fn extract_teredo() {
        // Teredo: 2001:0000:... with XOR'd client IPv4 in hextets 6..7
        // Client 192.168.1.1 = 0xc0a80101, XOR'd = 0x3f57fefe
        let v6: Ipv6Addr = "2001:0000:abcd:efgh:0000:0000:3f57:fefe"
            .parse()
            .unwrap_or_else(|_| "2001:0000:abcd:0000:0000:0000:3f57:fefe".parse().unwrap());
        let v4 = extract_embedded_ipv4_from_ipv6(v6).unwrap();
        assert_eq!(v4, Ipv4Addr::new(192, 168, 1, 1));
    }

    // ---- CIDR matching ----

    #[test]
    fn cidr_match_ipv4() {
        assert!(is_ip_in_cidr("192.168.1.100", "192.168.1.0/24"));
        assert!(!is_ip_in_cidr("192.168.2.1", "192.168.1.0/24"));
    }

    #[test]
    fn cidr_match_exact() {
        assert!(is_ip_in_cidr("10.0.0.1", "10.0.0.1"));
        assert!(!is_ip_in_cidr("10.0.0.2", "10.0.0.1"));
    }

    #[test]
    fn cidr_match_ipv6() {
        assert!(is_ip_in_cidr("2001:db8::1", "2001:db8::/32"));
        assert!(!is_ip_in_cidr("2001:db9::1", "2001:db8::/32"));
    }

    #[test]
    fn cidr_invalid_inputs() {
        assert!(!is_ip_in_cidr("not-an-ip", "192.168.1.0/24"));
        assert!(!is_ip_in_cidr("192.168.1.1", ""));
        assert!(!is_ip_in_cidr("192.168.1.1", "not-cidr"));
    }
}
