// Unlicense — cochranblock.org
// Service worker: aggressive offline caching for PWA.
const CACHE = 'wowasticker-v0.2.0';
const ASSETS = [
  '/wowasticker/',
  '/wowasticker/index.html',
  '/wowasticker/manifest.json',
  '/wowasticker/icon-192.svg',
  '/wowasticker/icon-512.svg',
  '/wowasticker/wowasticker_wasm.js',
  '/wowasticker/wowasticker_wasm_bg.wasm',
];

self.addEventListener('install', e => {
  e.waitUntil(caches.open(CACHE).then(c => c.addAll(ASSETS)));
  self.skipWaiting();
});

self.addEventListener('activate', e => {
  e.waitUntil(
    caches.keys().then(keys =>
      Promise.all(keys.filter(k => k !== CACHE).map(k => caches.delete(k)))
    )
  );
  self.clients.claim();
});

self.addEventListener('fetch', e => {
  e.respondWith(
    caches.match(e.request).then(r => r || fetch(e.request))
  );
});
