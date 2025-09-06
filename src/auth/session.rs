use super::bindings::{AuthError, ServerData};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

// JavaScript Date.now() binding for high-precision timestamps
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Date.now")]
    fn js_date_now() -> f64;

    // Performance.now() for monotonic time measurements
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

// Session data stored in localStorage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub server_time: u64,           // Server time when last synced
    pub client_time: u64,           // Client time when server_time was received
    pub performance_time: f64,      // Performance.now() when synced
    pub session_start: u64,         // Session start time (server-adjusted)
    pub subscription_token: String, // Encrypted subscription data
    pub subscription_expires: u64,  // Subscription expiration time
    pub offline_window_hours: u32,  // Allowed offline usage window
    pub checksum: String,           // Data integrity check
}

// Session state for reactive UI updates
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SessionState {
    #[default]
    Loading,
    Valid,
    Expired,
    Offline,
    TamperingDetected,
    SubscriptionExpired,
    NetworkError,
}

// Time synchronization data
#[derive(Debug, Clone)]
pub struct TimeSync {
    pub server_offset: i64,        // Difference between server and client time
    pub performance_baseline: f64, // Performance.now() baseline for calculations
    pub last_sync: u64,            // Last successful sync timestamp
}

// Public API functions that work with signals

// Initialize session with server data
pub fn initialize_session_data(
    server_data: ServerData,
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) -> Result<(), AuthError> {
    let current_time = js_date_now() as u64;
    let performance_time = now();

    // Create session data
    let session_data = SessionData {
        server_time: server_data.server_time,
        client_time: current_time,
        performance_time,
        session_start: server_data.server_time, // Session starts now
        subscription_token: server_data.subscription_token,
        subscription_expires: server_data.subscription_expires,
        offline_window_hours: server_data.offline_window_hours,
        checksum: String::new(), // Will be calculated below
    };

    // Calculate checksum for integrity
    let session_with_checksum = SessionData {
        checksum: calculate_checksum(&session_data),
        ..session_data
    };

    // Save to localStorage
    save_session_data(&session_with_checksum)?;

    // Update reactive signals
    session_data_signal.set(Some(session_with_checksum));
    update_session_state(session_data_signal, session_state_signal);

    Ok(())
}

