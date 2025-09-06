const CACHE_NAME = 'invomodo-v1.0.0';
const STATIC_CACHE_NAME = 'invomodo-static-v1.0.0';
const RUNTIME_CACHE_NAME = 'invomodo-runtime-v1.0.0';

// Assets to cache immediately on install
const STATIC_ASSETS = [
  '/',
  '/index.html',
  '/manifest.json',
  '/firebase_auth.js',
  '/pkg/secure_pwa.js',
  '/pkg/secure_pwa_bg.wasm',
  // Add other static assets as needed
];

// Routes to cache with network-first strategy
const NETWORK_FIRST_ROUTES = [
  '/api/',
  '/auth/'
];

// Routes to cache with cache-first strategy
const CACHE_FIRST_ROUTES = [
  '/static/',
  '/icons/',
  '/assets/'
];

// Install event - cache static assets
self.addEventListener('install', (event) => {
  console.log('Service Worker installing...');

  event.waitUntil(
    caches.open(STATIC_CACHE_NAME)
      .then((cache) => {
        console.log('Caching static assets...');
        return cache.addAll(STATIC_ASSETS);
      })
      .then(() => {
        // Force the waiting service worker to become the active service worker
        return self.skipWaiting();
      })
      .catch((error) => {
        console.error('Failed to cache static assets:', error);
      })
  );
});

// Activate event - clean up old caches
self.addEventListener('activate', (event) => {
  console.log('Service Worker activating...');

  event.waitUntil(
    caches.keys()
      .then((cacheNames) => {
        const deletionPromises = cacheNames
          .filter((cacheName) => {
            // Delete old versions of our caches
            return cacheName.startsWith('invomodo-') &&
              ![STATIC_CACHE_NAME, RUNTIME_CACHE_NAME].includes(cacheName);
          })
          .map((cacheName) => {
            console.log('Deleting old cache:', cacheName);
            return caches.delete(cacheName);
          });

        return Promise.all(deletionPromises);
      })
      .then(() => {
        // Take control of all clients immediately
        return self.clients.claim();
      })
      .catch((error) => {
        console.error('Failed to activate service worker:', error);
      })
  );
});

// Fetch event - handle requests with appropriate caching strategies
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const { url, method } = request;

  // Only handle GET requests
  if (method !== 'GET') {
    return;
  }

  // Skip Chrome extension requests
  if (url.startsWith('chrome-extension://')) {
    return;
  }

  // Determine caching strategy based on URL
  if (isNetworkFirstRoute(url)) {
    event.respondWith(networkFirstStrategy(request));
  } else if (isCacheFirstRoute(url)) {
    event.respondWith(cacheFirstStrategy(request));
  } else if (isStaticAsset(url)) {
    event.respondWith(cacheOnlyStrategy(request));
  } else {
    event.respondWith(staleWhileRevalidateStrategy(request));
  }
});

// Network-first strategy (for API calls and dynamic content)
async function networkFirstStrategy(request) {
  try {
    // Try network first
    const networkResponse = await fetch(request);

    // Cache successful responses
    if (networkResponse.ok) {
      const cache = await caches.open(RUNTIME_CACHE_NAME);
      cache.put(request, networkResponse.clone());
    }

    return networkResponse;
  } catch (error) {
    console.log('Network request failed, falling back to cache:', request.url);

    // Fallback to cache
    const cachedResponse = await caches.match(request);
    if (cachedResponse) {
      return cachedResponse;
    }

    // If no cache, return offline page for navigation requests
    if (request.mode === 'navigate') {
      return caches.match('/offline.html') || new Response(
        getOfflineHTML(),
        { headers: { 'Content-Type': 'text/html' } }
      );
    }

    // For other requests, return a generic error response
    return new Response('Offline - Resource not available', {
      status: 503,
      statusText: 'Service Unavailable'
    });
  }
}

