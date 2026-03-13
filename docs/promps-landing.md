🚀 PROMPT DE SINTONÍA: LANDING PAGE 3026 (Captura de Leads + SEO)
Construye la entrada pública que mostrará la propuesta de valor, capturará contactos y enlazará al ecosistema principal (dashboard + autenticación).

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ Astro 5 + SSR ya está configurado en el proyecto.
- ✅ Tailwind v4 + design tokens (theme) ya definidos para el resto de la app.
- ✅ Componentes UI base (Button, Input, Card, Modal) compatibles con HTMX.
- ✅ Sistema de rutas y layouts ya establecidos (MainLayout, AppLayout).
- ✅ Landing funcional creada (hero + features + formulario de leads + backend persistente).
- ✅ Ajustes visuales: fondo gradiente + overlay fijo + tarjetas semitransparentes para que el fondo no quede tapado.
- ✅ Recursos de branding: `og-image.svg` y `hero-illustration.svg` incluidos.
- ✅ Soporte básico de i18n (Accept-Language / `?lang=`) para copy dinámico.
- ✅ Backend ya persiste leads en SQLite y responde con feedback (HTMX toast).
- ✅ Desarrollo: correr `bun run dev` desde `apps/frontend_astro` (usa puerto 4322 si 4321 está ocupado).
- ✅ Backend: `cargo run` o `cargo watch` en `crates/api_server`.

---

🧭 2. PRÓXIMO OBJETIVO: LA LANDING COMO PUERTA DE ENTRADA
Vamos a crear una landing que sea:
1. **Rápida (SSR + optimizada)**
2. **SEO-friendly** (meta, OG, sitemap)
3. **Captura leads** (form dinámico + persistencia en backend)
4. **Consistente con la UX existente** (misma UI & tokens)

---

🧭 3. TAREAS CLAVE (TODO)
1. **Página principal**
   - Crear `src/pages/index.astro` usando `LandingLayout`.
   - Asegurar SSR y que no dependa de JS para el contenido principal.

2. **Layout + SEO**
   - Crear `presentation/layouts/LandingLayout.astro`.
   - Incluir `title`, `description`, meta OG, Twitter card y favicon.
   - Soportar props para `canonical`, `ogImage`, `ogType`.

3. **Contenido estructurado**
   - Hero (headline + subtítulo + CTA primario).
   - Sección de Problema + Solución (valor claro).
   - Feature cards con iconos SVG.
   - Prueba social (logos + testimonios) opcional.
   - Footer con política de privacidad, contacto y enlaces.

4. **Captura de leads (Lead magnet)**
   - Crear componente `presentation/components/landing/LeadForm.astro`.
   - Validación ligera con Alpine/HTMX (email requerido, nombre mínimo y honeypot).
   - Endpoint backend `POST /api/v1/landing/leads` que persista en tabla `leads`.
   - Use case + repositorio + migración `leads` en `infra_db`.
   - Protección anti-spam (honeypot + rate-limit por email).
   - Feedback en UI (success toast + error) usando HTMX swaps.

5. **SEO y Performance**
   - Generar `sitemap.xml` en `src/routes/sitemap.xml.ts` o estático.
   - Crear `robots.txt` si no existe.
   - Configurar cache headers para assets + HTML.
   - Añadir análisis ligero (ej: Matomo/GTAG) con banner de consentimiento (ya implementado en el layout).   - Soportar opt-out mediante endpoint (ej: `/api/consent/opt-out`).
6. **Medición y despliegue**
   - Pruebas E2E con Playwright/Cypress validando que el formulario envía.
   - Asegurar que la landing se sirve correctamente con el despliegue actual (Caddy/Docker).

---

🧭 4. ACCIÓN: ¿Qué hacer AHORA?
1. ✅ E2E (Playwright) para validar que la landing carga, el formulario envía y el rate-limit se aplica.
   - Tests implementados: carga de página, envío exitoso, email inválido, email duplicado, honeypot, rate-limit.
2. ✅ Headers Cache-Control SSR + assets implementados en middleware.
3. ✅ Monitoreo implementado:
   - Logging de leads exitosos y fallidos con tracing.
   - Middleware error_logging para capturar errores 5xx.
   - Auditoría de errores en tabla audit_logs.
4. Revisar el despliegue en Caddy/Docker para asegurar que `robots.txt` y `sitemap.xml` son servidos correctamente.

---

✅ Resultado esperado: una landing funcional con captura de leads que se vea moderna y que soporte despliegue sin dependencias de JS extra, con métricas básicas habilitadas.
