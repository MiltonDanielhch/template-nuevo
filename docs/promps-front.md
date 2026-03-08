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

---

🚀 ACCIÓN: FASE COMPLETADA. Ahora realizaremos el commit de los cambios.

---

🧭 3. SIGUIENTE OBJETIVO: SOBERANÍA DE DATOS (CRUD de Usuarios)

Vamos a implementar:
1. **Partial Fragments** - Definir fragmentos HTML para HTMX.
2. **User Repository (Frontend)** - Adaptador para el backend Rust.
3. **Admin Users Page** - Tabla interactiva con HTMX (Search/Delete/Edit).
4. **Modales con Alpine** - Creación y edición de usuarios sin recarga.

---

🛠️ 4. STACK Y REGLAS (Fase 4)
- Integración: HTMX OOB Swaps para actualizaciones parciales.
- UX: Feedback instantáneo con Alpine e indicadores de carga.
- Reglas: Protocolo 3026, código limpio y modular.

---

🚀 ACCIÓN: Inicia la implementación del CRUD de Usuarios.
