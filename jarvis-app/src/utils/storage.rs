use wasm_bindgen::prelude::*;
use web_sys::window;

/// Local storage wrapper
pub struct LocalStorage;

impl LocalStorage {
    /// Get an item from local storage
    pub fn get(key: &str) -> Option<String> {
        let window = window()?;
        let storage = window.local_storage().ok()??;
        storage.get_item(key).ok()?
    }

    /// Set an item in local storage
    pub fn set(key: &str, value: &str) -> Result<(), JsValue> {
        let window = window().ok_or("No window")?;
        let storage = window
            .local_storage()?
            .ok_or("No local storage available")?;
        storage.set_item(key, value)
    }

    /// Remove an item from local storage
    pub fn remove(key: &str) -> Result<(), JsValue> {
        let window = window().ok_or("No window")?;
        let storage = window
            .local_storage()?
            .ok_or("No local storage available")?;
        storage.remove_item(key)
    }

    /// Clear all items from local storage
    pub fn clear() -> Result<(), JsValue> {
        let window = window().ok_or("No window")?;
        let storage = window
            .local_storage()?
            .ok_or("No local storage available")?;
        storage.clear()
    }
}
