import { browser } from "$app/environment";

const fallbackApiUrl = "http://localhost:8080";

export const apiBaseUrl = import.meta.env.VITE_API_PUBLIC_URL ?? fallbackApiUrl;

export function apiUrl(path: string): string {
  const normalizedPath = path.startsWith("/") ? path : `/${path}`;
  return `${apiBaseUrl}${normalizedPath}`;
}

export function websocketUrl(path = "/api/v1/ws"): string {
  const base = new URL(apiBaseUrl);
  base.protocol = base.protocol === "https:" ? "wss:" : "ws:";
  base.pathname = path;
  base.search = "";
  base.hash = "";
  return base.toString();
}

export function currentClientPlatform(): string {
  if (!browser) return "server-render";
  return navigator.platform || "unknown";
}
