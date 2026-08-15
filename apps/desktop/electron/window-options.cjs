function createWindowOptions(preloadPath) {
  return {
    width: 1280,
    height: 800,
    minWidth: 960,
    minHeight: 640,
    show: false,
    backgroundColor: '#111827',
    webPreferences: {
      preload: preloadPath,
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
    },
  };
}

function isAllowedNavigation(currentUrl, targetUrl) {
  try {
    const current = new URL(currentUrl);
    const target = new URL(targetUrl);

    return (
      current.protocol === target.protocol &&
      current.host === target.host &&
      current.pathname === target.pathname &&
      current.search === target.search
    );
  } catch {
    return false;
  }
}

function getDevServerUrl(argumentsList) {
  const allowedUrl = 'http://127.0.0.1:5173';
  const argument = argumentsList.find((value) =>
    value.startsWith('--dev-server='),
  );

  return argument?.slice('--dev-server='.length) === allowedUrl
    ? allowedUrl
    : undefined;
}

module.exports = {
  createWindowOptions,
  getDevServerUrl,
  isAllowedNavigation,
};
