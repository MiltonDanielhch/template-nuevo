## Objetivo
Herramienta final para convertirte en maestro. Cada vez que la IA termine un punto o fase, no solo lo leas, sino que lo **integres** en tu cerebro usando el método de Feynman adaptado al Código 3026.

> "Actúa como un Mentor de Ingeniería de Software experto en la metodología de Feynman.

## Los 5 Niveles del Método 3026

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Una explicación precisa pero sin rodeos |
| **2** | **¿Para qué sirve?** (El Propósito) | El problema real que resuelve. Si no existiera esto, ¿qué desastre ocurriría? |
| **3** | **¿Cómo funciona?** (La Anatomía) | Explica la mecánica interna paso a paso. Usa diagramas de texto o analogías si es complejo |
| **4** | **Ejemplo Práctico 3026** | Muestra un fragmento de código mínimo, limpio y comentado que aplique este concepto a nuestro proyecto (Rust, Python o Astro)  aqui el comando que ultilizaste y como implentarlo en el proyecto para saber si funciona
| **5** | **¿Por qué es vital para nuestro sistema?** | Explica cómo este concepto ayuda a nuestra meta de Bajo Costo ($5), Alto Rendimiento y Multiplataforma |

--

## 📡 BLOQUE V: DESPLIEGUE SOBERANO

### 🧠 Integración: Fase 5.1 - Orquestación Rootless (Podman & Caddy)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Un sistema de despliegue basado en contenedores que combina **Podman** (motor de contenedores sin daemon y sin root), **Docker Multi-stage Build** (para compilar Rust) y **Caddy** (servidor web con HTTPS automático). |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para poner tu aplicación en producción de forma segura, rápida y barata. El desastre que evita es tener que configurar manualmente servidores, certificados SSL que caducan, o sufrir brechas de seguridad masivas si un atacante escapa del contenedor (ya que no somos root). |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Compilación (Chef):** El `Dockerfile` usa `cargo-chef` para cachear las dependencias de Rust. Solo recompila tu código, no las librerías, ahorrando 90% del tiempo. <br> 2. **Empaquetado (Distroless):** El binario final se copia a una imagen `cc-debian12` minúscula, sin shell ni herramientas extra. <br> 3. **Orquestación (Compose):** `podman-compose` levanta dos servicios: la `app` (API) y `caddy`. <br> 4. **Proxy:** Caddy recibe el tráfico en el puerto 443 (HTTPS), gestiona los certificados y pasa las peticiones a la `app` en el puerto 8080 dentro de una red privada. |
| **4** | **Ejemplo Práctico 3026** | Hemos creado 3 archivos clave: `deploy/Dockerfile`, `deploy/podman-compose.yml` y `deploy/Caddyfile`. Para desplegar tu sistema ahora mismo: <br><br> ```bash # 1. Construir y Levantar cd deploy podman-compose up --build -d # 2. Ver logs podman-compose logs -f ``` <br> **Resultado:** Tu API estará accesible en `https://localhost` (o tu dominio real) con cifrado de grado militar. |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Bajo Costo ($5):** La imagen final es diminuta (<50MB) y Caddy es muy ligero. Todo corre holgadamente en 512MB de RAM. <br><br> **Seguridad Rootless:** Si alguien hackea tu API, se queda atrapado en un usuario sin privilegios. No puede tocar el sistema operativo del VPS. <br><br> **Velocidad:** El caché de `cargo-chef` permite iterar y desplegar cambios en segundos, no minutos. |

---

### 🧠 Integración: Fase 5.2 - Corrección de Inyección de Dependencias (Auth Extractor)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Una corrección en la implementación del trait `FromRequestParts` en Axum 0.8 para el extractor `CurrentUser`. Cambiamos la forma de obtener el estado de la aplicación (`AppState`) desde `parts.extensions` (método antiguo) a inyección directa en la firma de la función (método moderno). |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para que el middleware de autenticación tenga acceso a la base de datos. El desastre que evita es el error **"Estado no disponible"** (Error 500) cuando un usuario intenta acceder a rutas protegidas como `/me` o `/logout`. Sin esto, la autenticación era imposible. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Antes (Axum 0.7 legacy):** Intentábamos buscar el estado manualmente en un mapa de extensiones (`parts.extensions.get::<AppState>()`). Esto fallaba porque Axum 0.8 gestiona el estado de forma diferente. <br> 2. **Ahora (Axum 0.8 way):** Definimos `impl FromRequestParts<AppState> for CurrentUser`. Axum inyecta automáticamente el `AppState` como segundo argumento (`state: &AppState`). <br> 3. **Resultado:** El extractor tiene acceso inmediato y garantizado al repositorio de sesiones sin búsquedas falibles. |
| **4** | **Ejemplo Práctico 3026** | El archivo corregido es `crates/api_server/src/entry_points/auth.rs`. <br><br> **Código Corregido:** <br> ```rust impl FromRequestParts<AppState> for CurrentUser { async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> { // Ahora 'state' ya es nuestro AppState válido. ¡Magia! let session_repo = &state.session_repo; // ... validación del token ... } } ``` |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Estabilidad:** Elimina una fuente de pánicos en tiempo de ejecución. Si el servidor compila, la inyección de dependencias funciona. <br><br> **Simplicidad:** El código es más limpio y fácil de leer, eliminando "ruido" de infraestructura innecesario. |

---

