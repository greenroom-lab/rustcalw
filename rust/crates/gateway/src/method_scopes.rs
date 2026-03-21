use std::sync::LazyLock;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Operator scopes
// ---------------------------------------------------------------------------

pub const ADMIN_SCOPE: &str = "operator.admin";
pub const READ_SCOPE: &str = "operator.read";
pub const WRITE_SCOPE: &str = "operator.write";
pub const APPROVALS_SCOPE: &str = "operator.approvals";
pub const PAIRING_SCOPE: &str = "operator.pairing";

pub type OperatorScope = &'static str;

pub const CLI_DEFAULT_OPERATOR_SCOPES: &[&str] = &[
    ADMIN_SCOPE,
    READ_SCOPE,
    WRITE_SCOPE,
    APPROVALS_SCOPE,
    PAIRING_SCOPE,
];

// ---------------------------------------------------------------------------
// Node-role methods
// ---------------------------------------------------------------------------

const NODE_ROLE_METHODS: &[&str] = &[
    "node.invoke.result",
    "node.event",
    "node.pending.drain",
    "node.canvas.capability.refresh",
    "node.pending.pull",
    "node.pending.ack",
    "skills.bins",
];

pub fn is_node_role_method(method: &str) -> bool {
    NODE_ROLE_METHODS.contains(&method)
}

// ---------------------------------------------------------------------------
// Method → scope mapping
// ---------------------------------------------------------------------------

const ADMIN_METHOD_PREFIXES: &[&str] = &["exec.approvals.", "config.", "wizard.", "update."];

struct MethodScopeGroups {
    by_name: HashMap<&'static str, &'static str>,
}

impl MethodScopeGroups {
    fn new() -> Self {
        let mut by_name = HashMap::new();

        let groups: &[(&str, &[&str])] = &[
            (
                APPROVALS_SCOPE,
                &[
                    "exec.approval.request",
                    "exec.approval.waitDecision",
                    "exec.approval.resolve",
                ],
            ),
            (
                PAIRING_SCOPE,
                &[
                    "node.pair.request",
                    "node.pair.list",
                    "node.pair.approve",
                    "node.pair.reject",
                    "node.pair.verify",
                    "device.pair.list",
                    "device.pair.approve",
                    "device.pair.reject",
                    "device.pair.remove",
                    "device.token.rotate",
                    "device.token.revoke",
                    "node.rename",
                ],
            ),
            (
                READ_SCOPE,
                &[
                    "health",
                    "doctor.memory.status",
                    "logs.tail",
                    "channels.status",
                    "status",
                    "usage.status",
                    "usage.cost",
                    "tts.status",
                    "tts.providers",
                    "models.list",
                    "tools.catalog",
                    "agents.list",
                    "agent.identity.get",
                    "skills.status",
                    "voicewake.get",
                    "sessions.list",
                    "sessions.get",
                    "sessions.preview",
                    "sessions.resolve",
                    "sessions.subscribe",
                    "sessions.unsubscribe",
                    "sessions.messages.subscribe",
                    "sessions.messages.unsubscribe",
                    "sessions.usage",
                    "sessions.usage.timeseries",
                    "sessions.usage.logs",
                    "cron.list",
                    "cron.status",
                    "cron.runs",
                    "gateway.identity.get",
                    "system-presence",
                    "last-heartbeat",
                    "node.list",
                    "node.describe",
                    "chat.history",
                    "config.get",
                    "config.schema.lookup",
                    "talk.config",
                    "agents.files.list",
                    "agents.files.get",
                ],
            ),
            (
                WRITE_SCOPE,
                &[
                    "send",
                    "poll",
                    "agent",
                    "agent.wait",
                    "wake",
                    "talk.mode",
                    "talk.speak",
                    "tts.enable",
                    "tts.disable",
                    "tts.convert",
                    "tts.setProvider",
                    "voicewake.set",
                    "node.invoke",
                    "chat.send",
                    "chat.abort",
                    "sessions.create",
                    "sessions.send",
                    "sessions.abort",
                    "browser.request",
                    "push.test",
                    "node.pending.enqueue",
                ],
            ),
            (
                ADMIN_SCOPE,
                &[
                    "channels.logout",
                    "agents.create",
                    "agents.update",
                    "agents.delete",
                    "skills.install",
                    "skills.update",
                    "secrets.reload",
                    "secrets.resolve",
                    "cron.add",
                    "cron.update",
                    "cron.remove",
                    "cron.run",
                    "sessions.patch",
                    "sessions.reset",
                    "sessions.delete",
                    "sessions.compact",
                    "connect",
                    "chat.inject",
                    "web.login.start",
                    "web.login.wait",
                    "set-heartbeats",
                    "system-event",
                    "agents.files.set",
                ],
            ),
        ];

        for &(scope, methods) in groups {
            for &method in methods {
                by_name.insert(method, scope);
            }
        }

        Self { by_name }
    }
}

static SCOPE_GROUPS: LazyLock<MethodScopeGroups> = LazyLock::new(MethodScopeGroups::new);

fn resolve_scoped_method(method: &str) -> Option<&'static str> {
    if let Some(&scope) = SCOPE_GROUPS.by_name.get(method) {
        return Some(scope);
    }
    if ADMIN_METHOD_PREFIXES.iter().any(|p| method.starts_with(p)) {
        return Some(ADMIN_SCOPE);
    }
    None
}

