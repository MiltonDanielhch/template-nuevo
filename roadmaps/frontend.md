# 🎨 ROADMAP: FRONTEND 3026 (Astro + HTMX + Tailwind v4)

**Stack:** Astro 5.0 (SSR) | HTMX | Alpine.js | Tailwind v4 | Nanostores | ArkType
**Arquitectura:** Sintonía Hexagonal (Espejo de Backend Rust)

---

### 🏗️ BLOQUE I: LA FUNDACIÓN (Infraestructura)
**Objetivo:** Configurar el chasis del sistema y el motor de interactividad ligera.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Astro SSR Setup | `astro.config.mjs`: Configurar `output: 'server'` con adaptador Node. | ✅ |
| 2 | Tailwind v4 Base | `src/styles/global.css`: Setup de `@theme` y variables de sintonía. | ✅ |
| 3 | Atomic UI (shadcn) | `presentation/components/ui/`: Componentes base adaptados para HTMX (`hx-`). | ✅ |
| 4 | Layouts Maestros | `presentation/layouts/`: `MainLayout.astro` (SEO) y `AppLayout.astro` (Dashboard). | ✅ |
| 5 | Theme Engine | `src/lib/alpine.ts`: Stores Alpine para theme y auth. | ✅ |
| 6 | Integración Alpine | `astro.config.mjs` + `src/lib/alpine.ts`: Alpine.js para estados efímeros. | ✅ |
| 7 | Middleware Base | `src/middleware.ts`: Configuración base para SSR. | ✅ |

---

### 📡 BLOQUE II: INTELIGENCIA (El Hexágono Interior)
**Objetivo:** Establecer el ADN (Protobuf) y la comunicación de fragmentos HTML.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | ADN & Fragments | `domain/entities/`: Tipos Protobuf y definición de Partial Frames para HTMX. | ✅ |
| 2 | Aduana ArkType | `domain/schemas/`: Esquemas de validación compartidos para Cliente/Servidor. | ✅ |
| 3 | API Adapter | `infrastructure/api/auth-client.ts`: Cliente para peticiones tradicionales y streaming. | ✅ |
| 4 | HTMX Bridge | `infrastructure/api/hx-bridge.ts`: Configuración de headers HTMX (HX-Request). | ✅ |
| 5 | Sincronía de Estado | `application/stores/`: Nanostores solo para sesión; HTMX para datos de vista. | ✅ |
| 6 | Middleware Auth | `src/middleware.ts`: Validación de JWT/Session en cada petición SSR. | ✅ |

---

### 🖥️ BLOQUE III: VISTAS (Casos de Uso & UX)
**Objetivo:** Implementar la lógica de negocio orquestada y reactiva.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Auth Flow HTMX | `presentation/pages/login.astro`, `register.astro`: Login/Register usando `hx-post` hacia Axum. | ✅ |
| 2 | Use Cases UI | `application/use-cases/`: Orquestación de comandos entre UI y Backend. | ✅ |
| 3 | Dashboard Reactivo | `presentation/pages/dashboard/`: Polling/OOB Swap con HTMX para métricas. | ✅ |
| 4 | Command Palette | `presentation/components/command/`: Buscador global con Alpine.js. | ✅ |
| 5 | Health Monitor | `presentation/components/shared/`: Pulso de latencia entre el VPS y el cliente. | ✅ |

---

### 📱 BLOQUE IV: PUENTES (Soberanía Multiplataforma)
**Objetivo:** Exportar la experiencia hacia entornos nativos.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Tauri Bridge | `src-tauri/`: Wrapper para escritorio con acceso a FS local. | ⏳ |
| 2 | PWA & SW | `public/`: Manifest y Service Worker para carga offline de assets. | ⏳ |

---

### 🛡️ BLOQUE V: GESTIÓN DE ACCESO (RBAC UI)
**Objetivo:** Sincronizar la interfaz con los roles de la Capa 6 del Backend.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Role Guard | `presentation/components/auth/Guard.astro`: Control de visibilidad del lado servidor. | ✅ |
| 2 | Admin Panel | `presentation/pages/admin/`: CRUD completo (Listar, Crear, Editar, Eliminar) con HTMX. | ✅ |

---

## 📊 Progreso General

```
BLOQUE I: ████████████ 100% (7/7 completados)
BLOQUE II: ████████████ 100% (6/6 completados)
BLOQUE III: ████████████ 100% (5/5 completados)
BLOQUE IV: ██░░░░░░░░░░ 0% (0/2 completados)
BLOQUE V: ████████████ 100% (2/2 completados)
```
