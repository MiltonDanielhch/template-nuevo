# ✅ CHECKLIST: LANDING PAGE (Paso a paso)

Este checklist está diseñado para guiarte **paso a paso** desde el primer archivo hasta el despliegue, en un orden lógico y ejecutable.

---

## 1) Preparar la base (Infra + Layout)
1. Crea `presentation/layouts/LandingLayout.astro`.
   - Incluye `title`, `description`, meta OG/Twitter y favicon.
   - Añade props para `canonical`, `ogImage`, `ogType`, `noIndex`.
   - ✅ Ya creado y en uso (`LandingLayout.astro`).
2. Crea `src/pages/index.astro` usando `LandingLayout`.
   - Asegúrate de que el contenido principal esté en HTML semántico.
   - No dependas de JS para mostrar contenido básico.
   - ✅ Ya creado y usa markup semántico.
3. Crea o verifica existencia de los assets de marca:
   - `public/images/hero.png` (hero), logos, screenshots.
   - `public/fonts/` (tipografías custom si aplica).
   - ⏳ Opcional: en este momento la landing usa textos y colores de token, sin imágenes.
4. Añade variables de marca (colores, tokens) en `src/styles/brand.css` y agrégalas a la configuración de Tailwind si corresponde.
   - ✅ Se usan tokens en `src/styles/global.css` (colores, fondos, tipografías).

---

## 2) Contenido principal (Copy + Secciones)
1. Define el texto del Hero (titular, subtítulo, CTA principal).
2. Implementa secciones:
   - Problema (¿qué duele?)
   - Solución (¿qué aporta?)
   - Feature cards (3–5 beneficios clave)
   - Testimonios / logotipos / métricas rápidas
3. Añade footer con enlaces legales (privacidad, términos, contacto) y redes.

---

## 3) Captura de leads (Form + Backend)
1. Crea componente `presentation/components/landing/LeadForm.astro`.
   - Campos: email (requerido), nombre opcional, honeypot hidden.
   - Usar HTMX para submit (`hx-post`, `hx-swap`, `hx-include`).
   - ✅ Ya existe y se integra con `LandingLayout`.
2. Agrega validación ligera (Alpine/JS) para email y campos obligatorios.
   - ⏳ Básica: el form exige email, pero puede reforzarse con validación adicional.
3. Crea endpoint backend `POST /api/v1/landing/leads`:
   - Definir esquema `Lead` en dominio.
   - Crear migración `leads` en `infra_db/migrations`.
   - Implementar repositorio y caso de uso (core_logic) + handler en `api_server`.
   - ✅ Endpoint + persistencia ya están implementados y funcionan (`/api/v1/landing/leads`).   - ✅ Honeypot anti-spam implementado en el handler.4. Implementa feedback en UI (success/fail toast o swap de fragmento HTMX).
   - ✅ Ya hay feedback tipo toast mediante HTMX y respuesta JSON.

---

## 4) SEO + Performance
1. Configura `sitemap.xml` (ej: `src/routes/sitemap.xml.ts`).
   - ✅ Ya existe en `src/pages/sitemap.xml.ts`.
2. Añade `robots.txt` en `public/` si no existe.
   - ✅ Ya existe en `public/robots.txt`.
3. Asegura meta tags relevantes en `LandingLayout` (canonical, noindex si aplica).
4. Configura headers de cache apropiados (SSR: `Cache-Control`, assets versionados).
5. Añade tracking ligero (Matomo/GTAG) con consentimiento opcional.
   - ✅ Ya hay un banner de consentimiento + carga condicional de GTAG en el layout.

---

## 5) Estilos y fondo
1. Asegura que el `body` no use `bg-white` que taparía la gradiente de fondo.
   - ✅ Se ajustó `LandingLayout` y `MainLayout` para usar `bg-slate-50` (y `dark:bg-slate-950`).
   - ✅ Se agregó un overlay fijo (`fixed inset-0`) para que el fondo siempre sea visible incluso si el contenido es blanco.
2. Evita tarjetas totalmente opacas para que el fondo se note (usar `bg-surface/70` + `backdrop-blur`).
   - ✅ Ya aplicado en el form y en las cards de features.
3. Comprueba en dev server que no haya estilos cacheados y que el fondo se vea correctamente.
   - ✅ La página carga con el fondo visible tras limpiar cache.

---

## 6) QA / Pruebas / Despliegue
1. Crea tests E2E (Playwright/Cypress) que:
   - Validen que la landing carga.
   - Envíen el formulario y esperen respuesta de éxito.
2. Asegura que el despliegue actual (Caddy/Docker) sirva la landing correctamente.
3. Monitorea logs de backend para errores 5xx o fail de leads.

---

✅ Este checklist es más granular y orientado a ejecución concreta; complementa el roadmap (que es más estratégico y de alto nivel). Sigue estos pasos en orden para avanzar rápido.
