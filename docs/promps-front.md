🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase 2 - Autenticación HTMX)
Actúa como Ingeniero de Software Senior y Arquitecto Jefe. La fundación está completa. Ahora implementaremos el flujo de autenticación usando HTMX + Alpine.js.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ Astro SSR configurado con Node adapter
- ✅ Tailwind v4 con theme 3026
- ✅ Alpine.js con stores (theme, auth)
- ✅ MainLayout.astro con HTMX
- ✅ Build funcional (~3.7s)

🧭 2. PRÓXIMO OBJETIVO: FLUJO DE AUTENTICACIÓN HTMX

Vamos a crear:
1. **Página de Login** (`/login`) - Formulario HTMX
2. **Página de Register** (`/register`) - Formulario HTMX
3. **Cliente API** (`infrastructure/api/auth-client.ts`)
4. **Validación ArkType** (`domain/schemas/auth.ts`)
5. **Use Cases** (`application/use-cases/auth.ts`)

---

🛠️ 3. STACK Y REGLAS
- Interactividad: HTMX (hx-post, hx-target, hx-swap)
- Validación: ArkType (schemas compartidos)
- Estado: Nanostores + Alpine stores
- Estilos: Tailwind v4 + Componentes reutilizables
- Reglas: Archivos < 150 líneas, Protocolo 3026

---

🏗️ 4. ESTRUCTURA DE ARCHIVOS A CREAR

```
apps/frontend_astro/src/
├── domain/
│   ├── entities/
│   │   └── auth.ts          # Tipos (User, Session)
│   └── schemas/
│       └── auth.ts           # Validación ArkType
│
├── infrastructure/
│   └── api/
│       ├── auth-client.ts    # Cliente HTTP
│       └── htmx-bridge.ts    # Config HTMX
│
├── application/
│   ├── use-cases/
│   │   └── auth.ts           # Lógica de login/register
│   └── stores/
│       └── auth.ts           # Nanostores (si es necesario)
│
└── presentation/
    ├── pages/
    │   ├── login.astro       # Página login
    │   └── register.astro    # Página register
    └── components/
        └── ui/
            ├── Button.astro
            ├── Input.astro
            └── Card.astro
```

---

🎯 5. IMPLEMENTACIÓN REQUERIDA

### A) SCHEMAS DE VALIDACIÓN (ArkType)
En `domain/schemas/auth.ts`:
```typescript
import { type } from 'arktype';

export const loginSchema = type({
  email: "string.email",
  password: "string.min(8)"
});

export const registerSchema = type({
  email: "string.email",
  password: "string.min(8).regex(/[A-Z]/).regex(/[0-9]/)",
  confirmPassword: "string"
});
```

### B) CLIENTE API
En `infrastructure/api/auth-client.ts`:
- Funciones: `login(email, password)`, `register(email, password)`, `logout()`, `getCurrentUser()`
- Debe manejar el token JWT y guardarlo en cookie/localStorage

### C) PÁGINAS HTMX
En `presentation/pages/login.astro`:
- Usar `hx-post="/api/auth/login"`
- `hx-target="#login-form"` para mostrar errores
- `hx-swap="outerHTML"` para reemplazar el formulario tras éxito
- Validación cliente con ArkType antes de enviar

---

📋 6. INTEGRACIÓN CON BACKEND

El frontend debe comunicarse con:
- `POST /api/auth/login` - Recibe token JWT
- `POST /api/auth/register` - Crea usuario
- `GET /api/auth/me` - Obtiene usuario actual
- `POST /api/auth/logout` - Cierra sesión

---

✅ 7. VERIFICACIÓN FINAL

Después de implementar:
1. `bun run build` - Debe compilar sin errores
2. `bun run dev` - Servidor en puerto 4321
3. Probar login/register en el navegador
4. Verificar que HTMX hace las peticiones correctamente

---

🚀 ACCIÓN: Implementa el flujo de autenticación HTMX completo siguiendo la arquitectura hexagonal. Usa los comandos de prueba en `docs/testing-front.md` para verificar cada componente.
