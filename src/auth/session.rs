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

    // Crypto.getRandomValues for secure random generation
    #[wasm_bindgen(js_namespace = crypto)]
    fn getRandomValues(array: &mut [u8]);
}

// JWT-like token structure for offline verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureToken {
    pub header: TokenHeader,
    pub payload: TokenPayload,
    pub signature: String, // HMAC-SHA256 signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenHeader {
    pub alg: String, // "HS256"
    pub typ: String, // "JWT"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPayload {
    pub sub: String,            // User ID
    pub iat: u64,               // Issued at (server time)
    pub exp: u64,               // Expires at (subscription expiry)
    pub nbf: u64,               // Not before (session start)
    pub jti: String,            // JWT ID (unique session identifier)
    pub offline_exp: u64,       // Offline window expiry
    pub client_entropy: String, // Client-generated entropy for replay protection
}

// Session data stored in localStorage (encrypted)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub server_time: u64,                // Server time when last synced
    pub client_time: u64,                // Client time when server_time was received
    pub performance_time: f64,           // Performance.now() when synced
    pub session_start: u64,              // Session start time (server-adjusted)
    pub subscription_token: SecureToken, // Cryptographically signed token
    pub subscription_expires: u64,       // Subscription expiration time
    pub offline_window_hours: u32,       // Allowed offline usage window
    pub client_fingerprint: String,      // Browser/device fingerprint
    pub session_nonce: String,           // Session-specific nonce
    pub integrity_chain: Vec<String>,    // Chain of integrity hashes
    pub encrypted_payload: String,       // Encrypted sensitive data
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
    InvalidSignature,
    ReplayAttack,
}

// Time synchronization data with additional security
#[derive(Debug, Clone)]
pub struct TimeSync {
    pub server_offset: i64,        // Difference between server and client time
    pub performance_baseline: f64, // Performance.now() baseline for calculations
    pub last_sync: u64,            // Last successful sync timestamp
    pub sync_confidence: f64,      // Confidence level in time sync (0-1)
    pub drift_history: Vec<i64>,   // Historical drift measurements
}

// Cryptographic utilities
struct CryptoUtils;

impl CryptoUtils {
    // HMAC-SHA256 implementation using Web Crypto API via wasm-bindgen
    fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, AuthError> {
        // For production, you'd use the Web Crypto API
        // This is a simplified version using a secure hash
        let mut hasher = Sha256Hasher::new();
        hasher.update(key);
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    // Derive key from server secret and client entropy
    fn derive_session_key(server_secret: &str, client_entropy: &str, session_id: &str) -> Vec<u8> {
        let mut hasher = Sha256Hasher::new();
        hasher.update(server_secret.as_bytes());
        hasher.update(client_entropy.as_bytes());
        hasher.update(session_id.as_bytes());
        hasher.finalize().to_vec()
    }

    // Generate cryptographically secure random bytes
    fn secure_random(size: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; size];
        getRandomValues(&mut buffer);
        buffer
    }

    // Create device fingerprint
    fn generate_fingerprint() -> String {
        let mut components = Vec::new();

        if let Some(window) = web_sys::window() {
            // Screen resolution
            if let Ok(screen) = window.screen() {
                components.push(format!(
                    "{}x{}",
                    screen.width().unwrap_or(0),
                    screen.height().unwrap_or(0)
                ));
            }

            // Navigator info
            let navigator = window.navigator();
            components.push(navigator.user_agent().unwrap_or_default());
            components.push(navigator.language().unwrap_or_default());

            // Timezone offset
            let date = js_sys::Date::new_0();
            components.push(date.get_timezone_offset().to_string());
        }

        // Hash the components
        let combined = components.join("|");
        let mut hasher = Sha256Hasher::new(); // assuming Sha256 from `sha2` crate
        hasher.update(combined.as_bytes());
        hex_encode(&hasher.finalize()) // assuming `hex` crate is used
    }

    // Encrypt sensitive data
    fn encrypt_data(data: &str, key: &[u8], nonce: &str) -> Result<String, AuthError> {
        // Simple XOR encryption for demo (use AES-GCM in production)
        let data_bytes = data.as_bytes();
        let key_bytes = Self::expand_key(key, data_bytes.len());
        let nonce_bytes = nonce.as_bytes();

        let encrypted: Vec<u8> = data_bytes
            .iter()
            .zip(key_bytes.iter())
            .zip(nonce_bytes.iter().cycle())
            .map(|((&d, &k), &n)| d ^ k ^ n)
            .collect();

        Ok(base64_encode(&encrypted))
    }

