# 🩺 PROTOCOLO DE SINTONÍA: TROUBLESHOOTING 3026

**Estado:** 🟢 Activo

---

## Visión

Este documento es la bitácora de combate del Laboratorio 3026. No documenta el "cómo funciona" (para eso está `docs-fases/`), sino el **"qué hacer cuando no funciona"**. Cada error superado es una lección aprendida y una vacuna para el futuro.

---

### 1. Error de Compilación: `file not found for module '...'`

- **Síntoma:** `cargo check` o `clippy` fallan con un error indicando que no pueden encontrar un módulo que has declarado con `mod my_module;`.
  ```text
  error[E0583]: file not found for module `persistence`
   --> crates\infra_db\src\lib.rs:2:1
    |
  2 | pub mod persistence;
    | ^^^^^^^^^^^^^^^^^^^^
  ```

- **Diagnóstico:** En Rust, la declaración de un módulo (`mod module_name;`) le dice al compilador que busque un archivo `module_name.rs` o una carpeta `module_name/mod.rs` en el mismo directorio. Este error significa que esa estructura de archivos no existe.

- **Cura (Sintonía 3026):**
  1.  Si declaras `pub mod persistence;` en `lib.rs`, debes crear un archivo `persistence.rs` o, para una estructura más compleja, una carpeta `persistence/` con un archivo `mod.rs` dentro.
  2.  **Ejemplo:** Para nuestra estructura `infra_db/src/persistence/sqlite/repositories/`, necesitamos una cadena de archivos `mod.rs` que conecten todo:
      - `infra_db/src/lib.rs` contiene `pub mod persistence;`
      - `infra_db/src/persistence/mod.rs` contiene `pub mod sqlite;`
      - `infra_db/src/persistence/sqlite/mod.rs` contiene `pub mod repositories;`
      - `infra_db/src/persistence/sqlite/repositories/mod.rs` contiene `pub mod sqlite_user_repo;`

---

### 2. Error de SQLx: `set 'DATABASE_URL' to use query macros`

- **Síntoma:** Al compilar, `sqlx` falla en las macros `query!` o `query_as!` porque no puede conectarse a la base de datos para verificar las consultas.

- **Diagnóstico:** `sqlx` necesita validar las consultas SQL en tiempo de compilación para garantizar la seguridad de tipos. Para ello, debe poder acceder a la base de datos o a un caché de metadatos.

- **Cura (Sintonía 3026):**
  1.  **Asegurar la DB:** Ejecuta `just db-migrate`. Esto crea el archivo `backend.db` y las tablas necesarias.
  2.  **Generar el Caché Offline:** Ejecuta el comando que hemos creado en nuestro `Justfile`. Esto genera un archivo `sqlx-data.json` en la raíz del crate, que `sqlx` usará para la compilación si no puede acceder a la DB.
      ```bash
      just db-prepare
      ```
  3.  Este enfoque es vital para entornos de CI/CD donde la base de datos no está disponible durante la compilación.

---

### 3. Error de Compilación: `temporary value dropped while borrowed`

- **Síntoma:** El compilador se queja de que un valor temporal se elimina mientras todavía está siendo "prestado".
  ```text
  error[E0716]: temporary value dropped while borrowed
  --> crates\infra_db\src\persistence\sqlite\repositories\sqlite_user_repo.rs:94:13
     |
  94 |             email.as_str()
     |             ^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
  ```

- **Diagnóstico:** Ocurre cuando pasas el resultado de una función (ej: `email.as_str()`) directamente a otra función o macro que necesita una referencia que viva más tiempo. El `&str` devuelto por `as_str()` es temporal y Rust lo elimina inmediatamente, pero la macro `sqlx` y la llamada `.await` posterior todavía lo necesitan.

- **Cura (Sintonía 3026):**
  - Almacena el valor temporal en una variable con `let` antes de usarlo. Esto extiende su ciclo de vida para toda la función.
    ```rust
    // INCORRECTO
    let db_user = sqlx::query_as!(..., email.as_str()).await?;

    // CORRECTO (Sintonía 3026)
      let email_str = email.as_str();
      let db_user = sqlx::query_as!(..., email_str).await?;
      ```

