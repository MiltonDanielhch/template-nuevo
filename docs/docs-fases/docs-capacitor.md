## Objetivo Mobile (Fase 10)
Herramienta de aprendizaje para entender cómo estamos llevando "Laboratorio 3026" a los bolsillos de los doctores mediante **CapacitorJS**. En esta fase aprenderás con el Método Feynman cómo funciona este puente móvil.

> "Actúa como un Arquitecto Mobile Senior explicando CapacitorJS con la metodología Feynman.

## Los 5 Niveles del Método 3026

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** | Definición Técnica sin rodeos |
| **2** | **¿Para qué sirve?** | El problema real que resuelve. |
| **3** | **¿Cómo funciona?** | La estructura y mecánica interna (Analogías) |
| **4** | **Ejemplo Práctico 3026** | Comando de código y como llamarlo. |
| **5** | **¿Por qué es vital para el sistema?** | Costo, Velocidad y Multiplataforma. |

---

## 📱 BLOQUE X: MÓVIL (Capacitor & Web-To-Native)

### 🧠 Integración: Fase 10.1 - Capacitor Core (El Envoltorio)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** | Capacitor es un entorno de ejecución (runtime) web nativo multiplataforma. Toma nuestro sitio generado por Astro y lo mete dentro de una Web View de iOS y Android. |
| **2** | **¿Para qué sirve?** | Sirve para no tener que aprender Kotlin (Android) ni Swift (iOS). Resuelve el desastre de tener 3 equipos de desarrollo distintos, condensando todo en un único equipo Web (Astro/HTMX). |
| **3** | **¿Cómo funciona?** | 1. Astro hace `build` y genera el HTML/JS en `/dist`. <br> 2. `capacitor.config.ts` lee esa carpeta. <br> 3. Al hacer `cap sync`, copia el HTML estático a las carpetas nativas de Android Studio. <br> 4. El celular usa Chrome (Android) o Safari (iOS) en pantalla completa, sin barras, aparentando ser una app instalable `.apk`. |
| **4** | **Ejemplo Práctico 3026** | **Inicializar:** <br> ```bash npx cap init "Lab3026" "bo.lab3026.mobile" ``` <br> Esto crea el archivo maestro de configuración del que depende todo el flujo móvil. |
| **5** | **¿Por qué es vital para nuestro sistema?** | Cero costo de migración: 100% del frontend actual funciona y está listo. Nos permite entregar resultados hoy mismo a los doctores sin retrasar meses la arquitectura. |

---

### 🧠 Integración: Fase 10.2 - El Puente Nativo (Plugins de Capacitor)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** | Un puente Javascript-To-Java/Swift. Usando librerías oficiales (plugins) inyectadas en tu HTML, el navegador le habla al sistema operativo del celular. |
| **2** | **¿Para qué sirve?** | Sin estos plugins tu app sería sólo una web visual. Sirven para hacer vibrar el móvil (Haptics), activar el Flash, detectar GPS o recibir Push Notifications. |
| **3** | **¿Cómo funciona?** | Astro ejecuta una función asíncrona de un plugin (`@capacitor/camera`). El "Puente" detiene JavaScript, le lanza un mensaje por IPC al sistema Android, Android enciende la cámara de fotos, toma la captura, y envía la imagen de regreso a Astro en formato codificado Base64. Todo en milisegundos. |
| **4** | **Ejemplo Práctico 3026** | **Vibrar el celular al hacer Login:** <br> ```javascript import { Haptics, ImpactStyle } from '@capacitor/haptics'; const login = async () => { await Haptics.impact({ style: ImpactStyle.Heavy }); console.log("¡Vibró, login exitoso!"); }; ``` |
| **5** | **¿Por qué es vital para nuestro sistema?** | Da ese "toque especial" nativo. Los usuarios se olvidan que es una página web envuelta cuando el celular empieza a usar la cámara nativa o vibrar al tocar un botón HTMX. |

---

### 🧠 Integración: Fase 10.3 - Sincronización Android (Android Studio)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** | Es el directorio `/android` dentro de tu proyecto Astro que se maneja a través del IDE oficial de Google "Android Studio". |
| **2** | **¿Para qué sirve?** | Es el lugar donde ocurre la compilación final. Es obligatorio para poder firmar el APK/AAB y subir la aplicación a la Google Play Store. |
| **3** | **¿Cómo funciona?** | Tras compilar Astro (`bun run build`), ejecutas `npx cap sync android`. Esto empuja el código web actualizado hacia el chasis nativo. Luego, al ejecutar `npx cap open android`, se abre el IDE pesado de Google que lee toda tu app y te de deja pulsar "Run" para emular el celular. |
| **4** | **Ejemplo Práctico 3026** | **Sincronización:** <br> ```bash bun run build npx cap copy npx cap open android ``` <br> El último comando abre Android Studio con el proyecto listo. |
| **5** | **¿Por qué es vital para nuestro sistema?** | Separación de preocupaciones (SoC). Tú sigues codificando rápido en tu VS Code sin sentir el peso del entorno Android, el cual solo usarás 1 vez al mes para el release formal. |
