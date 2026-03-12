import type { APIRoute } from "astro";

const DEFAULT_PAGES = ["/", "/login"];

function formatDate(date: Date) {
  return date.toISOString().split("T")[0];
}

export const GET: APIRoute = ({ request }) => {
  const now = new Date();
  const baseUrl = import.meta.env.SITE ?? new URL(request.url).origin;

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${DEFAULT_PAGES.map((path) => {
  const url = `${baseUrl.replace(/\/$/, "")}${path}`;
  return `  <url>
    <loc>${url}</loc>
    <lastmod>${formatDate(now)}</lastmod>
    <changefreq>weekly</changefreq>
  </url>`;
}).join("\n")}
</urlset>`;

  return new Response(xml, {
    status: 200,
    headers: {
      "Content-Type": "application/xml",
      "Cache-Control": "public, max-age=3600",
    },
  });
};