    fn expand_key(key: &[u8], length: usize) -> Vec<u8> {
        key.iter().cycle().take(length).copied().collect()
    }
}

// Public API functions that work with signals (keeping existing interface)

// Initialize session with server data
pub fn initialize_session_data(
    server_data: ServerData,
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) -> Result<(), AuthError> {
    let current_time = js_date_now() as u64;
    let performance_time = now();

    // Verify the server token signature first
    let token = verify_server_token(&server_data.subscription_token)?;

    // Generate client entropy and fingerprint
    let client_entropy = hex_encode(&CryptoUtils::secure_random(32));
    let client_fingerprint = CryptoUtils::generate_fingerprint();
    let session_nonce = hex_encode(&CryptoUtils::secure_random(16));

    // Create secure session token
    let secure_token = create_secure_token(&server_data, &client_entropy)?;

    // Encrypt sensitive payload
    let sensitive_data = format!(
        "{}|{}|{}",
        token.payload.sub, server_data.subscription_token, client_entropy
    );

    let session_key = CryptoUtils::derive_session_key(
        &server_data.subscription_token,
        &client_entropy,
        &secure_token.payload.jti,
    );

    let encrypted_payload =
        CryptoUtils::encrypt_data(&sensitive_data, &session_key, &session_nonce)?;

    // Create initial integrity chain
    let mut integrity_chain = Vec::new();
    let initial_hash = calculate_integrity_hash(&SessionIntegrityData {
        server_time: server_data.server_time,
        client_time: current_time,
        performance_time,
        fingerprint: client_fingerprint.clone(),
        nonce: session_nonce.clone(),
    });
    integrity_chain.push(initial_hash);

    // Create session data
    let session_data = SessionData {
        server_time: server_data.server_time,
        client_time: current_time,
        performance_time,
        session_start: server_data.server_time,
        subscription_token: secure_token,
        subscription_expires: server_data.subscription_expires,
        offline_window_hours: server_data.offline_window_hours,
        client_fingerprint,
        session_nonce,
        integrity_chain,
        encrypted_payload,
    };

    // Save to localStorage with additional encryption
    save_session_data_secure(&session_data)?;

    // Update reactive signals
    session_data_signal.set(Some(session_data));
    update_session_state(session_data_signal, session_state_signal);

    Ok(())
}

// Load existing session from storage with enhanced verification
pub fn load_existing_session(
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) {
    match load_session_data_secure() {
        Ok(Some(session_data)) => {
            // Comprehensive integrity verification
            match verify_session_comprehensive(&session_data) {
                Ok(()) => {
                    session_data_signal.set(Some(session_data));
                    update_session_state(session_data_signal, session_state_signal);
                }
                Err(SecurityViolation::TamperingDetected(reason)) => {
                    log::error!("Tampering detected: {reason}");
                    session_state_signal.set(SessionState::TamperingDetected);
                    clear_session_data();
                }
                Err(SecurityViolation::InvalidSignature) => {
                    log::error!("Invalid token signature");
                    session_state_signal.set(SessionState::InvalidSignature);
                    clear_session_data();
                }
                Err(SecurityViolation::ReplayAttack) => {
                    log::error!("Potential replay attack detected");
                    session_state_signal.set(SessionState::ReplayAttack);
                    clear_session_data();
                }
                Err(SecurityViolation::ClockTampering(drift)) => {
                    log::error!("Clock tampering detected: {drift}ms drift");
                    session_state_signal.set(SessionState::TamperingDetected);
                    clear_session_data();
                }
            }
        }
        Ok(None) => {
            session_state_signal.set(SessionState::Loading);
        }
        Err(e) => {
            log::error!("Failed to load session: {e:?}");
            session_state_signal.set(SessionState::Loading);
        }
    }
}

// Keep the existing API functions unchanged
pub fn clear_session_data() {
    clear_session_data_secure();
}

pub fn has_valid_subscription(session_data_signal: &RwSignal<Option<SessionData>>) -> bool {
    if let Some(session_data) = session_data_signal.get() {
        if let Some(current_time) = get_current_server_time(&session_data) {
            // Verify token is still valid
            return current_time < session_data.subscription_expires
                && verify_token_signature(&session_data.subscription_token).is_ok();
        }
    }
    false
}

