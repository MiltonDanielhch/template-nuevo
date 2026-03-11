# 📱 Documentación de Implementación - Capacitor

## Visión General

Capacitor permite convertir la aplicación web Astro en aplicaciones nativas para Android e iOS, manteniendo la misma base de código.

---

## Arquitectura

```
┌─────────────────────────────────────────────┐
│              Frontend (Astro)               │
│         + HTMX + Tailwind CSS               │
│                                             │
│  ┌───────────────────────────────────────┐  │
│  │  Capacitor Plugins                    │  │
│  │  - Preferences (Auth Storage)         │  │
│  │  - Haptics (Vibración)              │  │
│  │  - Camera (Futuro)                   │  │
│  │  - Filesystem (Futuro)               │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────┐
│           Backend (Rust/Axum)               │
│              Puerto 8080                    │
└─────────────────────────────────────────────┘
```

---

## Instalación Realizada

### 1. Dependencias NPM
```bash
cd apps/frontend_astro
npm install @capacitor/core @capacitor/cli @capacitor/android
npm install @capacitor/preferences
```

### 2. Inicialización
```bash
npx cap init "Laboratorio 3026" "com.lab3026.app" --web-dir dist
npx cap add android
```

### 3. Configuración (capacitor.config.ts)
```typescript
import type { CapacitorConfig } from "@capacitor/cli";

const config: CapacitorConfig = {
  appId: "com.lab3026.app",
  appName: "Laboratorio 3026",
  webDir: "dist",
  server: {
    url: "http://localhost:4321",
    cleartext: true,
  },
};

export default config;
```

---

## Archivos Clave Modificados

### MainLayout.astro
- Viewport sin zoom: `user-scalable=no`
- Theme color: `#0ea5e9`
- Manifest PWA: `/manifest.json`
- Script de Capacitor Storage para Auth

### public/manifest.json
```json
{
  "name": "Laboratorio 3026",
  "short_name": "Lab3026",
  "display": "standalone",
  "theme_color": "#0ea5e9",
  "orientation": "portrait"
}
```

### global.css
- Touch targets mínimos de 44px para móvil
- Estilos responsive

---

## Comandos de Uso

### Desarrollo
```bash
# Iniciar servidor Astro
cd apps/frontend_astro
bun run dev --host

# Sincronizar cambios a Android
npx cap sync android

# Ejecutar en dispositivo/emulador
npx cap run android
```

### Producción
```bash
# Build de Astro
cd apps/frontend_astro
bun run build

# Sync a Android
npx cap sync android

# Abrir en Android Studio
npx cap open android
```

---

## Autenticación en Móvil

El script en MainLayout maneja el token de autenticación:

```javascript
window.CapacitorStorage = {
  async getToken() {
    if (window.Capacitor && window.Capacitor.isNativePlatform()) {
      const { Preferences } = window.Capacitor.Plugins;
      const result = await Preferences.get({ key: 'auth_token' });
      return result.value;
    }
    return localStorage.getItem('auth_token');
  },
  // ... setToken, removeToken
};
```

---

## Próximos Pasos

1. **Service Worker**: Implementar caché offline
2. **Camera**: Agregar `@capacitor/camera`
3. **Filesystem**: Agregar `@capacitor/filesystem`
4. **iOS**: Ejecutar `npx cap add ios` (requiere Mac)
5. **Probar**: Conectar dispositivo y ejecutar `npx cap run android`
