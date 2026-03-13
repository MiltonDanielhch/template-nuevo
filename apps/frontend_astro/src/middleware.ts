import { defineMiddleware } from "astro:middleware";

const STATIC_ASSETS_EXTENSIONS = [
  ".js",
  ".css",
  ".png",
  ".jpg",
  ".jpeg",
  ".gif",
  ".svg",
  ".ico",
  ".woff",
  ".woff2",
  ".ttf",
  ".eot",
  ".webp",
  ".avif",
];

function isStaticAsset(url: string): boolean {
  const pathname = new URL(url, "http://localhost").pathname;
  return STATIC_ASSETS_EXTENSIONS.some((ext) => pathname.endsWith(ext));
}

function getCacheControl(url: string, isDev: boolean): string {
  if (isDev) {
    return "no-cache, no-store, must-revalidate";
  }

  if (isStaticAsset(url)) {
    return "public, max-age=31536000, immutable";
  }

  if (url.startsWith("/api/")) {
    return "no-cache, no-store, must-revalidate";
  }

  if (url === "/" || url.startsWith("/sitemap") || url.startsWith("/robots.txt")) {
    return "public, max-age=3600, s-maxage=86400";
  }

  return "public, max-age=300";
}

export const onRequest = defineMiddleware(async (context, next) => {
  const response = await next();
  const url = context.url.pathname;
  const isDev = import.meta.env.DEV;

  const cacheControl = getCacheControl(url, isDev);
  response.headers.set("Cache-Control", cacheControl);

  if (!isDev && !url.startsWith("/api/")) {
    response.headers.set("X-Content-Type-Options", "nosniff");
    response.headers.set("X-Frame-Options", "DENY");
    response.headers.set("X-XSS-Protection", "1; mode=block");
  }

  return response;
});
