interface DesktopApi {
  platform: NodeJS.Platform;
  versions: Readonly<{
    chrome: string;
    electron: string;
  }>;
}

declare global {
  interface Window {
    desktop?: DesktopApi;
  }
}

export {};
