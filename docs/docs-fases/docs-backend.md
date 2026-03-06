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

## 🏗️ BLOQUE 0: EL MOTOR (Tooling & Workspace)

### 🧠 Integración: Fase G.1 - La Semilla Física (Workspace)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Un **Workspace de Cargo** es una característica de la herramienta de compilación de Rust que nos permite gestionar múltiples `crates` (paquetes de código) interrelacionados dentro de un mismo superproyecto o monorepo. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para organizar nuestro código en módulos lógicos e independientes (`core_logic`, `api_server`, `infra_db`) pero tratarlos como una unidad cohesiva. El desastre que evita es tener proyectos separados con dependencias duplicadas, configuraciones de compilación inconsistentes y un infierno de integración manual. Centraliza la gestión. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. Se crea un archivo `Cargo.toml` en la raíz del proyecto. <br> 2. Dentro, una sección `[workspace]` declara qué carpetas son miembros (ej: `members = ["crates/core_logic"]`). <br> 3. Todos los `crates` del workspace comparten un único directorio de compilación (`/target`) y un archivo de bloqueo de dependencias (`Cargo.lock`), evitando conflictos. <br> 4. Los comandos de `cargo` (como `check` o `build`) ejecutados desde la raíz se aplican a todo el workspace. |
| **4** | **Ejemplo Práctico 3026** | El `Cargo.toml` que creamos en la raíz es el corazón del workspace. Para verificar que la sintonía es correcta, abre una terminal en la raíz (`c:\laravel\templates\template1\`) y ejecuta el siguiente comando: <br><br> ```bash # Comando de Verificación de Sintonía del Workspace cargo check ``` <br> **Implementación:** Este comando le pide al compilador de Rust que verifique todos los `crates` del workspace en busca de errores, sin llegar a generar un binario. Si termina con `Finished dev [unoptimized + debuginfo]`, significa que la estructura base es coherente y está correctamente enlazada. |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Bajo Costo ($5):** El `Cargo.toml` raíz nos permite definir un `[profile.release]` global. Esto nos garantiza que *todos* los binarios que generemos estén optimizados al extremo para un tamaño mínimo (`opt-level = 'z'`), algo no negociable para un VPS de 512MB de RAM. <br><br> **Alto Rendimiento y Arquitectura Hexagonal:** El workspace es la implementación física de nuestra arquitectura. Fuerza la separación de `core_logic` (el corazón puro) de `api_server` (la antena web). Podemos compilar, probar y auditar la lógica de negocio de forma aislada, asegurando que nunca se contamine con detalles de la infraestructura web, lo que es la esencia de la Sintonía Hexagonal 3026. |

---

### 🧠 Integración: Fase G.2 - El Oráculo y Protocolos (Protobuf)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | **Protocol Buffers (Protobuf)** es un formato de serialización binaria desarrollado por Google para comunicar servicios. Es como un JSON, pero más rápido, pequeño y estricto. **`buf`** es la herramienta que usamos para compilar, validar y generar código a partir de nuestros archivos de definición `.proto`. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para crear un **contrato de datos inquebrantable** (el "ADN") entre el backend (Rust) y cualquier cliente (web, móvil). El desastre que evita es el caos de la desincronización: que el frontend espere un campo `userName` y el backend envíe `user_name`, causando errores en producción. Garantiza que todos los componentes del sistema hablen exactamente el mismo idioma. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Defines** la estructura de tus datos y servicios en un archivo de texto simple (`auth.proto`). <br> 2. **Configuras** las reglas de generación en `buf.gen.yaml`, especificando qué lenguajes quieres (Rust, TypeScript) y dónde guardar el código. <br> 3. **Ejecutas** el comando `buf generate`. <br> 4. `buf` lee tus `.proto`, los valida y usa plugins para **traducir el ADN a código nativo** (structs de Rust, clases de TS) que puedes importar directamente en tu proyecto. |
| **4** | **Ejemplo Práctico 3026** | Para materializar el ADN que definimos en `proto/auth.proto` y convertirlo en código Rust utilizable, usamos nuestro centro de mando. Desde la carpeta raíz (`c:\laravel\templates\template1\`), ejecuta el comando: <br><br> ```bash # Comando de Generación de Código desde el ADN just proto-gen ``` <br> **Implementación:** Este comando le ordena a `buf` que genere el código según las reglas de `buf.gen.yaml`. Para verificar que funcionó, busca los nuevos archivos autogenerados en `crates/core_logic/src/gen/`. ¡No modifiques esos archivos a mano! Son sagrados y se regeneran cada vez que cambias el ADN. |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Bajo Costo ($5) y Alto Rendimiento:** El formato binario de Protobuf es mucho más compacto y rápido de procesar que el JSON. Esto significa menos uso de CPU y menos ancho de banda, dos recursos críticos en un VPS económico. La deserialización es casi instantánea, contribuyendo al rendimiento extremo de nuestro backend en Rust. <br><br> **Multiplataforma (Sintonía Total):** Este es el pilar de la Sintonía 3026. El *mismo* archivo `auth.proto` nos servirá para generar el código del servidor en Rust, el código del cliente web en TypeScript (con Astro/ArkType), y en el futuro, el código para una app móvil nativa. Elimina por completo la posibilidad de errores de comunicación entre plataformas. |

---

## 🏗️ BLOQUE I: FUNDACIÓN (Persistencia y Dominio)

### 🧠 Integración: Fase 1.1 - ADN SQL (Migraciones con SQLx)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Un sistema de **migraciones de base de datos** es un método para gestionar y versionar el esquema de tu base de datos mediante archivos de código SQL. `sqlx-cli` proporciona una herramienta para crear y aplicar estos archivos de forma ordenada y automática. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para garantizar que la estructura de la base de datos sea **consistente y predecible** en todos los entornos (tu PC, el servidor de producción, los tests). El desastre que evita es que tu aplicación se despliegue en un servidor cuya base de datos tiene una estructura diferente a la que espera, causando fallos catastróficos. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. **Creas** una carpeta (`migrations`). <br> 2. **Añades** un archivo SQL con un nombre que empieza por una marca de tiempo (ej: `20260305_create_users.sql`). <br> 3. **Escribes** el SQL para crear o modificar tablas (`CREATE TABLE ...`). <br> 4. **Ejecutas** el comando `sqlx migrate run`. La herramienta se conecta a la DB, revisa una tabla especial (`_sqlx_migrations`) para ver qué migraciones ya ha ejecutado, y aplica solo las nuevas en orden cronológico. |
| **4** | **Ejemplo Práctico 3026** | Hemos creado la migración `20260305135148_create_users_table.sql` y un comando en nuestro `Justfile` para ejecutarla de forma segura. Para aplicarla, abre tu terminal en la raíz y ejecuta: <br><br> ```bash # Comando de Sintonía de Base de Datos just db-migrate ``` <br> **Implementación:** Este comando primero se asegura de que el archivo `backend.db` exista (`db-setup`) y luego ejecuta `sqlx migrate run`. Esto lee nuestra migración y crea la tabla `users` con las columnas correctas. Si lo ejecutas de nuevo, te dirá que no hay nada que hacer, demostrando su inteligencia. |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Bajo Costo ($5):** Nos permite empezar con una base de datos SQLite simple y evolucionarla sin miedo. Si necesitamos añadir una columna, creamos una nueva migración y la desplegamos. Esto es infinitamente más barato y seguro que conectarse manualmente al servidor para modificar la base de datos, arriesgándote a romperlo todo. <br><br> **Alto Rendimiento y Soberanía:** Al escribir el SQL nosotros mismos, tenemos control absoluto sobre los tipos de datos (`BLOB` para UUIDv7), los índices y las restricciones. Esto es clave para optimizar las consultas y asegurar que la base de datos funcione a máxima velocidad. No dependemos de la "magia" de un ORM que podría generar un esquema ineficiente. |

---

### 🧠 Integración: Fase 1.2 - El Corazón Inmortal (Adaptador de Repositorio)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Un **Adaptador de Repositorio** es una clase o estructura (`SqliteUserRepository`) que implementa un "Puerto" de dominio (`IUserRepository`). Es el puente concreto que traduce las necesidades de la lógica de negocio ("guarda este usuario") a comandos específicos de una tecnología ("`INSERT INTO users ...`"). |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para **aislar completamente** nuestra lógica de negocio (`core_logic`) de los detalles de la base de datos (`infra_db`). El desastre que evita es tener código SQL mezclado con las reglas de negocio. Si eso ocurre, cambiar de SQLite a otra base de datos requeriría reescribir el 80% de la aplicación. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. El **Puerto** (`IUserRepository` en `core_logic`) define un contrato abstracto: `async fn save(&self, user: &User)`. <br> 2. El **Adaptador** (`SqliteUserRepository` en `infra_db`) implementa este contrato. Recibe una entidad de dominio (`User`). <br> 3. Un **Mapper** (`DbUser::from_domain`) convierte la entidad de dominio a una estructura que representa la tabla de la base de datos. <br> 4. **SQLx** ejecuta la consulta SQL usando los datos del mapper. <br> 5. Para leer, el proceso es inverso: SQLx lee la fila, la convierte a `DbUser`, y otro mapper (`to_domain_user`) la convierte de nuevo en una entidad `User` con sus `Value Objects`. |
| **4** | **Ejemplo Práctico 3026** | Hemos implementado `SqliteUserRepository` en `crates/infra_db/src/persistence/sqlite/repositories/sqlite_user_repo.rs`. Este adaptador implementa el trait `IUserRepository`. Para verificar que todo compila y está correctamente enlazado, el comando de auditoría es nuestro mejor aliado: <br><br> ```bash # Comando de Verificación de Sintonía de Crates just audit ``` <br> **Implementación:** Si este comando se ejecuta sin errores, significa que `infra_db` puede "ver" el trait `IUserRepository` de `core_logic` y que su implementación es sintácticamente correcta. Hemos conectado exitosamente el "qué" (el puerto) con el "cómo" (el adaptador). |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Bajo Costo ($5) y Soberanía:** Esta arquitectura nos da la libertad de empezar con SQLite, que es gratis y se ejecuta en el mismo VPS. Si el proyecto crece exponencialmente, podemos escribir un `SurrealUserRepository` en un par de días, cambiar **una sola línea** en el archivo de inyección de dependencias, y migrar a una base de datos distribuida sin que la lógica de negocio se entere. Esta flexibilidad es la máxima expresión de la soberanía tecnológica. <br><br> **Alto Rendimiento:** El mapper nos permite asegurar que los datos que viajan hacia y desde la base de datos son óptimos. No arrastramos información innecesaria. Además, la separación permite crear tests de integración que solo prueban la capa de base de datos, facilitando la optimización de consultas SQL complejas. |

---

### 🧠 Integración: Fase 1.4 - El Punto de Entrada (Binario vs. Biblioteca)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | En Rust, un crate puede ser una **biblioteca** (`lib`), diseñada para ser usada por otro código, o un **binario** (`bin`), diseñado para ser ejecutado como un programa. Un crate se convierte en binario cuando tiene un archivo `src/main.rs` con una función `main()`. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para distinguir entre código reutilizable (bibliotecas como `core_logic` y `infra_db`) y el programa que inicia todo el sistema (el binario `api_server`). El desastre que evita es la confusión sobre qué parte del sistema es la responsable de arrancar. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. `cargo run` por sí solo no funciona en un workspace con múltiples crates porque no sabe cuál ejecutar. <br> 2. Usamos el flag `-p` (o `--package`) para especificar el crate binario que queremos ejecutar: `cargo run -p api_server`. <br> 3. Para que un `main` sea asíncrono, usamos la macro `#[tokio::main]`. Esta macro requiere que el crate específico (`api_server`) tenga a `tokio` como dependencia en su propio `Cargo.toml`. |
| **4** | **Ejemplo Práctico 3026** | Hemos convertido `api_server` en un binario creando `crates/api_server/src/main.rs` y añadiendo `tokio` a su `Cargo.toml`. Para una experiencia de desarrollo superior, hemos actualizado el `Justfile` para que el comando `dev` ejecute y vigile nuestro binario: <br><br> ```bash # Comando de Desarrollo con Reinicio Automático just dev ``` <br> **Implementación:** Este comando utiliza `cargo watch` para ejecutar `cargo run -p api_server` y reiniciarlo automáticamente cada vez que guardas un cambio en cualquier archivo dentro de la carpeta `crates/`. |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Claridad Arquitectónica:** Esta distinción refuerza la Arquitectura Hexagonal. `core_logic` y `infra_db` son herramientas pasivas, mientras que `api_server` es el actor principal que las utiliza para construir el sistema. <br><br> **Eficiencia de Desarrollo:** El comando `just dev` nos da un ciclo de feedback instantáneo. Cambias una línea en `core_logic`, guardas, y el servidor se recompila y reinicia solo, permitiéndonos probar los cambios en segundos. |

---

## 🔒 BLOQUE II: EL ESCUDO (Seguridad y Protocolo)

### 🧠 Integración: Fase 2.2 - El Escudo de Contraseñas (Hashing)

| Nivel | Nombre | Descripción |
|-------|--------|-------------|
| **1** | **¿Qué es?** (Definición Técnica) | Es la implementación de un sistema de hashing de contraseñas seguro usando el patrón **Puerto-Adaptador**. El **Puerto** es un `trait` (`IHasher`) en `core_logic` que define un contrato abstracto para hashear y verificar. El **Adaptador** es una estructura (`Argon2idHasher`) en `infra_db` que implementa ese contrato usando el algoritmo criptográfico Argon2id. |
| **2** | **¿Para qué sirve?** (El Propósito) | Sirve para **nunca almacenar contraseñas en texto plano**. El desastre que evita es una brecha de seguridad donde un atacante obtiene acceso a la base de datos y roba las credenciales de todos los usuarios. Con un hashing robusto, incluso si la base de datos es robada, las contraseñas siguen siendo inutilizables. |
| **3** | **¿Cómo funciona?** (La Anatomía) | 1. El caso de uso `RegisterUser` necesita hashear una contraseña. Llama al método `hash()` de su dependencia, que es del tipo abstracto `Arc<dyn IHasher>`. <br> 2. El adaptador `Argon2idHasher` recibe la llamada. <br> 3. Genera una `salt` criptográficamente segura. <br> 4. Usa la librería `argon2` para combinar la contraseña y la `salt`, realizando un cálculo intensivo en CPU para producir un hash. <br> 5. Esta operación se ejecuta en un hilo aparte usando `tokio::spawn_blocking` para no congelar el servidor. <br> 6. Devuelve el hash resultante (que contiene el algoritmo, la salt y el hash real) como un `PasswordHash` (Value Object). |
| **4** | **Ejemplo Práctico 3026** | Hemos creado el puerto `IHasher` y el adaptador `Argon2idHasher`, y el caso de uso `RegisterUser` ya lo está utilizando. Para verificar que la sintonía entre las capas es correcta y que todo el sistema compila, el comando de auditoría es nuestro mejor aliado: <br><br> ```bash # Comando de Verificación de Sintonía de Crates just audit ``` <br> **Implementación:** Un resultado exitoso de `just audit` confirma que el `core_logic` puede usar el puerto `IHasher` y que el `infra_db` lo implementa correctamente, todo sin que el `core_logic` sepa nada sobre Argon2id. Es la Arquitectura Hexagonal en acción. |
| **5** | **¿Por qué es vital para nuestro sistema?** | **Seguridad (Prioridad Absoluta):** Argon2id es el estándar de oro actual para el hashing de contraseñas, diseñado para ser resistente a ataques de fuerza bruta con hardware especializado (GPUs/ASICs). Esto es un pilar fundamental de la Sintonía de Seguridad Total. <br><br> **Bajo Costo ($5) y Alto Rendimiento:** Aunque Argon2id es intencionadamente lento, nuestra implementación con `spawn_blocking` es altamente eficiente para un VPS. Delegamos el trabajo pesado a un hilo secundario, liberando el hilo principal de Tokio para que siga aceptando miles de peticiones por segundo. Esto evita que una sola petición de registro pueda degradar el rendimiento de todo el servidor. <br><br> **Soberanía Tecnológica:** Si en el futuro Argon2id es vulnerado, podemos crear un `SuperHasher2030` que implemente el trait `IHasher` y cambiar **una sola línea** en nuestro `Composition Root` (el archivo de inyección de dependencias). La lógica de negocio (`RegisterUser`) no se modifica, dándonos una libertad y capacidad de adaptación absolutas. |
