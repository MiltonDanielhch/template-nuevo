# 🧪 Manual de Pruebas - Frontend 3026

**Objetivo:** Guía maestra para validar cada capa del frontend, desde el build hasta la interacción con HTMX.

---

## 🏗️ Nivel 1: Pruebas de Build y Tipado

Antes de ejecutar la aplicación, el código debe compilar sin errores.

### Comando Maestro
```bash
cd apps/frontend_astro
bun run build
```

**Qué verifica:**
1.  **Build SSR:** Que Astro compile correctamente las páginas server-side.
2.  **TypeScript:** Que no haya errores de tipos en TypeScript.
3.  **Tailwind v4:** Que los estilos se procesen correctamente.
4.  **Integraciones:** Que Alpine.js y React se configuren sin errores.
5.  **Shadcn/ui:** Que los componentes React se bundeleen correctamente.

**Resultado Esperado:**
```text
✓ Completed in 10-15s
output: "server"
adapter: @astrojs/node
dist/client/*.js (bundled)
dist/server/*.server entry)
```

### Verificar Tipado TypeScript
```bash
cd apps/frontend_astro
bun run astro check
```

---

## 🎨 Nivel 2: Pruebas de Estilos y Theme

Validación visual del sistema de estilos.

### Verificar Tailwind v4 + Shadcn
```bash
cd apps/frontend_astro
# El build ya incluye esto
ls dist/client/*.css
```

### Verificar Theme 3026
1.  Abre el navegador en `http://localhost:4321`
2.  Abre DevTools (F12)
3.  Ejecuta en consola:
```javascript
// Verificar que las variables CSS están definidas
getComputedStyle(document.documentElement).getPropertyValue('--color-primary-500')
// Debe mostrar: #0ea5e9

// Verificar que Alpine.js está cargado
Alpine.store('theme')
// Debe mostrar el objeto del store

// Verificar que React/shadcn está cargado
document.querySelector('[data-slot="button"]')
// Debe mostrar el botón de shadcn
```

---

## 🎯 Nivel 3: Pruebas de shadcn/ui

Validación de componentes React en Astro.

### Verificar Componentes shadcn
```bash
# Listar componentes instalados
ls apps/frontend_astro/src/components/ui/
# Debe mostrar: button.tsx, input.tsx, card.tsx, label.tsx
```

### Usar componente en Astro
Los componentes shadcn se usan con `client:load` o `client:visible`:
```astro
---
import { Button } from "@/components/ui/button"
---
<Button client:load>Click me</Button>
```

### Agregar nuevo componente
```bash
cd apps/frontend_astro
bunx shadcn@latest add [component-name] -y
```

---

## 🔄 Nivel 4: Pruebas de Integración HTMX

Validación del flujo HTMX con el backend.

**Requisitos:**
- Backend corriendo: `cargo run -p api_server` (puerto 8080)
- Frontend corriendo: `bun run dev` (puerto 4321)

### 🧪 Flujo de Autenticación HTMX

**1. Verificar que HTMX está cargado**
```javascript
// En consola del navegador
htmx.version
// Debe mostrar: "2.0.8"
```

**2. Registro de Usuario via HTMX**
```bash
curl -v -X POST http://localhost:8080/register \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"test@lab3026.com\",\"password\":\"SecurePass123!\"}"
```

**3. Login via HTMX (desde el navegador)**
1.  Ve a `http://localhost:4321/login`
2.  Rellena el formulario
3.  Presiona el botón de login
4.  Observa en Network que la petición usa `hx-post`
5.  Verifica que la respuesta es un fragmento HTML (no JSON)

**4. Verificar indicadores de carga**
- Al enviar el formulario, debe aparecer un spinner
- En consola: `htmx:beforeRequest` y `htmx:afterRequest` events

### Verificar OOB (Out of Band) Swaps
```javascript
// En consola del navegador
htmx.on('htmx:afterSwap', (e) => {
  console.log('Elemento actualizado:', e.detail.target.id);
});
```

---

## 🧪 Nivel 5: Pruebas de Alpine.js

Validación de estado efímero.

