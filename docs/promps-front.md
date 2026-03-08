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
1. **Middleware de Auth** - Validar JWT en SSR
2. **Dashboard Page** - Página principal tras login
3. **Logout** - Cerrar sesión
4. **Navbar/Sidebar** - Navegación

---

🛠️ 3. STACK Y REGLAS
- Integración: Astro SSR endpoints que proxy a Axum
- Estado: Alpine stores + Cookies HttpOnly
- Reglas: Archivos < 150 líneas, Protocolo 3026

---

🏗️ 4. ESTRUCTURA DE ARCHIVOS A CREAR

```
apps/frontend_astro/src/
├── presentation/
│   ├── pages/
│   │   ├── dashboard.astro    # Dashboard tras login
│   │   ├── logout.astro       # Cerrar sesión
│   │   └── api/
│   │       └── auth/
│   │           └── login.ts   # Endpoint API (opcional)
│   └── components/
│       └── shared/
│           ├── Navbar.astro
│           └── Sidebar.astro
```

---

📋 5. INTEGRACIÓN CON BACKEND

El frontend debe comunicarse con:
- Backend Axum en puerto 8080
- Usar cookies HttpOnly para el token JWT
- Validar sesión en cada request SSR

---

✅ 6. VERIFICACIÓN FINAL

Después de implementar:
1. `bun run dev` - Frontend en puerto 4321
2. `cargo run -p api_server` - Backend en puerto 8080
3. Probar login/register
4. Verificar redirect al dashboard

---

🚀 ACCIÓN: Implementa la integración backend y el dashboard. Consulta `docs/testing-front.md` para pruebas.
