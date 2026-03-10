# 📱 RoadMap: Laboratorio 3026 - Fase Móvil (Capacitor & PWA)

Este documento traza la ruta arquitectónica para trasladar nuestra aplicación web (Astro + HTMX) al mundo móvil (Android e iOS) usando la estrategia de esfuerzo mínimo y máxima compatibilidad: **CapacitorJS**.

---

## 🗺️ Mapa de Fases (Mobile Strategy)

### BLOQUE I: Adaptación PWA y Responsividad
Preparar el terreno web para que se sienta como una app nativa en el celular.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Meta Viewport | Asegurar `<meta name="viewport">` sin zoom manual (`user-scalable=no`). | ✅ |
| 2 | Tailwind Mobile | Ajustar padding, botones y modales para pantallas táctiles (Touch targets > 44px). | ✅ |
| 3 | Web Manifiest | Crear `manifest.json` e íconos PWA (`pwa-192x192.png`, etc.) en `/public`. | ✅ |
| 4 | Service Worker | Opcional: Caché offline básica usando Workbox o script manual en Astro. | ⏭️ |

---

### BLOQUE II: El Chasis Móvil (Capacitor Core)
Inyectar el envoltorio nativo al proyecto Astro.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Instalar Capacitor | Instalar `@capacitor/core` y `@capacitor/cli` en `apps/frontend_astro`. | ✅ |
| 2 | Inicialización | Ejecutar `npx cap init` y crear `capacitor.config.ts` apuntando a `dist`. | ✅ |
| 3 | Agregar Plataformas | Instalar `@capacitor/android` e inicializar el directorio `/android`. | ✅ |
| 4 | Build & Sync | Compilar Astro (`bun run build`) y sincronizar a Android con `npx cap sync`.| ✅ |

---

### BLOQUE III: Funciones de Hardware (El Puente Mobile)
Interactuar con los sensores y almacenamiento nativo del teléfono.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Auth Storage | Usar `@capacitor/preferences` para guardar el JWT (Reemplaza a Tauri Store). | ✅ |
| 2 | Haptics & Toast | Agregar vibración a botones clave usando `@capacitor/haptics`. | ✅ |
| 3 | Cámara / QR | (Opcional) Instalar `@capacitor/camera` para tomar fotos de expedientes. | ⏳ |
| 4 | Archivos (Export) | Usar `@capacitor/filesystem` para descargar reportes PDF en el celular. | ⏳ |

---

## 📊 Progreso Global de Mobile

```text
Actualice rellenando (█) por cada subtarea completada.

BLOQUE I:  ████████████████ 100% (4/4 completados)
BLOQUE II: ████████████████ 100% (4/4 completados)
BLOQUE III:░░░░░░░░░░░░░░░░ 50% (2/4 completados)
```

> **Leyenda de Estado:**
> ⏳ = Pendiente
> 🚧 = En Progreso
> ✅ = Completado

---

## 📜 Reglas del Código 3026 - Mobile Edition
1. **Single Source of Truth:** La lógica de negocio SIEMPRE vive en Rust (Axum, 8080).
2. **Reutilización:** Usa HTMX siempre que sea posible. Minimiza escribir JS específico para móvil.
3. **Graceful Degradation:** Protege el código nativo con un `if (Capacitor.isNativePlatform())` para que no rompa la web ni Tauri.
