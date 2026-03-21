//! IPv4 validation helpers — mirrors src/shared/net/ipv4.ts

use crate::net_ip::is_canonical_dotted_decimal_ipv4;

/// Validate a dotted-decimal IPv4 input for custom bind mode.
/// Returns `None` if valid, or an error message if invalid.
pub fn validate_dotted_decimal_ipv4_input(value: Option<&str>) -> Option<&'static str> {
    let Some(v) = value else {
        return Some("IP address is required for custom bind mode");
    };
    if v.is_empty() {
        return Some("IP address is required for custom bind mode");
    }
    if is_canonical_dotted_decimal_ipv4(Some(v)) {
        return None;
    }
    Some("Invalid IPv4 address (e.g., 192.168.1.100)")
}

/// Backward-compatible alias for callers using the old helper name.
pub fn validate_ipv4_address_input(value: Option<&str>) -> Option<&'static str> {
    validate_dotted_decimal_ipv4_input(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ipv4() {
        assert!(validate_dotted_decimal_ipv4_input(Some("192.168.1.100")).is_none());
        assert!(validate_dotted_decimal_ipv4_input(Some("0.0.0.0")).is_none());
        assert!(validate_dotted_decimal_ipv4_input(Some("255.255.255.255")).is_none());
    }

    #[test]
    fn invalid_ipv4() {
        assert!(validate_dotted_decimal_ipv4_input(Some("not-an-ip")).is_some());
        assert!(validate_dotted_decimal_ipv4_input(Some("192.168.1")).is_some());
        assert!(validate_dotted_decimal_ipv4_input(Some("256.1.1.1")).is_some());
    }

    #[test]
    fn none_and_empty() {
        assert!(validate_dotted_decimal_ipv4_input(None).is_some());
        assert!(validate_dotted_decimal_ipv4_input(Some("")).is_some());
    }

    #[test]
    fn alias_works() {
        assert!(validate_ipv4_address_input(Some("10.0.0.1")).is_none());
        assert!(validate_ipv4_address_input(None).is_some());
    }
}
