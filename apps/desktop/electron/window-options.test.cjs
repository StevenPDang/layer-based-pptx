const assert = require('node:assert/strict');
const test = require('node:test');

const {
  createWindowOptions,
  getDevServerUrl,
  isAllowedNavigation,
} = require('./window-options.cjs');

test('creates an isolated and sandboxed renderer', () => {
  const options = createWindowOptions('/app/electron/preload.cjs');

  assert.equal(options.webPreferences.preload, '/app/electron/preload.cjs');
  assert.equal(options.webPreferences.contextIsolation, true);
  assert.equal(options.webPreferences.nodeIntegration, false);
  assert.equal(options.webPreferences.sandbox, true);
});

test('allows same-document navigation and blocks other destinations', () => {
  assert.equal(
    isAllowedNavigation(
      'http://127.0.0.1:5173/editor',
      'http://127.0.0.1:5173/editor#slide-2',
    ),
    true,
  );
  assert.equal(
    isAllowedNavigation(
      'http://127.0.0.1:5173/editor',
      'https://example.com/',
    ),
    false,
  );
});

test('only accepts the configured loopback development server', () => {
  assert.equal(
    getDevServerUrl(['electron', '.', '--dev-server=http://127.0.0.1:5173']),
    'http://127.0.0.1:5173',
  );
  assert.equal(
    getDevServerUrl(['electron', '.', '--dev-server=https://example.com']),
    undefined,
  );
});
