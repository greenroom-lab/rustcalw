use crate::method_scopes::is_node_role_method;

// ---------------------------------------------------------------------------
// Gateway roles
// ---------------------------------------------------------------------------

pub const GATEWAY_ROLES: &[&str] = &["operator", "node"];

pub type GatewayRole = &'static str;

pub const ROLE_OPERATOR: &str = "operator";
pub const ROLE_NODE: &str = "node";

/// Parse a raw value into a valid `GatewayRole`, returning `None` for
/// unrecognised values.
pub fn parse_gateway_role(role_raw: Option<&str>) -> Option<&'static str> {
    match role_raw? {
        "operator" => Some(ROLE_OPERATOR),
        "node" => Some(ROLE_NODE),
        _ => None,
    }
}

/// Operators with shared auth can skip device identity verification.
pub fn role_can_skip_device_identity(role: &str, shared_auth_ok: bool) -> bool {
    role == ROLE_OPERATOR && shared_auth_ok
}

/// Check whether the given role is authorised to call `method`.
///
/// Node-role methods require the `node` role; all other methods require
/// the `operator` role.
pub fn is_role_authorized_for_method(role: &str, method: &str) -> bool {
    if is_node_role_method(method) {
        return role == ROLE_NODE;
    }
    role == ROLE_OPERATOR
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_roles() {
        assert_eq!(parse_gateway_role(Some("operator")), Some(ROLE_OPERATOR));
        assert_eq!(parse_gateway_role(Some("node")), Some(ROLE_NODE));
    }

    #[test]
    fn parse_invalid_roles() {
        assert_eq!(parse_gateway_role(Some("admin")), None);
        assert_eq!(parse_gateway_role(None), None);
    }

    #[test]
    fn skip_device_identity() {
        assert!(role_can_skip_device_identity(ROLE_OPERATOR, true));
        assert!(!role_can_skip_device_identity(ROLE_OPERATOR, false));
        assert!(!role_can_skip_device_identity(ROLE_NODE, true));
    }

    #[test]
    fn role_authorization() {
        assert!(is_role_authorized_for_method(ROLE_NODE, "node.invoke.result"));
        assert!(!is_role_authorized_for_method(ROLE_OPERATOR, "node.invoke.result"));
        assert!(is_role_authorized_for_method(ROLE_OPERATOR, "send"));
        assert!(!is_role_authorized_for_method(ROLE_NODE, "send"));
    }
}