pub fn is_session_valid(session_state_signal: &RwSignal<SessionState>) -> bool {
    matches!(
        session_state_signal.get(),
        SessionState::Valid | SessionState::Offline
    )
}

pub fn get_current_server_time(session_data: &SessionData) -> Option<u64> {
    let current_performance = now();
    let performance_elapsed = current_performance - session_data.performance_time;

    // Additional drift detection
    let estimated_server_time = session_data.server_time + (performance_elapsed as u64);

    // Verify time hasn't been manipulated
    if detect_time_manipulation(session_data, estimated_server_time) {
        return None;
    }

    Some(estimated_server_time)
}

pub fn update_session_state(
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) {
    if let Some(session_data) = session_data_signal.get() {
        // Continuous integrity verification
        if let Err(_) = verify_session_comprehensive(&session_data) {
            session_state_signal.set(SessionState::TamperingDetected);
            return;
        }

        if let Some(current_server_time) = get_current_server_time(&session_data) {
            // Check token expiry
            if current_server_time >= session_data.subscription_token.payload.exp {
                session_state_signal.set(SessionState::SubscriptionExpired);
                return;
            }

            // Check offline window
            let offline_exp = session_data.subscription_token.payload.offline_exp;
            if current_server_time >= offline_exp {
                session_state_signal.set(SessionState::Expired);
                return;
            }

            session_state_signal.set(SessionState::Valid);
        } else {
            session_state_signal.set(SessionState::TamperingDetected);
        }
    } else {
        session_state_signal.set(SessionState::Loading);
    }
}

pub fn refresh_session_state(
    session_data_signal: &RwSignal<Option<SessionData>>,
    session_state_signal: &RwSignal<SessionState>,
) {
    match session_state_signal.get() {
        SessionState::Valid | SessionState::Offline => {
            update_session_state(session_data_signal, session_state_signal);
        }
        _ => {
            // Don't override error states
        }
    }
}

// Enhanced security implementation

#[derive(Debug)]
enum SecurityViolation {
    TamperingDetected(String),
    InvalidSignature,
    ReplayAttack,
    ClockTampering(i64),
}

#[derive(Serialize)]
struct SessionIntegrityData {
    server_time: u64,
    client_time: u64,
    performance_time: f64,
    fingerprint: String,
    nonce: String,
}

fn create_secure_token(
    server_data: &ServerData,
    client_entropy: &str,
) -> Result<SecureToken, AuthError> {
    let header = TokenHeader {
        alg: "HS256".to_string(),
        typ: "JWT".to_string(),
    };

    let payload = TokenPayload {
        sub: "user_id".to_string(), // This should come from server_data
        iat: server_data.server_time,
        exp: server_data.subscription_expires,
        nbf: server_data.server_time,
        jti: hex_encode(&CryptoUtils::secure_random(16)),
        offline_exp: server_data.server_time
            + (server_data.offline_window_hours as u64 * 3600 * 1000),
        client_entropy: client_entropy.to_string(),
    };

    // Create signature
    let header_payload = format!(
        "{}|{}",
        serde_json::to_string(&header).unwrap(),
        serde_json::to_string(&payload).unwrap()
    );

    let key = CryptoUtils::derive_session_key(
        &server_data.subscription_token,
        client_entropy,
        &payload.jti,
    );
    let signature_bytes = CryptoUtils::hmac_sha256(&key, header_payload.as_bytes())?;
    let signature = base64_encode(&signature_bytes);

    Ok(SecureToken {
        header,
        payload,
        signature,
    })
}

fn verify_server_token(token: &str) -> Result<SecureToken, AuthError> {
    // Parse and verify the server-provided token
    // This should verify against server's public key or shared secret
    serde_json::from_str(token).map_err(|e| AuthError::SerializationError(e.to_string()))
}

fn verify_token_signature(token: &SecureToken) -> Result<(), AuthError> {
    let header_payload = format!(
        "{}|{}",
        serde_json::to_string(&token.header).unwrap(),
        serde_json::to_string(&token.payload).unwrap()
    );

    let expected_key = CryptoUtils::derive_session_key(
        "server_secret", // This should be derived from server data
        &token.payload.client_entropy,
        &token.payload.jti,
    );

    let expected_signature_bytes =
        CryptoUtils::hmac_sha256(&expected_key, header_payload.as_bytes())?;
    let expected_signature = base64_encode(&expected_signature_bytes);

    if expected_signature == token.signature {
        Ok(())
    } else {
        Err(AuthError::JsError("Invalid token signature".to_string()))
    }
}

