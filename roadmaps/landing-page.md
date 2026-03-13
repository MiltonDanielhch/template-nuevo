# 🌐 ROADMAP: LANDING PAGE 3026 (Marketing + Captura de Leads)

**Stack:** Astro 5.0 (SSR) | Tailwind v4 | HTMX | SEO / Open Graph | Analytics | Forms

---

## 🎯 Objetivo General
Construir una landing page moderna, accesible y de alto rendimiento que refleje la propuesta de valor del producto, capture leads y sirva como puerta de entrada al ecosistema (Front+Backend). Debe ser fácilmente mantenible y ampliable, respetando las mismas convenciones de arquitectura y estilo que el resto del proyecto.

---

## 🧱 BLOQUE I: FUNDACIÓN (Estructura & Estilos)
**Objetivo:** Establecer la base de la landing para que cualquier contenido o experimento pueda desplegarse sin fricciones.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Página base | `src/pages/index.astro` como entrypoint principal. | ✅ |
| 2 | Layout dedicado | `presentation/layouts/LandingLayout.astro` con SEO, OG y favicon. | ✅ |
| 3 | Diseño responsivo | Tailwind + utilidades para grid, tipografía y espaciado. | ✅ |
| 4 | Variables de marca | `src/styles/global.css` (colores, sombras, tipografías, tokens). | ✅ |
| 5 | Tipografías y assets | `public/` (fuentes desde @fontsource + imágenes en `public/images/`). | ✅ |
| 6 | Accesibilidad básica | Semántica HTML, contraste WCAG, `aria-*` y orden tab. | ✅ |

---

## 🎨 BLOQUE II: CONTENIDO & COPY
**Objetivo:** Definir los mensajes clave, secciones y elementos de conversión (CTAs) para guiar al visitante.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Estructura de secciones | Hero, Problema, Solución, Características, Testimonios, CTA, Footer. | ✅ |
| 2 | Hero & CTA principal | Titular + subtítulo + botón ("Comenzar", "Ver demo", "Regístrate"). | ✅ |
| 3 | Feature cards | Listado de beneficios con iconografía (SVG) + microinteracciones. | ✅ |
| 4 | Prueba social / confianza | Logos, quotes, métricas o casos de uso. | ✅ |
| 5 | Footer con enlaces | Políticas, contacto, redes y datos legales. | ✅ |
| 6 | Contenidos dinámicos | Soporte para i18n / traducción (basado en `Accept-Language` o query param `?lang=`). | ✅ |

---

## 🔌 BLOQUE III: CAPTURA DE LEADS & FORMS
**Objetivo:** Implementar los mecanismos para convertir visitas en leads o usuarios, con feedback inmediato y seguridad.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Formulario de contacto / suscripción | `presentation/components/landing/LeadForm.astro` con HTMX. | ✅ |
| 2 | Validación client-side | Alpine + HTMX para validar email y campos obligatorios. | ✅ |
| 3 | Endpoint de backend | `api_server/entry_points/api/v1/landing.rs` (POST /leads). | ✅ |
| 4 | Persistencia de leads | Nueva tabla `leads` (SQLite) + repositorio + use case. | ✅ |
| 5 | Protección anti-spam | Honeypot + rate-limit y reCAPTCHA-like (simple token). | ✅ |
| 6 | Analytics / consent banner | Banner simple + carga condicional de GTAG. | ✅ |
| 6 | Confirmación & UI | Modal/alerta con feedback de éxito/error usando HTMX swap. | ✅ |

---

## 🚀 BLOQUE IV: SEO, PERFORMANCE & ANALYTICS
**Objetivo:** Garantizar la visibilidad orgánica y medir el impacto con mínimo overhead.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Meta tags SEO | `LandingLayout.astro` + `src/routes/rss.xml` (si aplica). | ✅ |
| 2 | OG + Twitter Cards | Open Graph + Twitter meta tags para sharing. | ✅ |
| 3 | Sitemap | `src/routes/sitemap.xml.ts` (o estático). | ✅ |
| 4 | Performance antes/después | Lighthouse / Pagespeed en CI. | ⏳ |
| 5 | Analytics ligera | Event tracking (Matomo/Pinia/GTAG) + consent banner opcional. | ✅ |
| 6 | Opt-out | Endpoint para resetear consentimiento de cookies. | ✅ |
| 6 | Caching & CDN | `public/` con assets versionados y `Cache-Control` SSR. | ✅ |

---

## 🧪 BLOQUE V: QA Y DESPLIEGUE
**Objetivo:** Establecer un flujo reproducible de prueba, pasada de revisión y despliegue.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Storybook / Playground | Opcional: prototipar componentes de landing (si aplica). | ⏳ |
| 2 | Tests de UI | Cypress / Playwright para validar flujo de conversión. | ✅ |
| 3 | Tests de integración | `api_server/tests` para endpoint de leads. | ✅ |
| 4 | Despliegue | Ajustes en `deploy/` (caddy, docker) para servir la landing. | ⏳ |
| 5 | Monitoreo | Logs + alertas (backend) para leads y errores 5xx. | ✅ |

---

## ✅ Progreso General

```
BLOQUE I: ██████████ 100% (6/6 completados)
BLOQUE II: ██████████ 100% (6/6 completados)
BLOQUE III: ██████████ 100% (6/6 completados)
BLOQUE IV: ██████████ 100% (6/6 completados)
BLOQUE V: ████░░░░░░░ 60% (3/5 completados)
```

---

## 🧭 Notas rápidas
- Apunta a que la landing sea 100% estática (SSR) para SEO y rendimiento.
- Usa la misma base de tokens de Tailwind y componentes UI que en el dashboard.
- Mantén la lógica de negocio mínima en el frontend; delega validaciones pesadas al backend.
- Diseña la landing como un “puente” hacia la app principal o el flujo de autenticación.
