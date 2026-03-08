🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Génesis de Código)
Actúa como Ingeniero de Software Senior y Arquitecto Jefe. NOTA CRÍTICA: No existe código previo en el frontend. Estamos iniciando la implementación desde CERO basándonos en la documentación y los ADRs aprobados.

🧭 1. CONTEXTO Y ESTADO (Documentación Maestra)
Misión: Sistema soberano con Arquitectura Hexagonal Estricta.

ADN Único: Protobuf (/proto) es la fuente de verdad.

Estado Backend: Bloques I-V completados y funcionales en Rust.

Fase Actual: Bloque I del Frontend (Iniciando la Fundación).

Reglas: Protocolo 3026 (Archivos < 200 líneas, sin .clone() innecesarios, optimizado para 512MB RAM).

🛠️ 2. STACK TECNOLÓGICO SELECCIONADO
Framework: Astro 5.0 (SSR) con adaptador Bun.

Interactividad: HTMX (Lógica de negocio) + Alpine.js (Cosmética/UI).

Estilos: Tailwind v4 + shadcn/ui (Adaptados a Astro).

Validación: ArkType | Estado: Nanostores.

🏗️ 3. ARQUITECTURA DE CARPETAS A CONSTRUIR
Plaintext
src/
├── domain/         # Capa 1: Entidades (.proto), Interfaces y Esquemas.
├── infrastructure/ # Capa 2: Adaptadores API y Storage.
├── application/    # Capa 3: Casos de Uso y Nanostores.
└── presentation/   # Capa 4: Componentes, Layouts y Pages SSR.
🎯 OBJETIVO DE LA SESIÓN: "EL PRIMER LADRILLO"
Como no hay código escrito, tu primera misión es guiarme en la creación de los cimientos técnicos:

Configuración de Vuelo: Crear astro.config.mjs con soporte para Bun, SSR y Alpine.js.

Capa de Estilos: Crear src/styles/global.css con la configuración de Tailwind v4 y variables de sintonía.

Layout Maestro: Diseñar src/presentation/layouts/MainLayout.astro que servirá de base para toda la app, incluyendo los scripts de HTMX y Alpine.