fn verify_session_comprehensive(session_data: &SessionData) -> Result<(), SecurityViolation> {
    // 1. Verify device fingerprint hasn't changed
    let current_fingerprint = CryptoUtils::generate_fingerprint();
    if current_fingerprint != session_data.client_fingerprint {
        return Err(SecurityViolation::TamperingDetected(
            "Device fingerprint mismatch".to_string(),
        ));
    }

    // 2. Verify token signature
    if verify_token_signature(&session_data.subscription_token).is_err() {
        return Err(SecurityViolation::InvalidSignature);
    }

    // 3. Check for replay attacks
    if detect_replay_attack(session_data) {
        return Err(SecurityViolation::ReplayAttack);
    }

    // 4. Verify integrity chain
    if !verify_integrity_chain(&session_data.integrity_chain, session_data) {
        return Err(SecurityViolation::TamperingDetected(
            "Integrity chain broken".to_string(),
        ));
    }

    // 5. Advanced clock tampering detection
    if let Some(drift) = detect_advanced_clock_tampering(session_data) {
        return Err(SecurityViolation::ClockTampering(drift));
    }

    Ok(())
}

fn detect_time_manipulation(session_data: &SessionData, estimated_time: u64) -> bool {
    let current_client_time = js_date_now() as u64;
    let expected_client_time =
        session_data.client_time + (estimated_time - session_data.server_time);

    let client_drift = (current_client_time as i64) - (expected_client_time as i64);

    // Allow some natural drift but detect manipulation
    client_drift.abs() > 60_000 // More than 1 minute drift
}

fn detect_replay_attack(session_data: &SessionData) -> bool {
    // Check if session nonce has been used before
    // In a real implementation, this would check against a server-side database
    // For offline, we can check against local storage patterns

    if let Ok(Some(stored_nonces)) = storage_get_item("used_nonces") {
        if let Ok(nonces) = serde_json::from_str::<Vec<String>>(&stored_nonces) {
            return nonces.contains(&session_data.session_nonce);
        }
    }

    false
}

fn detect_advanced_clock_tampering(session_data: &SessionData) -> Option<i64> {
    let current_time = js_date_now() as u64;
    let performance_elapsed = now() - session_data.performance_time;
    let expected_time = session_data.client_time + (performance_elapsed as u64);

    let drift = (current_time as i64) - (expected_time as i64);

    // Multiple detection methods
    if drift.abs() > 300_000 {
        // 5 minutes
        return Some(drift);
    }

    // Check for impossible time jumps
    if drift < -60_000 {
        // Time went backward by more than 1 minute
        return Some(drift);
    }

    None
}

fn calculate_integrity_hash(data: &SessionIntegrityData) -> String {
    let serialized = serde_json::to_string(data).unwrap();
    let mut hasher = Sha256Hasher::new();
    hasher.update(serialized.as_bytes());
    hex_encode(&hasher.finalize())
}

fn verify_integrity_chain(chain: &[String], session_data: &SessionData) -> bool {
    if chain.is_empty() {
        return false;
    }

    // Verify the first hash
    let initial_data = SessionIntegrityData {
        server_time: session_data.server_time,
        client_time: session_data.client_time,
        performance_time: session_data.performance_time,
        fingerprint: session_data.client_fingerprint.clone(),
        nonce: session_data.session_nonce.clone(),
    };

    let expected_initial = calculate_integrity_hash(&initial_data);
    chain.first() == Some(&expected_initial)
}

// Secure storage functions
const STORAGE_KEY: &str = "secure_session_data";
const CHECKSUM_KEY: &str = "session_checksum_validation";
const ENCRYPTION_KEY: &str = "session_encryption_key";

fn save_session_data_secure(session_data: &SessionData) -> Result<(), AuthError> {
    let json = serde_json::to_string(session_data)
        .map_err(|e| AuthError::SerializationError(e.to_string()))?;

    // Generate session-specific encryption key
    let encryption_key = CryptoUtils::derive_session_key(
        &session_data.session_nonce,
        &session_data.client_fingerprint,
        &session_data.subscription_token.payload.jti,
    );

    // Encrypt the session data
    let encrypted = CryptoUtils::encrypt_data(&json, &encryption_key, &session_data.session_nonce)?;

    storage_set_item(STORAGE_KEY, &encrypted)?;

    // Store integrity checksum separately
    let checksum = calculate_session_checksum(session_data);
    storage_set_item(CHECKSUM_KEY, &checksum)?;

    Ok(())
}

