# 🧪 Manual de Pruebas - Mobile Capacitor 3026

**Objetivo:** Guía maestra para validar la integración de la aplicación móvil PWA/Híbrida usando CapacitorJS, comandos útiles de sincronización y pruebas nativas en Android/iOS.

---

## 🏗️ Nivel 1: Pruebas de Entorno Mobile

Antes de interactuar con el celular o emulador, verifica que tu entorno Node y Capacitor están configurados y el CLI funciona correctamente.

### Comando de Verificación (Doctor)
```bash
cd apps/frontend_astro
npx cap doctor
```

**Qué verifica:**
1.  **Dependencias Core:** Que `@capacitor/core` y las plataformas (`@capacitor/android`, `@capacitor/ios`) estén instaladas y emparejadas en versión.
2.  **Android Studio SDK:** En Windows/Linux/Mac, que tengas las variables de entorno de Android correctamente puestas.
3.  **Xcode:** En Mac, la viabilidad de compilar para iOS.

**Resultado Esperado:**
```text
💊   Capacitor Doctor  💊

Latest Dependencies:
  @capacitor/cli: 6.X.X
  @capacitor/core: 6.X.X
  @capacitor/android: 6.X.X

Installed Dependencies:
  @capacitor/cli: 6.X.X
  @capacitor/core: 6.X.X
  @capacitor/android: 6.X.X

[success] Android looking great! 👌
```

---

## 🔧 Nivel 2: Desarrollo con Live Reload (Dev Mode)

Para probar la comunicación entre el front móvil (Astro) y tu backend local en tiempo de desarrollo **dentro de tu celular físico** o emulador.

### Comando de Servidor Local
1. Inicia Astro exportando tu red local (asegúrate de que tu celular está en el mismo WiFi):
```bash
cd apps/frontend_astro
bun run dev --host
```
*Toma nota de la IP (ej. `http://192.168.1.50:4321`)*

### Configurar Live Reload Móvil
Edita temporalmente `capacitor.config.ts` para que Capacitor no lea `/dist` sino el servidor en vivo:
```typescript
import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'bo.lab3026.mobile',
  appName: 'Lab3026',
  webDir: 'dist',
  server: {
    url: 'http://192.168.1.50:4321', // Tu IP local
    cleartext: true
  }
};
export default config;
```

### Ejecutar en Teléfono
Conecta tu celular por USB, habilita depuración USB y ejecuta:
```bash
npx cap run android
```
Cualquier cambio que guardes en VS Code (Astro) se actualizará instantáneamente en la pantalla de tu teléfono físico.

---

## 📦 Nivel 3: Pruebas de Sincronización (Build PWA)

Para empaquetar la aplicación y generar el código fuente de las Apps móviles (Android Studio / Xcode).

### Flujo de Sincronización (Sync)
Este es el comando más importante. Transforma tu web en binarios.
```bash
cd apps/frontend_astro
bun run build
npx cap sync
```

**Qué verifica:**
1.  **Build Astro:** Renderiza todo el HTML, JS y CSS en `dist`.
2.  **Capacitor Sync:** Copia la carpeta `/dist` intacta a `android/app/src/main/assets/public`. Copia además cualquier plugin nativo nuevo (cámara, push) a los archivos Gradles/Pods.

### Compilar a Release (.APK / .AAB)
```bash
npx cap open android
```
**Dentro de Android Studio (IDE visual):**
1.  Espera que termine el "Gradle Sync" (barra inferior cargando).
2.  Ve a `Build` > `Build Bundle(s) / APK(s)` > `Build APK(s)` para generar un instalador de prueba.
3.  Ve a `Build` > `Generate Signed Bundle / APK...` para la versión final de la Play Store.

---

## 🚀 Nivel 4: Pruebas del Puente Nativo (Plugins de Hardware)

Verificar que tu página web de Astro sea capaz de acceder a los sensores del celular.

### 1. Test de Plataforma (Degradación Graciosa)
Si tienes botones que dicen "Descargar a escritorio", usa esto en tu JS (Alpine o React) para ocultarlos en móvil:
```javascript
import { Capacitor } from '@capacitor/core';
if (Capacitor.isNativePlatform()) {
  console.log("Estoy en Android o iOS natively!");
  // Ocultar botones de Tauri o Escritorio
}
```

### 2. Vibración Táctica (Haptics Test)
Verificar que HTMX puede disparar eventos nativos:
```html
<button 
  hx-post="/api/login"
  onclick="window.testVibrate()"
  class="...">
    Verificar Haptics
</button>
```
```javascript
// window.testVibrate
import { Haptics, ImpactStyle } from '@capacitor/haptics';
Haptics.impact({ style: ImpactStyle.Heavy });
```

### 3. Depuración Chrome Inspection
Aunque la app esté instalada en el celular (`.apk`), **puedes ver el F12 (Consola)** desde tu PC.
1. Conecta el celular por USB con la App abierta.
2. Abre Google Chrome en tu PC y navega a: `chrome://inspect/#devices`
3. Dale a "Inspect" bajo el nombre de tu aplicación. 
4. Tendrás acceso total a la consola, network de HTMX y elementos DOM.

---

## 🛡️ Nivel 5: Troubleshooting Rápido

### "Hice cambios en Astro pero no aparecen en el celular"
**Error**: Se te olvidó sincronizar.
**Solución**: Ejecuta siempre `bun run build && npx cap sync` antes de compilar en Android Studio.

### "Error EACCES / Permisos Denegados en Mac"
**Error**: El CLI necesita perimsos sobre las carpetas de compilación.
**Solución**: `sudo npx cap sync ios`

### "Mis peticiones a mi API en Rust no llegan"
**Error**: En la app compilada, la ruta "relativa" `hx-post="/login"` asume `http://localhost`, lo cual apunta al propio celular, no a tu servidor Rust real.
**Solución**: Configura una variable de entorno `PUBLIC_API_URL` en Astro. En producción móvil, tus peticiones deben ser a la IP de producción, ej. `hx-post="https://api.lab3026.com/login"`.
