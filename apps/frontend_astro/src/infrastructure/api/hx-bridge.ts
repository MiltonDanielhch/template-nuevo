export const HTMX_CONFIG = {
  DEFAULT_SWAP: "innerHTML",
  SETTLE_DELAY: 0,
  HISTORY_CACHE_SIZE: 0,
  REFRESH_ON_HISTORY_MISS: false,
};

export function getHtmxHeaders(): Record<string, string> {
  const csrfToken = document.querySelector('meta[name="csrf-token"]')?.getAttribute("content");

  return {
    "Content-Type": "application/json",
    ...(csrfToken ? { "X-CSRF-Token": csrfToken } : {}),
  };
}

export function initHtmxConfig(): void {
  if (typeof window === "undefined" || typeof htmx === "undefined") return;

  htmx.config.defaultSwapStyle = HTMX_CONFIG.DEFAULT_SWAP;
  htmx.config.defaultSettleDelay = HTMX_CONFIG.SETTLE_DELAY;
  htmx.config.historyCacheSize = HTMX_CONFIG.HISTORY_CACHE_SIZE;
  htmx.config.refreshOnHistoryMiss = HTMX_CONFIG.REFRESH_ON_HISTORY_MISS;
}

export function setupHtmxIndicator(_selector: string): void {
  document.addEventListener("htmx:beforeRequest", (e) => {
    const target = e.detail?.target as HTMLElement;
    if (!target) return;
    const indicator = target.querySelector(".htmx-indicator");
    if (indicator) indicator.classList.remove("hidden");
  });

  document.addEventListener("htmx:afterRequest", (e) => {
    const target = e.detail?.target as HTMLElement;
    if (!target) return;
    const indicator = target.querySelector(".htmx-indicator");
    if (indicator) indicator.classList.add("hidden");
  });
}
