use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

// External JavaScript function bindings
#[wasm_bindgen]
extern "C" {
    // Initialize Firebase auth with callback
    #[wasm_bindgen(js_name = "initializeAuth")]
    fn initialize_auth_js(callback: &js_sys::Function) -> Option<String>;

    // Sign in with Google - returns Promise
    #[wasm_bindgen(js_name = "signInWithGoogle")]
    fn sign_in_with_google_js() -> js_sys::Promise;

    // Sign out user - returns Promise
    #[wasm_bindgen(js_name = "signOutUser")]
    fn sign_out_user_js() -> js_sys::Promise;

    // Get current user ID token - returns Promise
    #[wasm_bindgen(js_name = "getCurrentUserToken")]
    fn get_current_user_token_js() -> js_sys::Promise;

    // Get current user data
    #[wasm_bindgen(js_name = "getCurrentUser")]
    fn get_current_user_js() -> Option<String>;

    // Fetch server data - returns Promise
    #[wasm_bindgen(js_name = "fetchServerData")]
    fn fetch_server_data_js(id_token: &str) -> js_sys::Promise;
}

// User data structure
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserData {
    pub uid: String,
    pub email: String,
    pub display_name: String,
    pub photo_url: String,
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
}

// Server data structure
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServerData {
    #[serde(rename = "serverTime")]
    pub server_time: u64,
    #[serde(rename = "subscriptionToken")]
    pub subscription_token: String,
    #[serde(rename = "subscriptionExpires")]
    pub subscription_expires: u64,
    #[serde(rename = "offlineWindowHours")]
    pub offline_window_hours: u32,
}

// Auth error types
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("JavaScript error: {0}")]
    JsError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("No user data available")]
    NoUserData,
    #[error("Authentication failed")]
    AuthFailed,
}

// Initialize Firebase authentication with callback
pub fn initialize_auth<F>(callback: F) -> Result<Option<UserData>, AuthError>
where
    F: Fn(Option<UserData>) + 'static,
{
    // Create JavaScript callback closure
    let closure = Closure::wrap(Box::new(move |user_json: Option<String>| {
        let user_data = match user_json {
            Some(json) => match serde_json::from_str::<UserData>(&json) {
                Ok(data) => Some(data),
                Err(e) => {
                    log::error!("Failed to parse user data: {e}");
                    None
                }
            },
            None => None,
        };
        callback(user_data);
    }) as Box<dyn Fn(Option<String>)>);

    let result = initialize_auth_js(closure.as_ref().unchecked_ref());

    // Keep closure alive
    closure.forget();

    // Parse initial user data if available
    match result {
        Some(json) => {
            let user_data = serde_json::from_str::<UserData>(&json)
                .map_err(|e| AuthError::SerializationError(e.to_string()))?;
            Ok(Some(user_data))
        }
        None => Ok(None),
    }
}

// Sign in with Google
pub async fn sign_in_with_google() -> Result<UserData, AuthError> {
    let promise = sign_in_with_google_js();
    let future = JsFuture::from(promise);

    match future.await {
        Ok(value) => {
            let json = value
                .as_string()
                .ok_or_else(|| AuthError::JsError("Invalid response format".to_string()))?;

            let user_data = serde_json::from_str::<UserData>(&json)
                .map_err(|e| AuthError::SerializationError(e.to_string()))?;

            Ok(user_data)
        }
        Err(e) => {
            let error_msg = e
                .as_string()
                .unwrap_or_else(|| "Unknown sign-in error".to_string());
            Err(AuthError::JsError(error_msg))
        }
    }
}

// Sign out user
pub async fn sign_out_user() -> Result<(), AuthError> {
    let promise = sign_out_user_js();
    let future = JsFuture::from(promise);

    match future.await {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = e
                .as_string()
                .unwrap_or_else(|| "Unknown sign-out error".to_string());
            Err(AuthError::JsError(error_msg))
        }
    }
}

// Get current user ID token
pub async fn get_current_user_token() -> Result<Option<String>, AuthError> {
    let promise = get_current_user_token_js();
    let future = JsFuture::from(promise);

    match future.await {
        Ok(value) => {
            if value.is_null() || value.is_undefined() {
                Ok(None)
            } else {
                let token = value
                    .as_string()
                    .ok_or_else(|| AuthError::JsError("Invalid token format".to_string()))?;
                Ok(Some(token))
            }
        }
        Err(e) => {
            let error_msg = e
                .as_string()
                .unwrap_or_else(|| "Failed to get token".to_string());
            Err(AuthError::JsError(error_msg))
        }
    }
}

// Get current user data
pub fn get_current_user() -> Result<Option<UserData>, AuthError> {
    match get_current_user_js() {
        Some(json) => {
            let user_data = serde_json::from_str::<UserData>(&json)
                .map_err(|e| AuthError::SerializationError(e.to_string()))?;
            Ok(Some(user_data))
        }
        None => Ok(None),
    }
}

// Fetch server data (time and subscription info)
pub async fn fetch_server_data(id_token: &str) -> Result<ServerData, AuthError> {
    let promise = fetch_server_data_js(id_token);
    let future = JsFuture::from(promise);

    match future.await {
        Ok(value) => {
            let json = value
                .as_string()
                .ok_or_else(|| AuthError::JsError("Invalid server response format".to_string()))?;

            let server_data = serde_json::from_str::<ServerData>(&json)
                .map_err(|e| AuthError::SerializationError(e.to_string()))?;

            Ok(server_data)
        }
        Err(e) => {
            let error_msg = e
                .as_string()
                .unwrap_or_else(|| "Server request failed".to_string());
            Err(AuthError::JsError(error_msg))
        }
    }
}