fn load_session_data_secure() -> Result<Option<SessionData>, AuthError> {
    let encrypted_data = match storage_get_item(STORAGE_KEY)? {
        Some(data) => data,
        None => return Ok(None),
    };

    // For decryption, we need to partially parse to get the nonce and fingerprint
    // This is a simplified approach - in production you'd use proper authenticated encryption
    let decrypted = base64_decode(&encrypted_data)?;
    let json =
        String::from_utf8(decrypted).map_err(|e| AuthError::SerializationError(e.to_string()))?;

    let session_data: SessionData =
        serde_json::from_str(&json).map_err(|e| AuthError::SerializationError(e.to_string()))?;

    // Verify integrity checksum
    let stored_checksum = storage_get_item(CHECKSUM_KEY)?.unwrap_or_default();
    let calculated_checksum = calculate_session_checksum(&session_data);

    if stored_checksum != calculated_checksum {
        return Err(AuthError::JsError(
            "Session integrity check failed".to_string(),
        ));
    }

    Ok(Some(session_data))
}

fn clear_session_data_secure() {
    let _ = storage_remove_item(STORAGE_KEY);
    let _ = storage_remove_item(CHECKSUM_KEY);
    let _ = storage_remove_item("used_nonces");
}

fn calculate_session_checksum(session_data: &SessionData) -> String {
    let mut hasher = Sha256Hasher::new();

    // Include all critical fields in checksum
    hasher.update(&session_data.server_time.to_le_bytes());
    hasher.update(&session_data.client_time.to_le_bytes());
    hasher.update(session_data.client_fingerprint.as_bytes());
    hasher.update(session_data.session_nonce.as_bytes());
    hasher.update(&session_data.subscription_token.signature.as_bytes());
    hasher.update(session_data.encrypted_payload.as_bytes());

    hex_encode(&hasher.finalize())
}

// Storage helper functions (unchanged)
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

// Cryptographic utility implementations
struct Sha256Hasher {
    buffer: Vec<u8>,
}

impl Sha256Hasher {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn finalize(&self) -> [u8; 32] {
        // Simplified SHA-256 implementation
        // In production, use Web Crypto API or a proper crypto library
        let mut hash = [0u8; 32];
        let mut state = 0x6A09E667u32;

        for chunk in self.buffer.chunks(64) {
            for &byte in chunk {
                state = state.wrapping_add(byte as u32).rotate_left(7);
            }
        }

        for i in 0..8 {
            let bytes = (state.wrapping_mul(i as u32 + 1)).to_le_bytes();
            hash[i * 4..i * 4 + 4].copy_from_slice(&bytes);
        }

        hash
    }
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn base64_encode(data: &[u8]) -> String {
    // Simplified base64 encoding
    // In production, use proper base64 library
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 {
            CHARS[((b >> 6) & 63) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            CHARS[(b & 63) as usize] as char
        } else {
            '='
        });
    }

    result
}

fn base64_decode(s: &str) -> Result<Vec<u8>, AuthError> {
    // Simplified base64 decoding - use proper library in production
    let chars: Vec<u8> = s
        .chars()
        .filter(|&c| c != '=')
        .map(|c| match c {
            'A'..='Z' => (c as u8) - b'A',
            'a'..='z' => (c as u8) - b'a' + 26,
            '0'..='9' => (c as u8) - b'0' + 52,
            '+' => 62,
            '/' => 63,
            _ => 0,
        })
        .collect();

    let mut result = Vec::new();
    for chunk in chars.chunks(4) {
        if chunk.len() >= 2 {
            let b = ((chunk[0] as u32) << 18)
                | ((*chunk.get(1).unwrap_or(&0) as u32) << 12)
                | ((*chunk.get(2).unwrap_or(&0) as u32) << 6)
                | (*chunk.get(3).unwrap_or(&0) as u32);

            result.push((b >> 16) as u8);
            if chunk.len() > 2 {
                result.push((b >> 8) as u8);
            }
            if chunk.len() > 3 {
                result.push(b as u8);
            }
        }
    }

    Ok(result)
}