---

### 9. Error de Privacidad: `field is private`

- **Síntoma:** Intentas acceder a un campo de un struct (ej. `user.email`) y el compilador te detiene.
  ```text
  error[E0616]: field `email` of struct `User` is private
  ```

- **Diagnóstico:** En la Arquitectura Hexagonal, las entidades de dominio (`User`) deben proteger su estado interno para garantizar la validez de los datos. Por eso sus campos no son `pub`.

- **Cura (Sintonía 3026):**
  1. **No hagas los campos públicos.** Eso rompería el encapsulamiento.
  2. **Usa Getters:** Define métodos públicos en tu entidad que devuelvan referencias a los datos.
      ```rust
      // En User
      pub fn email(&self) -> &Email { &self.email }
      // En el Handler
      let email_str = user.email().as_str();
      ```

---

### 10. Error de Display: `method `to_string` exists but its trait bounds were not satisfied`

- **Síntoma:** Cuando intentas convertir un Value Object a String en un handler o test.
  ```text
  error[E0599]: the method `to_string` exists for reference `&Email`, but its trait bounds were not satisfied
    --> crates\api_server\src\entry_points\api\v1\user_handlers.rs:42:33
  ```

- **Diagnóstico:** Los Value Objects (`UserId`, `Email`, `PasswordHash`) son wrappers alrededor de `String`. Aunque tienen el método `as_str()`, no implementan `Display`, por lo que no puedes usar `.to_string()` directamente.

- **Cura (Sintonía 3026):**
  - Implementa `Display` en tus Value Objects:
    ```rust
    // En email.rs o user_id.rs
    use std::fmt::{Display, Formatter};

    impl Display for Email {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    ```
  - Ahora puedes usar `.to_string()` en handlers:
    ```rust
    let email_string = user.email().to_string();
    ```

---

### 11. Error de Tests: `use of moved value: app` (Router en Tests)

- **Síntoma:** Al intentar usar el router dos veces en un test de integración.
  ```text
  error[E0382]: use of moved value: `app`
    --> crates\api_server\tests\integration_tests.rs:114:20
  ```

- **Diagnóstico:** El método `oneshot` consume el Router (toma `self`), por lo que no puedes reutilizarlo para múltiples requests en el mismo test sin clonarlo.

- **Cura (Sintonía 3026):**
  - Clona el router antes del segundo request:
    ```rust
    let response = app.clone().oneshot(...).await?;
    let response = app.oneshot(...).await?;
    ```

---

### 12. Error de Tests: `OnceCell::new()` en Contexto Estático

- **Síntoma:** No puedes usar `OnceCell::new()` en un `static`.
  ```text
  error[E0015]: cannot call non-const associated function `tokio::sync::OnceCell::<Migrator>::new` in statics
  ```

- **Diagnóstico:** `OnceCell::new()` no es una función const, no puede usarse en la inicialización de estáticos.

- **Cura (Sintonía 3026):**
  - Usa `LazyLock` en vez de `OnceCell`:
    ```rust
    use std::sync::LazyLock;

    static MIGRATOR: LazyLock<OnceCell<Migrator>> = LazyLock::new(OnceCell::new);
    ```

---

### 13. Error de Tests: `oneshot` method not found

- **Síntoma:** El método `oneshot` no existe en el Router.
  ```text
  error[E0599]: no method named `oneshot` found for struct `Router<S>`
  ```

- **Diagnóstico:** El trait `ServiceExt` de `tower` proporciona el método `oneshot`, pero no está en scope.

- **Cura (Sintonía 3026):**
  - Añade el import:
    ```rust
    use tower::util::ServiceExt;
    ```

---

### 14. Error de Tests: Migration path not found

- **Síntoma:** El migrador no encuentra la ruta de las migraciones.
  ```text
  Failed to create migrator: Source(Os { code: 3, kind: NotFound, message: "El sistema no puede encontrar la ruta especificada." })
  ```

