# 🧪 Manual de Pruebas - Landing Page 3026

**Objetivo:** Validar que la landing page funcione correctamente (SEO, captación de leads, UI/UX, análisis) sin depender de JS extra y con una experiencia consistente en todo el stack.

---

## 🏗️ Nivel 1: Verificar build & SSR

### Comando Maestro
```bash
cd apps/frontend_astro
bun run build
```

**Qué verifica:**
1.  **Build SSR:** Astro compila la landing como HTML estático/SSR.
2.  **TypeScript:** No hay errores de tipado en los módulos de la landing.
3.  **Assets:** `public/og-image.svg` y `public/images/hero-illustration.svg` se copian a `dist/`.

**Resultado esperado:**
```text
✓ Completed in 10-15s
output: "server"
adapter: @astrojs/node
dist/client/*.js (bundled)
dist/server/*.server (entry)
```

---

## 🌐 Nivel 2: Verificar SEO y meta tags

1.  Abre el navegador en `http://localhost:4321/`
2.  Inspecciona el `<head>` y verifica:
    - `<meta name="description" ...>` tiene el copy correcto
    - Open Graph: `<meta property="og:title" ...>` y `<meta property="og:image" ...>`
    - `<link rel="canonical" ...>` apunta a la URL correcta
3.  Verifica `sitemap.xml`:
```bash
curl -s http://localhost:4321/sitemap.xml | head
```

---

## 🧩 Nivel 3: Validar captura de leads (HTMX + backend)

### Requisitos
- Backend corriendo: `cargo run -p api_server` (puerto 8080)
- Frontend corriendo: `bun run dev` (puerto 4321)

### Flujo de prueba
1.  Abre la landing en el navegador
2.  Completa el formulario (email válido) y envía
3.  Verifica que el formulario se reemplaza con el mensaje de éxito
4.  Revisa en el backend que el lead se haya persistido (tabla `leads`)

### Verificar rate-limit
1.  Envía el mismo email 6 veces consecutivas
2.  El sexto intento debe devolver `429 Too Many Requests` y mostrar mensaje de límite en la UI

---

## ✅ Nivel 4: Validar i18n / contenido dinámico

### 1. Usar header `Accept-Language`
- En el navegador, forzar `Accept-Language: en` (o usar una extensión de devtools)
- Recargar la página y verificar que el texto cambia a Inglés

### 2. Usar query param `?lang=en`
- Carga: `http://localhost:4321/?lang=en`
- Verificar que el copy en el Hero/CTA/Features cambia al inglés

---

## 📊 Nivel 5: Validar analytics y consentimiento

### 1. Consent banner
- En la landing, el banner debe aparecer (si no se ha aceptado cookies)
- Al aceptar, se debe guardar `landing_cookie_consent=1` en `localStorage` y recargar la página

### 2. GA / Tracking
- Si hay `PUBLIC_GA_TRACKING_ID` configurado, el script de `gtag.js` debe cargarse solo después de aceptar

---

## 🧪 Nivel 6: Pruebas de UI / Responsividad

### Validaciones rápidas
- Asegúrate de que la landing se ve bien en móvil y desktop
- Verifica que los botones de CTA redirigen correctamente (`/login`, `#leads`)
- Verifica que el formulario mantenga el estilo `bg-surface/80` + `backdrop-blur`

---

## � Nivel 7: Pruebas E2E (Playwright)

### Setup
```bash
cd apps/frontend_astro
bun install
npm install
npx playwright install
```

### Ejecutar tests
```bash
cd apps/frontend_astro
bun run dev
# en otra terminal:
npm run test:e2e
```

### Qué valida la suite
- La landing carga y muestra los textos principales.
- El formulario de leads envía correctamente (HTMX) y el mensaje de éxito aparece.

---

## 🧭 Consejos de ejecución rápida
```bash
# Servir frontend + backend en paralelo (PowerShell)
cd apps/frontend_astro; bun run dev
cd ../../crates/api_server; cargo run
```

> Nota: El cacheo de assets y HTML se gestiona en el deploy (Caddyfile) mediante headers `Cache-Control` para facilitar el caching en CDN/proxy.