### Theme Toggle
```javascript
// En consola
Alpine.store('theme').dark
// false por defecto

Alpine.store('theme').toggle()
// Cambia a true

// Verificar localStorage
localStorage.getItem('theme')
// Debe mostrar: "dark"
### Auth Store
```javascript
// Simular login
Alpine.store('auth').setAuth('test-token-123', { email: 'test@lab3026.com' })

// Verificar persistencia
Alpine.store('auth').token
```

---

## 🛡️ Nivel 6: Pruebas de Middleware y SSR

Validación de protección de rutas y gestión de sesiones del lado servidor.

### 🧪 Flujo de Protección de Rutas

**1. Acceso sin sesión**
1.  Abre el navegador en `http://localhost:4321/dashboard`
2.  Verifica que el middleware te redirige automáticamente a `/login`
3.  Observa en la consola del navegador que no hay cookie `auth_token`

**2. Acceso con sesión (Simulado)**
1.  En la consola del navegador, ejecuta:
```javascript
document.cookie = "auth_token=test_session_token; path=/";
```
2.  Recarga `http://localhost:4321/dashboard`
3.  Verifica que ahora puedes ver el Dashboard

**3. Logout (Cierre de sesión)**
1.  Haz clic en el botón de cerrar sesión en el Dashboard
2.  Verifica que eres redirigido a `/login`
3.  Verifica que la cookie `auth_token` ha sido eliminada de `Application > Cookies`

**4. Redirección de Auth**
1.  Con la cookie `auth_token` activa, intenta entrar a `/login`
2.  Verifica que el middleware te redirige de vuelta al `/dashboard`

---

## 📡 Nivel 7: Pruebas de Monitoreo (Latency)

Validación de salud del sistema.

### Verificar Latencia en Dashboard
1.  Abre el Dashboard
2.  Observa el card "API Latency"
3.  Verifica que el valor se actualiza cada 5 segundos (polling)
4.  En Network, verifica las peticiones a `/api/health`"test-token-123"

// Limpiar
Alpine.store('auth').clearAuth()
// token debe ser null
```

---

## 🌐 Nivel 6: Pruebas de Responsive y Accesibilidad

### Verificar Meta Tags
```bash
# En el HTML generado
curl -s http://localhost:4321/ | grep -E '<meta|<title'
```

### Verificar Lighthouse (opcional)
```bash
# Instalar si no tienes
npm install -g lighthouse

lighthouse http://localhost:4321 --view
```

**Métricas objetivo:**
- Performance: > 90
- Accessibility: > 90
- Best Practices: > 90
- SEO: > 90

---

## 🔧 Nivel 7: Comandos de Diagnóstico (Forensics)

### Ver errores en tiempo real
```bash
cd apps/frontend_astro
bun run dev
# Observa los logs de Vite en la terminal
```

### Ver estructura del build
```bash
ls -la apps/frontend_astro/dist/
```

### Verificar archivos estáticos
```bash
ls -la apps/frontend_astro/dist/client/
```

---

## 📊 Nivel 8: Matriz de Errores Comunes

| Error | Causa Probable | Solución |
|-------|----------------|----------|
| **Build fail** | Dependencias faltantes | `bun install` |
| **HTMX no funciona** | Script no cargado | Verificar CDN en MainLayout |
| **Alpine no funciona** | Error en `alpine.ts` | Revisar sintaxis TypeScript |
| **Shadcn/ui no funciona** | Falta `client:load` | Agregar directiva de hydatation |
| **Estilos no aplican** | Tailwind no procesado | Verificar `postcss.config.cjs` |
| **CORS errors** | Backend no acepta petitions | Configurar CORS en Axum |
| **React hydration error** | Mismatch de estado | Verificar que `x-data` es consistente |

---

## 🚀 Nivel 9: Scripts de Prueba Rápidos

### Prueba completa de la fundación
```bash
cd apps/frontend_astro
bun run build && echo "✅ Build exitoso"
```

### Agregar componente shadcn
```bash
cd apps/frontend_astro
bunx shadcn@latest add [component] -y
```

### Desarrollo con hot reload
```bash
cd apps/frontend_astro
bun run dev
# Accede a http://localhost:4321
```

### Verificar todas las dependencias
```bash
cd apps/frontend_astro
bun pm ls
```