- **Diagnóstico:** La ruta relativa `./crates/infra_db/migrations` no funciona correctamente cuando se ejecutan tests desde el workspace.

- **Cura (Sintonía 3026):**
  - Usa `CARGO_MANIFEST_DIR` para obtener la ruta correcta:
    ```rust
    use std::env;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations_path = Path::new(&manifest_dir).join("../infra_db/migrations");
    ```

---

### 15. Error de Tests: `Argon2idHasher::new()` not found

- **Síntoma:** No existe el método `new()` en `Argon2idHasher`.
  ```text
  error[E0599]: no function or associated item named `new` found for struct `Argon2idHasher`
  ```

- **Diagnóstico:** `Argon2idHasher` usa `#[derive(Default)]`, no tiene un constructor `new()`.

- **Cura (Sintonía 3026):**
  - Usa `default()` en vez de `new()`:
    ```rust
    let hasher = Arc::new(infra_db::Argon2idHasher::default());
    ```

---

### 16. Error de Negocio: Email duplicado devuelve 500 en vez de 409

- **Síntoma:** El test de email duplicado falla porque devuelve 500 Internal Server Error en vez de 409 Conflict.

- **Diagnóstico:** El caso de uso `RegisterUser` no verificaba si el email ya existía antes de intentar guardar. El repositorio hacía `ON CONFLICT` pero no devolvía un error claro.

- **Cura (Sintonía 3026):**
  - Añade la verificación en el caso de uso antes de guardar:
    ```rust
    // En register.rs
    use crate::domain::errors::DomainError;

    // En execute():
    if self.user_repo.find_by_email(&email).await?.is_some() {
        return Err(DomainError::UserAlreadyExists(email.as_str().to_string()).into());
    }
    ```

---

### 4. Error de SQLx: `the trait bound ... is not satisfied` (Conversión de Tipos)

- **Síntoma:** `sqlx::query_as!` falla porque no puede convertir un tipo de dato de la base de datos (ej: `Vec<u8>` para `BLOB`) a un tipo de dato del dominio (ej: `uuid::Uuid`).

- **Diagnóstico:** La macro `query_as!` intenta hacer una conversión `From`/`Into` directa. Si no existe una implementación de ese trait entre el tipo de la DB y el tipo de tu struct, la compilación falla.

- **Cura (Sintonía 3026):**
  1.  **Acepta el Tipo Crudo:** Modifica tu struct DTO (ej: `DbUser`) para que sus campos coincidan con los tipos que `sqlx` lee de la base de datos (`Vec<u8>` para `BLOB`, `NaiveDateTime` para `DATETIME`).
  2.  **Mapea Explícitamente:** En tu función "mapper" (ej: `to_domain_user`), realiza la conversión manual y controlada, manejando los posibles errores.
    ```rust
    // En el mapper
    let uuid = Uuid::from_slice(&db_user.id)
        .map_err(|e| anyhow::anyhow!("Error al parsear UUID: {}", e))?;

    let created_at = DateTime::<Utc>::from_naive_utc_and_offset(db_user.created_at, Utc);
    ```
  - Esto refuerza la Arquitectura Hexagonal: el adaptador es el único responsable de conocer y traducir los detalles "sucios" de la infraestructura.

---

### 5. Error de Migración: `migration ... was previously applied but has been modified`

- **Síntoma:** `just db-migrate` falla porque una migración que ya se había aplicado ha sido modificada.

- **Diagnóstico:** `sqlx` almacena un hash de cada archivo de migración que aplica. Si modificas un archivo que ya fue aplicado, el hash no coincidirá y `sqlx` detendrá el proceso para prevenir la corrupción de la base de datos.

- **Cura (Sintonía 3026):**
  - **En desarrollo:** La forma más limpia es reiniciar la base de datos.
    1.  Elimina los archivos `backend.db`, `backend.db-shm`, `backend.db-wal`.
    2.  Elimina la carpeta `.sqlx` en la raíz del proyecto.
    3.  Vuelve a ejecutar `just db-migrate`.
  - **En producción:** NUNCA modifiques una migración aplicada. En su lugar, crea una **nueva migración** que aplique los cambios necesarios (`ALTER TABLE ...`).

