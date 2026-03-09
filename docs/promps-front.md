🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase 3 - Integración Backend + Dashboard)
Actúa como Ingeniero de Software Senior y Arquitecto Jefe. La autenticación HTMX está completa. Ahora implementaremos la integración con el backend y el dashboard.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ Esquemas ArkType (login/register)
- ✅ Entidades (User, Session, AuthResponse)
- ✅ Cliente API (auth-client.ts)
- ✅ HTMX Bridge (hx-bridge.ts)
- ✅ Componentes UI (Button, Input, Card)
- ✅ Página Login HTMX
- ✅ Página Register HTMX

🧭 2. PRÓXIMO OBJETIVO: INTEGRACIÓN BACKEND + DASHBOARD

Vamos a crear:
1. **Middleware de Auth** - ✅ Validar sessiones en SSR
2. **Dashboard Page** - ✅ Página principal tras login con Alpine.js
3. **Logout** - ✅ Cerrar sesión y limpiar cookies
4. **Navbar/Sidebar** - ✅ Navegación y componentes compartidos
5. **Command Palette** - ✅ Buscador global con `Ctrl+K`
6. **Health Monitor** - ✅ Indicador de latencia en tiempo real
7. **Auth Check Fix** - ✅ Eliminado 404 en `/api/auth/me`

---

🧭 3. SOBERANÍA DE DATOS (CRUD COMPLETE)
- ✅ **Partial Fragments**: Definidos fragmentos HTML para HTMX.
- ✅ **User Repository (Frontend)**: Adaptador para el backend Rust con soporte de `username`.
- ✅ **Admin Users Page**: Tabla interactiva con HTMX (Search/Delete/Edit).
- ✅ **Modales con Alpine**: Creación y edición de usuarios sin recarga.
- ✅ **HTMX Robustness**: Fix de envío (POST), `.prevent` y `htmx.process` en modales.
- ✅ **UI Fallbacks**: Estados y roles con valores por defecto para consistencia visual.

---

🧭 4. GESTIÓN DE PERFIL Y ROLES (COMPLETE)
- ✅ **Settings Page**: Página `/settings` para que el usuario gestione su perfil.
- ✅ **Dynamic Roles**: El admin panel carga roles reales desde `/api/roles`.
- ✅ **Navigation Fixes**: Sidebar y Command Palette sincronizados con las nuevas rutas.

---

🛠️ 5. STACK Y REGLAS (Fase 5)
- Integración: HTMX OOB Swaps para actualizaciones parciales.
- UX: Feedback instantáneo con Alpine e indicadores de carga.
- Reglas: Protocolo 3026, código limpio y modular.

---

🚀 ACCIÓN: Inicia la implementación del CRUD de Usuarios.
