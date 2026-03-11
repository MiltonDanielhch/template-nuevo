🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase Móvil - Capacitor)
Actúa como Ingeniero App Mobile Senior especializado en WebView Architectures y Frontend Astro. Queremos extender el alcance del Laboratorio 3026 desde Tauri (Escritorio) a Android/iOS usando CapacitorJS.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ **Backend Fuerte**: Rest API en Rust / Axum port 8080.
- ✅ **Frontend Web**: Astro + HTMX + Tailwind V4. Ya es responsivo y modular.
- ✅ **Comandos Tauri Configurados**: La app desktop opera por su lado independiente.
- ✅ **Capacitor Instalado**: Proyecto Android generado y configurado.

🧭 2. ESTADO ACTUAL - LO YA IMPLEMENTADO

**BLOQUE I - PWA ✅ COMPLETO:**
1. ✅ Meta Viewport sin zoom (`user-scalable=no`)
2. ✅ Touch targets de 44px para móvil
3. ✅ Manifest.json PWA en `/public`
4. ✅ Iconos PWA (192x192, 512x512)

**BLOQUE II - Capacitor Core ✅ COMPLETO:**
1. ✅ `@capacitor/core`, `@capacitor/cli`, `@capacitor/android` instalados
2. ✅ `capacitor.config.ts` configurado apuntando a `dist`
3. ✅ Proyecto Android generado en `apps/frontend_astro/android/`
4. ✅ Build y sync funcionando

**BLOQUE III - Hardware 50%:**
1. ✅ Auth Storage con `@capacitor/preferences`
2. ✅ Script de inicialización en MainLayout

🧭 3. PRÓXIMOS PASOS PENDIENTES

1. **Service Worker (Opcional)**: Implementar caché offline con Workbox
2. **Cámara/QR**: Agregar `@capacitor/camera` para fotos de expedientes
3. **Archivos**: Agregar `@capacitor/filesystem` para PDFs
4. **iOS**: Agregar plataforma iOS con `npx cap add ios`

---

🛠️ 4. STACK Y REGLAS DE LA FASE MOBILE
- Regla de Oro: **Jamás tocar Rust para la UI**. Todo el front móvil ocurre en Astro.
- Herramienta de compilación híbrida: CapacitorJS versión 6+.
- No generar lógica compleja en JS (HTMX debe seguir haciendo el 95% del trabajo asíncrono hacia Rust).
- Proteger código nativo con `if (Capacitor.isNativePlatform())`

---

🚀 ACCIÓN: Continuar con las tareas pendientes del BLOQUE III y preparar para pruebas.