// ---------------------------------------------------------------------------
// Public query functions
// ---------------------------------------------------------------------------

pub fn is_approval_method(method: &str) -> bool {
    resolve_scoped_method(method) == Some(APPROVALS_SCOPE)
}

pub fn is_pairing_method(method: &str) -> bool {
    resolve_scoped_method(method) == Some(PAIRING_SCOPE)
}

pub fn is_read_method(method: &str) -> bool {
    resolve_scoped_method(method) == Some(READ_SCOPE)
}

pub fn is_write_method(method: &str) -> bool {
    resolve_scoped_method(method) == Some(WRITE_SCOPE)
}

pub fn is_admin_only_method(method: &str) -> bool {
    resolve_scoped_method(method) == Some(ADMIN_SCOPE)
}

pub fn resolve_required_operator_scope_for_method(method: &str) -> Option<&'static str> {
    resolve_scoped_method(method)
}

pub fn resolve_least_privilege_operator_scopes_for_method(method: &str) -> Vec<&'static str> {
    match resolve_required_operator_scope_for_method(method) {
        Some(scope) => vec![scope],
        None => vec![],
    }
}

/// Result of scope authorization.
#[derive(Debug, Clone, PartialEq)]
pub enum AuthorizeScopesResult {
    Allowed,
    Denied { missing_scope: &'static str },
}

pub fn authorize_operator_scopes_for_method(
    method: &str,
    scopes: &[&str],
) -> AuthorizeScopesResult {
    // Admin can do anything.
    if scopes.contains(&ADMIN_SCOPE) {
        return AuthorizeScopesResult::Allowed;
    }

    let required = resolve_required_operator_scope_for_method(method)
        .unwrap_or(ADMIN_SCOPE);

    // READ is also satisfied by WRITE.
    if required == READ_SCOPE {
        if scopes.contains(&READ_SCOPE) || scopes.contains(&WRITE_SCOPE) {
            return AuthorizeScopesResult::Allowed;
        }
        return AuthorizeScopesResult::Denied { missing_scope: READ_SCOPE };
    }

    if scopes.contains(&required) {
        return AuthorizeScopesResult::Allowed;
    }
    AuthorizeScopesResult::Denied { missing_scope: required }
}

pub fn is_gateway_method_classified(method: &str) -> bool {
    if is_node_role_method(method) {
        return true;
    }
    resolve_required_operator_scope_for_method(method).is_some()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_role_methods() {
        assert!(is_node_role_method("node.invoke.result"));
        assert!(is_node_role_method("skills.bins"));
        assert!(!is_node_role_method("send"));
    }

    #[test]
    fn scope_classification() {
        assert!(is_approval_method("exec.approval.request"));
        assert!(is_pairing_method("node.pair.approve"));
        assert!(is_read_method("health"));
        assert!(is_read_method("sessions.list"));
        assert!(is_write_method("send"));
        assert!(is_write_method("chat.send"));
        assert!(is_admin_only_method("agents.create"));
        assert!(is_admin_only_method("cron.add"));
    }

    #[test]
    fn admin_prefix_methods() {
        assert!(is_admin_only_method("config.set"));
        assert!(is_admin_only_method("wizard.start"));
        assert!(is_admin_only_method("update.run"));
        assert!(is_admin_only_method("exec.approvals.get"));
    }

    #[test]
    fn unclassified_method() {
        assert!(!is_gateway_method_classified("unknown.method"));
        assert_eq!(
            resolve_least_privilege_operator_scopes_for_method("unknown.method"),
            Vec::<&str>::new(),
        );
    }

    #[test]
    fn authorize_admin_scope_grants_all() {
        let scopes = &[ADMIN_SCOPE];
        assert_eq!(
            authorize_operator_scopes_for_method("send", scopes),
            AuthorizeScopesResult::Allowed,
        );
        assert_eq!(
            authorize_operator_scopes_for_method("agents.create", scopes),
            AuthorizeScopesResult::Allowed,
        );
    }

    #[test]
    fn authorize_read_also_satisfied_by_write() {
        let scopes = &[WRITE_SCOPE];
        assert_eq!(
            authorize_operator_scopes_for_method("health", scopes),
            AuthorizeScopesResult::Allowed,
        );
    }

    #[test]
    fn authorize_denied_returns_missing_scope() {
        let scopes = &[READ_SCOPE];
        assert_eq!(
            authorize_operator_scopes_for_method("send", scopes),
            AuthorizeScopesResult::Denied { missing_scope: WRITE_SCOPE },
        );
    }

    #[test]
    fn authorize_unclassified_requires_admin() {
        let scopes = &[WRITE_SCOPE];
        assert_eq!(
            authorize_operator_scopes_for_method("unknown.method", scopes),
            AuthorizeScopesResult::Denied { missing_scope: ADMIN_SCOPE },
        );
    }

    #[test]
    fn classified_includes_node_and_operator() {
        assert!(is_gateway_method_classified("node.invoke.result"));
        assert!(is_gateway_method_classified("health"));
        assert!(is_gateway_method_classified("send"));
    }
}
