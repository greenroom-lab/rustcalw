//! Device auth store operations — mirrors src/shared/device-auth-store.ts

use crate::device_auth::{
    normalize_device_auth_role, normalize_device_auth_scopes, DeviceAuthEntry, DeviceAuthStore,
};
use std::collections::HashMap;

/// Adapter trait for reading/writing the device auth store.
pub trait DeviceAuthStoreAdapter {
    fn read_store(&self) -> Option<DeviceAuthStore>;
    fn write_store(&self, store: &DeviceAuthStore);
}

/// Load a device auth token from the store for the given device and role.
pub fn load_device_auth_token_from_store(
    adapter: &dyn DeviceAuthStoreAdapter,
    device_id: &str,
    role: &str,
) -> Option<DeviceAuthEntry> {
    let store = adapter.read_store()?;
    if store.device_id != device_id {
        return None;
    }
    let role = normalize_device_auth_role(role);
    let entry = store.tokens.get(&role)?;
    if entry.token.is_empty() {
        return None;
    }
    Some(entry.clone())
}

/// Store a device auth token in the store.
pub fn store_device_auth_token_in_store(
    adapter: &dyn DeviceAuthStoreAdapter,
    device_id: &str,
    role: &str,
    token: &str,
    scopes: Option<&[String]>,
) -> DeviceAuthEntry {
    let role = normalize_device_auth_role(role);
    let existing = adapter.read_store();

    let mut tokens = match &existing {
        Some(store) if store.device_id == device_id => store.tokens.clone(),
        _ => HashMap::new(),
    };

    let entry = DeviceAuthEntry {
        token: token.to_string(),
        role: role.clone(),
        scopes: normalize_device_auth_scopes(scopes),
        updated_at_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    };

    tokens.insert(role, entry.clone());

    let next = DeviceAuthStore {
        version: 1,
        device_id: device_id.to_string(),
        tokens,
    };
    adapter.write_store(&next);
    entry
}

/// Clear a device auth token from the store.
pub fn clear_device_auth_token_from_store(
    adapter: &dyn DeviceAuthStoreAdapter,
    device_id: &str,
    role: &str,
) {
    let Some(store) = adapter.read_store() else {
        return;
    };
    if store.device_id != device_id {
        return;
    }
    let role = normalize_device_auth_role(role);
    if !store.tokens.contains_key(&role) {
        return;
    }
    let mut tokens = store.tokens.clone();
    tokens.remove(&role);

    let next = DeviceAuthStore {
        version: 1,
        device_id: store.device_id.clone(),
        tokens,
    };
    adapter.write_store(&next);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockAdapter {
        store: RefCell<Option<DeviceAuthStore>>,
    }

    impl MockAdapter {
        fn new() -> Self {
            Self {
                store: RefCell::new(None),
            }
        }
    }

    impl DeviceAuthStoreAdapter for MockAdapter {
        fn read_store(&self) -> Option<DeviceAuthStore> {
            self.store.borrow().clone()
        }
        fn write_store(&self, store: &DeviceAuthStore) {
            *self.store.borrow_mut() = Some(store.clone());
        }
    }

    #[test]
    fn store_and_load_token() {
        let adapter = MockAdapter::new();
        let entry =
            store_device_auth_token_in_store(&adapter, "dev-1", "admin", "tok-abc", None);
        assert_eq!(entry.token, "tok-abc");
        assert_eq!(entry.role, "admin");

        let loaded = load_device_auth_token_from_store(&adapter, "dev-1", "admin");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().token, "tok-abc");
    }

    #[test]
    fn load_wrong_device_returns_none() {
        let adapter = MockAdapter::new();
        store_device_auth_token_in_store(&adapter, "dev-1", "admin", "tok-abc", None);

        let loaded = load_device_auth_token_from_store(&adapter, "dev-other", "admin");
        assert!(loaded.is_none());
    }

    #[test]
    fn clear_token() {
        let adapter = MockAdapter::new();
        store_device_auth_token_in_store(&adapter, "dev-1", "admin", "tok-abc", None);
        clear_device_auth_token_from_store(&adapter, "dev-1", "admin");

        let loaded = load_device_auth_token_from_store(&adapter, "dev-1", "admin");
        assert!(loaded.is_none());
    }

    #[test]
    fn clear_nonexistent_is_noop() {
        let adapter = MockAdapter::new();
        clear_device_auth_token_from_store(&adapter, "dev-1", "admin");
        assert!(adapter.read_store().is_none());
    }

    #[test]
    fn multiple_roles() {
        let adapter = MockAdapter::new();
        store_device_auth_token_in_store(&adapter, "dev-1", "admin", "tok-a", None);
        store_device_auth_token_in_store(&adapter, "dev-1", "user", "tok-b", None);

        let a = load_device_auth_token_from_store(&adapter, "dev-1", "admin").unwrap();
        let b = load_device_auth_token_from_store(&adapter, "dev-1", "user").unwrap();
        assert_eq!(a.token, "tok-a");
        assert_eq!(b.token, "tok-b");

        clear_device_auth_token_from_store(&adapter, "dev-1", "admin");
        assert!(load_device_auth_token_from_store(&adapter, "dev-1", "admin").is_none());
        assert!(load_device_auth_token_from_store(&adapter, "dev-1", "user").is_some());
    }
}