// Load existing session from storage
pub fn load_existing_session(
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) {
    match storage_get_item(STORAGE_KEY) {
        Ok(Some(stored_data)) => {
            match serde_json::from_str::<SessionData>(&stored_data) {
                Ok(session_data) => {
                    // Verify data integrity
                    if verify_session_integrity(&session_data) {
                        // Check for clock tampering
                        if detect_clock_tampering(&session_data) {
                            log::error!("Clock tampering detected!");
                            session_state_signal.set(SessionState::TamperingDetected);
                            clear_session_data();
                            return;
                        }

                        session_data_signal.set(Some(session_data));
                        update_session_state(session_data_signal, session_state_signal);
                    } else {
                        log::error!("Session data integrity check failed");
                        session_state_signal.set(SessionState::TamperingDetected);
                        clear_session_data();
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse session data: {e}");
                    clear_session_data();
                }
            }
        }
        Ok(None) => {
            // No stored session data
            session_state_signal.set(SessionState::Loading);
        }
        Err(e) => {
            log::error!("Failed to load session: {e:?}");
            session_state_signal.set(SessionState::Loading);
        }
    }
}

// Clear session data
pub fn clear_session_data() {
    if let Err(e) = storage_remove_item(STORAGE_KEY) {
        log::warn!("Failed to remove session data: {e:?}");
    }
    if let Err(e) = storage_remove_item(CHECKSUM_KEY) {
        log::warn!("Failed to remove checksum: {e:?}");
    }
}

// Check if user has valid subscription
pub fn has_valid_subscription(session_data_signal: &RwSignal<Option<SessionData>>) -> bool {
    if let Some(session_data) = session_data_signal.get() {
        if let Some(current_time) = get_current_server_time(&session_data) {
            return current_time < session_data.subscription_expires;
        }
    }
    false
}

// Check if session is valid (not expired and no tampering detected)
pub fn is_session_valid(session_state_signal: &RwSignal<SessionState>) -> bool {
    matches!(
        session_state_signal.get(),
        SessionState::Valid | SessionState::Offline
    )
}

// Get current server-adjusted time
pub fn get_current_server_time(session_data: &SessionData) -> Option<u64> {
    let current_performance = now();
    let performance_elapsed = current_performance - session_data.performance_time;

    // Use performance time for precise elapsed calculation
    let estimated_server_time = session_data.server_time + (performance_elapsed as u64);

    Some(estimated_server_time)
}

// Update session state based on current conditions
pub fn update_session_state(
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) {
    if let Some(session_data) = session_data_signal.get() {
        if let Some(current_server_time) = get_current_server_time(&session_data) {
            // Check subscription expiry
            if current_server_time >= session_data.subscription_expires {
                session_state_signal.set(SessionState::SubscriptionExpired);
                return;
            }

            // Calculate session duration
            let session_duration_hours =
                (current_server_time - session_data.session_start) / (1000 * 60 * 60);

            // Check if we're within the offline window
            if session_duration_hours <= session_data.offline_window_hours as u64 {
                // TODO: Add network connectivity check here
                // For now, assume we're valid if within offline window
                session_state_signal.set(SessionState::Valid);
            } else {
                session_state_signal.set(SessionState::Expired);
            }
        } else {
            session_state_signal.set(SessionState::TamperingDetected);
        }
    } else {
        session_state_signal.set(SessionState::Loading);
    }
}

// Periodic session state updater - call this from a timer or effect
pub fn refresh_session_state(
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) {
    // Only update if we have session data and aren't in an error state
    match session_state_signal.get() {
        SessionState::Valid | SessionState::Offline => {
            update_session_state(session_data_signal, session_state_signal);
        }
        _ => {
            // Don't override error states
        }
    }
}

// Private helper functions

const STORAGE_KEY: &str = "secure_session_data";
const CHECKSUM_KEY: &str = "session_checksum_validation";
const MAX_CLOCK_DRIFT_MINUTES: i64 = 5; // Allow 5 minutes of clock drift

fn save_session_data(session_data: &SessionData) -> Result<(), AuthError> {
    let json = serde_json::to_string(session_data)
        .map_err(|e| AuthError::SerializationError(e.to_string()))?;

    storage_set_item(STORAGE_KEY, &json)?;

    // Store additional checksum for verification
    storage_set_item(CHECKSUM_KEY, &session_data.checksum)?;

    Ok(())
}

fn calculate_checksum(session_data: &SessionData) -> String {
    // Simple checksum using session data fields
    // In production, use a proper HMAC with a server-provided secret
    let data = format!(
        "{}{}{}{}{}{}",
        session_data.server_time,
        session_data.client_time,
        session_data.session_start,
        session_data.subscription_token,
        session_data.subscription_expires,
        session_data.offline_window_hours
    );

    format!("{:x}", fxhash::hash64(&data))
}

fn verify_session_integrity(session_data: &SessionData) -> bool {
    let expected_checksum = calculate_checksum(session_data);
    let stored_checksum = storage_get_item(CHECKSUM_KEY)
        .unwrap_or(None)
        .unwrap_or_default();

    session_data.checksum == expected_checksum && expected_checksum == stored_checksum
}

fn detect_clock_tampering(session_data: &SessionData) -> bool {
    let current_time = js_date_now() as u64;
    let performance_elapsed = now() - session_data.performance_time;
    let expected_client_time = session_data.client_time + (performance_elapsed as u64);

    // Check if client clock has moved backward significantly
    let clock_drift = current_time as i64 - expected_client_time as i64;
    let max_drift_ms = MAX_CLOCK_DRIFT_MINUTES * 60 * 1000;

    // Detect backward clock movement (potential tampering)
    if clock_drift < -max_drift_ms {
        log::warn!("Potential clock tampering detected: drift = {clock_drift}ms");
        return true;
    }

    false
}

// Storage helper functions that work with single-threaded web storage
fn storage_get_item(key: &str) -> Result<Option<String>, AuthError> {
    let window = window().ok_or_else(|| AuthError::JsError("No window object".to_string()))?;
    let storage = window
        .local_storage()
        .map_err(|_| AuthError::JsError("localStorage not available".to_string()))?
        .ok_or_else(|| AuthError::JsError("localStorage is null".to_string()))?;

    storage
        .get_item(key)
        .map_err(|_| AuthError::JsError("Failed to get item from storage".to_string()))
}

fn storage_set_item(key: &str, value: &str) -> Result<(), AuthError> {
    let window = window().ok_or_else(|| AuthError::JsError("No window object".to_string()))?;
    let storage = window
        .local_storage()
        .map_err(|_| AuthError::JsError("localStorage not available".to_string()))?
        .ok_or_else(|| AuthError::JsError("localStorage is null".to_string()))?;

    storage
        .set_item(key, value)
        .map_err(|_| AuthError::JsError("Failed to set item in storage".to_string()))
}

fn storage_remove_item(key: &str) -> Result<(), AuthError> {
    let window = window().ok_or_else(|| AuthError::JsError("No window object".to_string()))?;
    let storage = window
        .local_storage()
        .map_err(|_| AuthError::JsError("localStorage not available".to_string()))?
        .ok_or_else(|| AuthError::JsError("localStorage is null".to_string()))?;

    storage
        .remove_item(key)
        .map_err(|_| AuthError::JsError("Failed to remove item from storage".to_string()))
}

// FNV hash implementation for checksums (simple but effective)
mod fxhash {
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    pub fn hash64(data: &str) -> u64 {
        let mut hash = FNV_OFFSET_BASIS;
        for byte in data.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }
}
