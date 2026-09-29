self.addEventListener('install', (e) => {
  self.skipWaiting();
});

self.addEventListener('activate', (e) => {
  e.waitUntil(self.clients.claim());
});

self.addEventListener('fetch', (e) => {
  // Simple pass-through fetch, required to pass PWA installability tests
  e.respondWith(fetch(e.request).catch(() => {
    return new Response('You are offline.');
  }));
});
