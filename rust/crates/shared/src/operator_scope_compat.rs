//! Operator scope compatibility — mirrors src/shared/operator-scope-compat.ts

use std::collections::HashSet;

const OPERATOR_ROLE: &str = "operator";
const OPERATOR_ADMIN_SCOPE: &str = "operator.admin";
const OPERATOR_READ_SCOPE: &str = "operator.read";
const OPERATOR_WRITE_SCOPE: &str = "operator.write";
const OPERATOR_SCOPE_PREFIX: &str = "operator.";

fn normalize_scope_list(scopes: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for scope in scopes {
        let trimmed = scope.trim();
        if !trimmed.is_empty() && seen.insert(trimmed.to_string()) {
            out.push(trimmed.to_string());
        }
    }
    out
}

fn operator_scope_satisfied(requested_scope: &str, granted: &HashSet<String>) -> bool {
    if granted.contains(OPERATOR_ADMIN_SCOPE)
        && requested_scope.starts_with(OPERATOR_SCOPE_PREFIX)
    {
        return true;
    }
    if requested_scope == OPERATOR_READ_SCOPE {
        return granted.contains(OPERATOR_READ_SCOPE) || granted.contains(OPERATOR_WRITE_SCOPE);
    }
    if requested_scope == OPERATOR_WRITE_SCOPE {
        return granted.contains(OPERATOR_WRITE_SCOPE);
    }
    granted.contains(requested_scope)
}

/// Check whether the given role + allowed scopes satisfy all requested scopes.
pub fn role_scopes_allow(
    role: &str,
    requested_scopes: &[&str],
    allowed_scopes: &[&str],
) -> bool {
    let requested = normalize_scope_list(requested_scopes);
    if requested.is_empty() {
        return true;
    }
    let allowed = normalize_scope_list(allowed_scopes);
    if allowed.is_empty() {
        return false;
    }
    let allowed_set: HashSet<String> = allowed.into_iter().collect();

    if role.trim() != OPERATOR_ROLE {
        return requested.iter().all(|s| allowed_set.contains(s));
    }
    requested
        .iter()
        .all(|s| operator_scope_satisfied(s, &allowed_set))
}

/// Return the first requested scope that is not satisfied, or `None` if all are allowed.
pub fn resolve_missing_requested_scope<'a>(
    role: &str,
    requested_scopes: &[&'a str],
    allowed_scopes: &[&str],
) -> Option<&'a str> {
    for &scope in requested_scopes {
        if !role_scopes_allow(role, &[scope], allowed_scopes) {
            return Some(scope);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_requested_always_passes() {
        assert!(role_scopes_allow("user", &[], &["a"]));
    }

    #[test]
    fn empty_allowed_always_fails() {
        assert!(!role_scopes_allow("user", &["a"], &[]));
    }

    #[test]
    fn non_operator_exact_match() {
        assert!(role_scopes_allow("user", &["read"], &["read", "write"]));
        assert!(!role_scopes_allow("user", &["admin"], &["read"]));
    }

    #[test]
    fn operator_admin_grants_all_operator_scopes() {
        assert!(role_scopes_allow(
            "operator",
            &["operator.read", "operator.write", "operator.custom"],
            &["operator.admin"]
        ));
    }

    #[test]
    fn operator_write_implies_read() {
        assert!(role_scopes_allow(
            "operator",
            &["operator.read"],
            &["operator.write"]
        ));
    }

    #[test]
    fn operator_read_does_not_imply_write() {
        assert!(!role_scopes_allow(
            "operator",
            &["operator.write"],
            &["operator.read"]
        ));
    }

    #[test]
    fn resolve_missing_returns_first_unmet() {
        let missing =
            resolve_missing_requested_scope("user", &["a", "b", "c"], &["a", "c"]);
        assert_eq!(missing, Some("b"));
    }

    #[test]
    fn resolve_missing_returns_none_when_all_met() {
        let missing =
            resolve_missing_requested_scope("user", &["a", "b"], &["a", "b"]);
        assert!(missing.is_none());
    }
}