### 🧠 Integración: Fase 5.3 - Sintonía de Telemetría (Logging & Tracing)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Implementación de un sistema de registro (logging) persistente utilizando `tracing`, `tracing-subscriber` y `tracing-appender`. Permite capturar eventos del sistema tanto en la consola como en archivos físicos con rotación diaria. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para tener visibilidad total de lo que ocurre en el servidor ("Caja Negra"). El desastre que evita es que, ante un error en producción, no sepamos qué pasó (¿Falló la DB? ¿Fue un 404? ¿Un error de validación?). |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Capas (Layers):** Configuramos dos capas en el suscriptor. Una envía logs a `stdout` (consola) y otra a un `RollingFileAppender`. <br> 2. **Rotación:** El appender crea un archivo nuevo cada día (`backend.log.YYYY-MM-DD`) en la carpeta `logs/`. <br> 3. **Filtrado:** Usamos `EnvFilter` para que, vía `.env`, podamos silenciar logs ruidosos (ej: sqlx) y enfocarnos en nuestra lógica (`debug` o `info`). |
| **4** | **Ejemplo Práctico 3026** | **Inicialización en `main.rs`:** <br> ```rust let file_appender = tracing_appender::rolling::daily("logs", "backend.log"); tracing_subscriber::registry().with(EnvFilter::from_default_env()).with(fmt::layer().with_writer(non_blocking)).init(); info!("🚀 Servidor Iniciado"); ``` <br> **Uso en Handlers:** `debug!("Payload recibido: {:?}", payload);` |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Mantenibilidad:** Reduce el tiempo de diagnóstico de fallos de horas a segundos. <br> **Auditoría:** Los archivos de log persistentes son la "caja negra" legal y técnica de cualquier sistema profesional. |

---

## 🛡️ BLOQUE VI: GESTIÓN DE ACCESO (RBAC)

### 🧠 Integración: Fase 6.1 - RBAC con Extractor Genérico (Axum & Traits)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Un sistema de Control de Acceso basado en Roles (RBAC) que utiliza un extractor genérico de Axum (`RequirePermission<P>`) y un sistema de "Permission Markers" (traits) para verificar permisos en tiempo de compilación y ejecución. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para restringir el acceso a partes específicas de la API según los privilegios del usuario. Evita que usuarios normales creen roles o borren otros usuarios, protegiendo la integridad del sistema. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Definición:** Creamos un trait `Permission` con una constante `NAME`. <br> 2. **Marcadores:** Creamos structs vacíos como `RolesWrite` que implementan `Permission`. <br> 3. **Extractor:** El extractor `RequirePermission<P>` se ejecuta antes que el handler. Busca el usuario actual, consulta sus permisos en el `IRoleRepository` y, si no tiene el permiso `P::NAME`, devuelve un `403 Forbidden`. |
| **4** | **Ejemplo Práctico 3026** | **En el Handler:** <br> ```rust pub async fn create_role_handler(State(state): State<AppState>, _perm: RequirePermission<RolesWrite>, Json(payload): Json<CreateRoleRequest>) -> ... { ... } ``` <br> **En el Middleware:** `pub struct RolesWrite; impl Permission for RolesWrite { const NAME: &'static str = "roles:write"; }` |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Seguridad Avanzada:** Implementa el principio de "mínimo privilegio". <br> **Mantenibilidad:** El uso de tipos (traits) en lugar de strings mágicos en los handlers evita errores tipográficos y facilita el refactor. |

---

## 👥 BLOQUE VII: SOBERANÍA DE DATOS (CRUD COMPLETE)

### 🧠 Integración: Fase 7.1 - Restauración de Operaciones CRUD

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Implementación de las operaciones básicas (Create, Read, Update, Delete) para la entidad de Usuario, siguiendo el flujo Hexagonal completo desde el repositorio hasta el API Handler. |
| **2** | **¿Para que sirve?** (El Propósito) | Permite la gestión administrativa de los usuarios del sistema. Sin esto, no podríamos listar usuarios, actualizar perfiles o dar de baja cuentas (Soft Delete). |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Domain:** `IUserRepository` define `find_all` y `delete`. <br> 2. **Application:** Casos de uso `ListUsers`, `UpdateUser` y `DeleteUser` manejan la lógica y validación. <br> 3. **Infrastructure:** `SqliteUserRepository` ejecuta el SQL (usando `deleted_at IS NULL` para Soft Delete). <br> 4. **API:** Handlers en `user_handlers.rs` exponen los endpoints protegidos. |
| **4** | **Ejemplo Práctico 3026** | **Endpoint:** `PUT /api/v1/users/{id}` <br> **Comando:** `cargo check --workspace` para validar que todos los mappers y tipos coinciden entre capas. |
### 🧠 Integración: Fase 7.2 - Soporte de Username e Integración Real
| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Extensión del sistema de usuarios para incluir un `username` opcional y la sincronización real del flujo de autenticación entre el frontend (Astro) y el backend (Rust). |
| **2** | **¿Para que sirve?** (El Propósito) | Permite una identificación más amigable que el email y asegura que el frontend use el motor de seguridad real de Rust en lugar de mocks. Evita el desastre de tener datos inconsistentes o una seguridad "de juguete". |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Backend:** Se actualizó la entidad `User` y el comando `RegisterUser` para aceptar un `Option<String>` como username. <br> 2. **DTOs:** Se añadieron campos de username a las peticiones y respuestas JSON. <br> 3. **Frontend:** El proxy de login en Astro ahora redirige las credenciales al puerto 8081 (Axum) y gestiona la cookie de sesión real. |
| **4** | **Ejemplo Práctico 3026** | **Comando de Verificación:** `node test_backend.js` (Script que creamos para simular el flujo completo de registro/login real). |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Profesionalismo:** Un sistema sin nombres de usuario se siente incompleto. Con esto, el laboratorio está listo para escalar a una red social o sistema de gestión real con identidad propia. |