// Cache-first strategy (for static assets)
async function cacheFirstStrategy(request) {
  const cachedResponse = await caches.match(request);

  if (cachedResponse) {
    return cachedResponse;
  }

  try {
    const networkResponse = await fetch(request);

    if (networkResponse.ok) {
      const cache = await caches.open(RUNTIME_CACHE_NAME);
      cache.put(request, networkResponse.clone());
    }

    return networkResponse;
  } catch (error) {
    console.error('Failed to fetch resource:', request.url, error);
    return new Response('Resource not available', {
      status: 404,
      statusText: 'Not Found'
    });
  }
}

// Cache-only strategy (for pre-cached static assets)
async function cacheOnlyStrategy(request) {
  return caches.match(request);
}

// Stale-while-revalidate strategy (for app shell and general content)
async function staleWhileRevalidateStrategy(request) {
  const cachedResponse = await caches.match(request);

  const networkResponsePromise = fetch(request)
    .then((networkResponse) => {
      if (networkResponse.ok) {
        const cache = caches.open(RUNTIME_CACHE_NAME);
        cache.then(c => c.put(request, networkResponse.clone()));
      }
      return networkResponse;
    })
    .catch(() => null);

  return cachedResponse || networkResponsePromise || new Response(
    'Offline - Resource not available',
    { status: 503, statusText: 'Service Unavailable' }
  );
}

// Helper functions to determine caching strategy
function isNetworkFirstRoute(url) {
  return NETWORK_FIRST_ROUTES.some(route => url.includes(route));
}

function isCacheFirstRoute(url) {
  return CACHE_FIRST_ROUTES.some(route => url.includes(route));
}

function isStaticAsset(url) {
  return STATIC_ASSETS.some(asset => url.endsWith(asset));
}

// Generate offline HTML page
function getOfflineHTML() {
  return `
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Offline - SecurePWA</title>
        <style>
            body { 
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                margin: 0; 
                padding: 0; 
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                min-height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                color: white;
                text-align: center;
            }
            .container { 
                max-width: 400px; 
                padding: 2rem;
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 20px;
                border: 1px solid rgba(255, 255, 255, 0.2);
            }
            h1 { 
                font-size: 2.5rem; 
                margin-bottom: 1rem;
                font-weight: 300;
            }
            p { 
                font-size: 1.1rem; 
                line-height: 1.6;
                margin-bottom: 2rem;
                opacity: 0.9;
            }
            .retry-btn {
                background: rgba(255, 255, 255, 0.2);
                border: 1px solid rgba(255, 255, 255, 0.3);
                color: white;
                padding: 12px 24px;
                border-radius: 25px;
                cursor: pointer;
                font-size: 1rem;
                transition: all 0.3s ease;
            }
            .retry-btn:hover {
                background: rgba(255, 255, 255, 0.3);
                transform: translateY(-2px);
            }
            .offline-icon {
                font-size: 4rem;
                margin-bottom: 1rem;
                animation: pulse 2s infinite;
            }
            @keyframes pulse {
                0% { opacity: 1; }
                50% { opacity: 0.5; }
                100% { opacity: 1; }
            }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="offline-icon">📡</div>
            <h1>You're Offline</h1>
            <p>It looks like you're not connected to the internet. Some features may not be available, but you can still use cached content.</p>
            <button class="retry-btn" onclick="location.reload()">Try Again</button>
        </div>
        
        <script>
            // Auto-retry when connection is restored
            window.addEventListener('online', () => {
                location.reload();
            });
            
            // Show connection status
            if (navigator.onLine) {
                document.querySelector('p').textContent = 'Connection restored! Reloading...';
                setTimeout(() => location.reload(), 1000);
            }
        </script>
    </body>
    </html>
  `;
}

// Handle background sync for when connection is restored
self.addEventListener('sync', (event) => {
  console.log('Background sync triggered:', event.tag);

  if (event.tag === 'session-refresh') {
    event.waitUntil(refreshUserSession());
  }
});

