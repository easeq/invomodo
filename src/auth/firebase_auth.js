import { initializeApp } from 'firebase/app';
import {
  getAuth,
  GoogleAuthProvider,
  signInWithPopup,
  signOut,
  onAuthStateChanged
} from 'firebase/auth';

// Firebase configuration - replace with your actual config
const firebaseConfig = {
  apiKey: "your-api-key",
  authDomain: "your-project.firebaseapp.com",
  projectId: "your-project-id",
  storageBucket: "your-project.appspot.com",
  messagingSenderId: "123456789",
  appId: "your-app-id"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const auth = getAuth(app);
const provider = new GoogleAuthProvider();

// Global auth state callback - will be set from Rust
let authStateCallback = null;

/**
 * Initialize Firebase auth with callback for state changes
 * @param {function} callback - Function to call on auth state changes
 */
export function initializeAuth(callback) {
  authStateCallback = callback;

  // Set up auth state listener
  onAuthStateChanged(auth, (user) => {
    if (authStateCallback) {
      if (user) {
        // User is signed in
        const userData = {
          uid: user.uid,
          email: user.email,
          displayName: user.displayName || '',
          photoURL: user.photoURL || ''
        };
        authStateCallback(JSON.stringify(userData));
      } else {
        // User is signed out
        authStateCallback(null);
      }
    }
  });

  // Return current user if already authenticated
  if (auth.currentUser) {
    const userData = {
      uid: auth.currentUser.uid,
      email: auth.currentUser.email,
      displayName: auth.currentUser.displayName || '',
      photoURL: auth.currentUser.photoURL || ''
    };
    return JSON.stringify(userData);
  }

  return null;
}

/**
 * Sign in with Google
 * @returns {Promise<string|null>} User data as JSON string or null if failed
 */
export async function signInWithGoogle() {
  try {
    const result = await signInWithPopup(auth, provider);
    const user = result.user;

    // Get Firebase ID token for backend authentication
    const idToken = await user.getIdToken();

    const userData = {
      uid: user.uid,
      email: user.email,
      displayName: user.displayName || '',
      photoURL: user.photoURL || '',
      idToken: idToken
    };

    return JSON.stringify(userData);
  } catch (error) {
    console.error('Sign in failed:', error);
    throw new Error(`Sign in failed: ${error.message}`);
  }
}

/**
 * Sign out current user
 * @returns {Promise<void>}
 */
export async function signOutUser() {
  try {
    await signOut(auth);
  } catch (error) {
    console.error('Sign out failed:', error);
    throw new Error(`Sign out failed: ${error.message}`);
  }
}

/**
 * Get current user's ID token
 * @returns {Promise<string|null>} ID token or null if not authenticated
 */
export async function getCurrentUserToken() {
  try {
    if (auth.currentUser) {
      return await auth.currentUser.getIdToken();
    }
    return null;
  } catch (error) {
    console.error('Failed to get user token:', error);
    return null;
  }
}

/**
 * Get current user data
 * @returns {string|null} User data as JSON string or null
 */
export function getCurrentUser() {
  if (auth.currentUser) {
    const userData = {
      uid: auth.currentUser.uid,
      email: auth.currentUser.email,
      displayName: auth.currentUser.displayName || '',
      photoURL: auth.currentUser.photoURL || ''
    };
    return JSON.stringify(userData);
  }
  return null;
}

/**
 * Fetch server time and subscription data from backend
 * @param {string} idToken - Firebase ID token
 * @returns {Promise<string>} Server data as JSON string
 */
export async function fetchServerData(idToken) {
  try {
    const response = await fetch('/api/user/session', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${idToken}`
      }
    });

    if (!response.ok) {
      throw new Error(`Server request failed: ${response.status}`);
    }

    const data = await response.json();

    // Expected server response format:
    // {
    //   serverTime: 1640995200000, // Unix timestamp in milliseconds
    //   subscriptionToken: "encrypted_subscription_data",
    //   subscriptionExpires: 1641081600000, // Unix timestamp
    //   offlineWindowHours: 12
    // }

    return JSON.stringify(data);
  } catch (error) {
    console.error('Failed to fetch server data:', error);
    throw new Error(`Server data fetch failed: ${error.message}`);
  }
}

// Make functions available globally for WASM
window.initializeAuth = initializeAuth;
window.signInWithGoogle = signInWithGoogle;
window.signOutUser = signOutUser;
window.getCurrentUserToken = getCurrentUserToken;
window.getCurrentUser = getCurrentUser;
window.fetchServerData = fetchServerData;
