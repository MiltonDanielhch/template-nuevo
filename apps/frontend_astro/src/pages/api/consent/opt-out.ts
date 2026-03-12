import type { APIRoute } from "astro";

export const GET: APIRoute = () => {
  const html = `<!DOCTYPE html>
<html lang="es">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Opt-out de cookies</title>
    <script>
      localStorage.removeItem('landing_cookie_consent');
      window.location.href = '/';
    </script>
  </head>
  <body>
    <p>Redirigiendo...</p>
  </body>
</html>`;

  return new Response(html, {
    status: 200,
    headers: {
      "Content-Type": "text/html; charset=utf-8",
    },
  });
};
