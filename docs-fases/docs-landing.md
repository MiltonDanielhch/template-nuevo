# 🚀 Documentación de Implementación - Landing Page

## Visión General

La Landing Page es el primer punto de contacto del producto: debe ser rápida, accesible y capaz de capturar leads con un flujo sencillo (formulario + persistencia). El objetivo es maximizar conversión sin sacrificar SEO ni rendimiento.

---

## Arquitectura

```
┌──────────────────────────────────────────────────┐
│                 Frontend (Astro)                 │
│          + HTMX + Tailwind + Alpine.js           │
│                                                  │
│  - Landing Layout (LandingLayout.astro)          │
│  - Página principal (src/pages/index.astro)      │
│  - Formulario de leads (LeadForm.astro)          │
│                                                  │
└──────────────────────────────────────────────────┘
                      │
                      ▼
┌──────────────────────────────────────────────────┐
│       Backend (Rust / Axum) - API server         │
│  - Endpoint: POST /api/v1/landing/leads          │
│  - Persistencia: SQLite / tabla `leads`          │
│  - Anti-spam: honeypot + rate-limit (5/h)        │
└──────────────────────────────────────────────────┘
```

---

## Implementación principal

### 1) Frontend (landing)

- **`src/pages/index.astro`**: entrada de la landing.
  - Usa `LandingLayout`.
  - Soporta i18n básico vía header `Accept-Language` o query `?lang=`.
  - Contiene hero, feature cards, sección de confianza y footer.

- **`presentation/layouts/LandingLayout.astro`**
  - Meta tags SEO + OG + Twitter.
  - Carga condicional de GTAG con consentimiento.
  - Fondeado en gradiente + overlays semi-transparentes.

- **`presentation/components/landing/LeadForm.astro`**
  - Formulario HTMX que envía a `/api/v1/landing/leads`.
  - Validación ligera en JS (email + nombre opcional).
  - Honeypot hidden para antispam.
  - Feedback visual con swap HTMX (mensaje de éxito/limite).

---

## Archivos clave modificados

- `apps/frontend_astro/src/pages/index.astro`
- `apps/frontend_astro/src/presentation/layouts/LandingLayout.astro`
- `apps/frontend_astro/src/presentation/components/landing/LeadForm.astro`
- `apps/frontend_astro/src/lib/i18n.ts` (traducciones / detección de idioma)
- `apps/frontend_astro/public/og-image.svg` (OG preview)
- `apps/frontend_astro/public/images/hero-illustration.svg` (hero visual)

---

## Backend / Persistencia

- **Endpoint**: `POST /api/v1/landing/leads`
- **Handler**: `crates/api_server/src/entry_points/api/v1/landing_handlers.rs`
  - Honeypot antispam
  - Rate-limit (5 envíos / hora por email)
  - Responde HTML para intercambiar con HTMX

- **Persistencia**: Tabla `leads` en SQLite (migraciones en `infra_db/migrations`).
- **Tests**: `crates/api_server/tests/integration_tests.rs` incluye test de rate-limit.

---

## Comandos útiles

```bash
# Desarrollo front
cd apps/frontend_astro
bun run dev

# Backend
cd crates/api_server
cargo run

# Build de producción
cd apps/frontend_astro
bun run build
```

---

## Próximos pasos (mejoras sugeridas)

1. **Pruebas E2E**: Añadir suite a Playwright o Cypress para validar el formulario y los flujos de conversión.
2. **Cacheo / headers**: Ajustar `Cache-Control` para la landing y el sitemap (SSR + assets).
3. **Mejora de conversión**: Añadir pruebas sociales reales, métricas dinámicas o testimonios.
4. **Automatización**: Agregar chequeos en CI (build + tests de backend + validación de sitemap).
