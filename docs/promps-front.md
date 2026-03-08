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

---

� ACCIÓN: FASE COMPLETADA. Ahora realizaremos el commit de los cambios.

---

🧭 3. SIGUIENTE PASO: COMMIT DE INTEGRACIÓN

```markdown
🚀 PROMPT DE COMMIT: INTEGRACIÓN BACKEND + DASHBOARD
Actúa como Ingeniero de Software Senior. La integración del dashboard y la autenticación SSR está lista. Realiza el commit siguiendo el protocolo.

---

�️ TAREAS REALIZADAS:
- Movidas páginas a `src/pages/` para habilitar routing de Astro.
- Implementado `middleware.ts` para validación de sesiones SSR.
- Creados endpoints de API Proxy (`/api/auth/login`, `/api/auth/logout`).
- Actualizado `dashboard.astro` con lógica de latencia y componentes UI.
- Implementado `/api/health` para monitoreo de salud.

---

� COMANDO DE COMMIT:
git add .
git commit -m "feat(frontend): implement auth middleware and dashboard integration"
```