// Handle push notifications (if needed in the future)
self.addEventListener('push', (event) => {
  if (event.data) {
    const data = event.data.json();
    console.log('Push notification received:', data);

    const options = {
      body: data.body || 'New notification from SecurePWA',
      icon: '/icons/icon-192x192.png',
      badge: '/icons/badge-72x72.png',
      tag: data.tag || 'default',
      requireInteraction: data.requireInteraction || false,
      actions: data.actions || []
    };

    event.waitUntil(
      self.registration.showNotification(data.title || 'SecurePWA', options)
    );
  }
});

// Handle notification clicks
self.addEventListener('notificationclick', (event) => {
  console.log('Notification clicked:', event.notification);

  event.notification.close();

  const urlToOpen = event.notification.data?.url || '/';

  event.waitUntil(
    clients.matchAll({ type: 'window', includeUncontrolled: true })
      .then((clientList) => {
        // Check if app is already open
        for (const client of clientList) {
          if (client.url.includes(self.location.origin)) {
            client.focus();
            client.postMessage({
              type: 'NOTIFICATION_CLICK',
              url: urlToOpen
            });
            return;
          }
        }

        // Open new window if app is not open
        return clients.openWindow(urlToOpen);
      })
  );
});

// Refresh user session when connection is restored
async function refreshUserSession() {
  try {
    // This would communicate with the main app to refresh session
    const clients = await self.clients.matchAll({ includeUncontrolled: true });

    clients.forEach(client => {
      client.postMessage({
        type: 'REFRESH_SESSION',
        timestamp: Date.now()
      });
    });

    console.log('Session refresh request sent to clients');
  } catch (error) {
    console.error('Failed to refresh session:', error);
  }
}

// Message handler for communication with main app
self.addEventListener('message', (event) => {
  const { data } = event;

  switch (data.type) {
    case 'SKIP_WAITING':
      self.skipWaiting();
      break;

    case 'CACHE_URLS':
      if (data.urls && Array.isArray(data.urls)) {
        cacheUrls(data.urls);
      }
      break;

    case 'CLEAR_CACHE':
      clearAllCaches();
      break;

    case 'GET_CACHE_STATUS':
      getCacheStatus().then(status => {
        event.ports[0].postMessage(status);
      });
      break;

    default:
      console.log('Unknown message type:', data.type);
  }
});

// Cache specific URLs on demand
async function cacheUrls(urls) {
  try {
    const cache = await caches.open(RUNTIME_CACHE_NAME);

    const cachePromises = urls.map(async (url) => {
      try {
        const response = await fetch(url);
        if (response.ok) {
          await cache.put(url, response);
          console.log('Cached:', url);
        }
      } catch (error) {
        console.warn('Failed to cache:', url, error);
      }
    });

    await Promise.all(cachePromises);
    console.log('Finished caching requested URLs');
  } catch (error) {
    console.error('Failed to cache URLs:', error);
  }
}

// Clear all caches
async function clearAllCaches() {
  try {
    const cacheNames = await caches.keys();
    const deletionPromises = cacheNames.map(cacheName => caches.delete(cacheName));
    await Promise.all(deletionPromises);
    console.log('All caches cleared');
  } catch (error) {
    console.error('Failed to clear caches:', error);
  }
}

// Get cache status and size information
async function getCacheStatus() {
  try {
    const cacheNames = await caches.keys();
    const status = {
      cacheNames,
      totalSize: 0,
      cacheDetails: {}
    };

    for (const cacheName of cacheNames) {
      const cache = await caches.open(cacheName);
      const requests = await cache.keys();
      let cacheSize = 0;

      for (const request of requests) {
        try {
          const response = await cache.match(request);
          if (response) {
            const blob = await response.blob();
            cacheSize += blob.size;
          }
        } catch (error) {
          console.warn('Failed to get size for:', request.url);
        }
      }

      status.cacheDetails[cacheName] = {
        itemCount: requests.length,
        size: cacheSize
      };
      status.totalSize += cacheSize;
    }

    return status;
  } catch (error) {
    console.error('Failed to get cache status:', error);
    return { error: error.message };
  }
}