---

### 6. Error de Compilación: `unresolved module 'tokio'` y `main is not allowed to be async`

- **Síntoma:** Al intentar usar `#[tokio::main]` en un crate del workspace, el compilador falla porque no encuentra `tokio` y, como consecuencia, no permite que `main` sea `async`.

- **Diagnóstico:** Las dependencias no se comparten globalmente en un workspace. Cada crate (`api_server`, `core_logic`, etc.) tiene su propio `Cargo.toml` y debe declarar explícitamente las dependencias que utiliza.

- **Cura (Sintonía 3026):**
  - Añade la dependencia necesaria al `Cargo.toml` del crate específico que la está usando. En este caso, `api_server`.
    ```toml
    # En crates/api_server/Cargo.toml
    [dependencies]
    # ... otras dependencias
    tokio = { version = "1.36", features = ["full"] }
    ```

---

### 7. Error de Compilación: `file not found for module 'test'` (Estructura de Tests)

- **Síntoma:** El compilador lanza errores `E0583` indicando que no encuentra módulos como `config` o `entry_points`, y a veces muestra errores extraños sobre módulos duplicados.
  ```text
  error[E0583]: file not found for module `config`
  --> crates\api_server\src\lib.rs:19:1
  ```

- **Diagnóstico:** Rust tiene una convención estricta para los tests de integración. Deben residir en una carpeta llamada **`tests`** (en plural) en la raíz del crate, no dentro de `src/`. Si creas una carpeta `src/test/` o `src/tests/`, el compilador se confunde al intentar resolver los módulos, rompiendo la visibilidad de todo el proyecto.

- **Cura (Sintonía 3026):**
  1.  **Mover:** Mueve tus tests de integración de `src/test/` a `crates/tu_crate/tests/`.
  2.  **Limpiar:** Elimina cualquier archivo `mod.rs` dentro de la nueva carpeta `tests/`. Cargo descubre los archivos de test automáticamente.
  3.  **Importar:** En tus tests, importa tu crate como una librería externa (`use api_server::...`) en lugar de usar `crate::...`.

---

### 8. Error de Compilación: `variant or associated item not found in DomainError`

- **Síntoma:** El compilador se queja de que una variante de un enum no existe, aunque jurarías haberla visto en el código.
  ```text
  error[E0599]: no variant or associated item named `UserAlreadyExists` found for enum `DomainError`
  ```

- **Diagnóstico:** Esto ocurre cuando `core_logic` (donde se define el error) y `api_server` (donde se usa) están desincronizados. Puede ser que hayas actualizado la definición del enum en `core_logic` pero no hayas guardado el archivo, o que estés intentando usar una variante antigua que fue refactorizada (ej. cambiar `InvalidEmail` por un `ValidationError` más genérico).

- **Cura (Sintonía 3026):**
  1.  **Verificar Definición:** Revisa `core_logic/src/domain/errors.rs` y asegúrate de que las variantes coincidan exactamente con lo que esperas.
  2.  **Actualizar Usos:** Si refactorizaste (ej. eliminaste `InvalidEmail`), busca todas las referencias en tu código (Value Objects, Handlers) y actualízalas a la nueva variante (`ValidationError`).

---

### 9. Error de Privacidad: `field is private`

- **Síntoma:** Intentas acceder a un campo de un struct (ej. `user.email`) y el compilador te detiene.
  ```text
  error[E0616]: field `email` of struct `User` is private
  ```

- **Diagnóstico:** En la Arquitectura Hexagonal, las entidades de dominio (`User`) deben proteger su estado interno para garantizar la validez de los datos. Por eso sus campos no son `pub`.

- **Cura (Sintonía 3026):**
  1.  **No hagas los campos públicos.** Eso rompería el encapsulamiento.
  2.  **Usa Getters:** Define métodos públicos en tu entidad que devuelvan referencias a los datos.
      ```rust
      // En User
      pub fn email(&self) -> &Email { &self.email }
      // En el Handler
      let email_str = user.email().as_str();
      ```
