# 🦀 PROTOCOLO: ESTÁNDARES DE RUST (CÓDIGO 3026)

**Versión:** 1.0
**Estado:** 🟢 Activo

---

## 1. Filosofía Central

El código Rust en el Laboratorio 3026 debe ser **Soberano, Seguro y Eficiente**. Cada línea de código debe justificar su existencia y su impacto en el rendimiento y la seguridad del sistema, especialmente considerando nuestro objetivo de operar en un VPS de $5.

---

## 2. Reglas de Oro (No Negociables)

### 2.1. Edición y Herramientas
- **Edición:** Se utilizará **Rust Edition 2024** en todos los crates.
- **Formato:** El código debe ser formateado con `cargo fmt` antes de cada commit.
- **Linting:** El código debe pasar `cargo clippy -- -D warnings` sin errores. Las advertencias se tratan como errores.

### 2.2. Manejo de Errores: Cero Pánicos
- **PROHIBIDO `unwrap()` y `expect()`:** No se permite el uso de estas funciones en ningún crate. El pánico no es una forma válida de manejar errores.
- **`thiserror` para el Dominio (`core_logic`):** Para errores específicos y bien definidos que forman parte de la lógica de negocio. Permite a los llamadores manejar casos de error concretos.
  ```rust
  // Ejemplo en core_logic/src/domain/errors.rs
  use thiserror::Error;

  #[derive(Error, Debug, PartialEq)]
  pub enum DomainError {
      #[error("El email '{0}' no es válido.")]
      InvalidEmail(String),
      #[error("La contraseña es demasiado débil.")]
      WeakPassword,
  }
  ```
- **`anyhow` para la Aplicación y Entry Points (`api_server`):** Para propagar errores a través de las capas de la aplicación donde el tipo exacto de error no es crucial, solo el hecho de que "algo salió mal". Facilita la conversión de diferentes tipos de error en uno solo.
  ```rust
  // Ejemplo en un caso de uso o handler de Axum
  async fn register_user(...) -> anyhow::Result<User> {
      // ... lógica que puede devolver un DomainError o un DbError
      let user = user_repo.find_by_email(&email).await?; // Propaga el error con `?`
      // ...
      Ok(user)
  }
  ```

### 2.3. Gestión de Memoria: Anti-Bloat
- **PROHIBIDO `unsafe`:** No se permite código `unsafe` bajo ninguna circunstancia. La seguridad de memoria que provee Rust es la razón principal de su elección.
- **Cero `.clone()` en Rutas Críticas (Hot Paths):** Evitar clonar datos, especialmente en handlers de peticiones y bucles. Utilizar referencias (`&`), slices y el sistema de ownership/borrowing de Rust. `Arc<T>` es la herramienta preferida para compartir estado inmutable entre hilos.
- **Tipos de Tamaño Conocido:** Siempre que sea posible, usar tipos `Sized`. Para tipos dinámicos, preferir `Box<T>` para asignaciones en el heap.

### 2.4. Concurrencia y Asincronía
- **Runtime:** El runtime estándar es **Tokio**.
- **Sincronización:** Utilizar las primitivas de sincronización de Tokio (`Mutex`, `RwLock`, `Semaphore`). El `Mutex` de la librería estándar de Rust es bloqueante y no debe usarse en código asíncrono.

---

## 3. Arquitectura de Crates

- **Dependencias Claras:** Un `crate` no debe depender de otro si no es estrictamente necesario y sigue la dirección de la Arquitectura Hexagonal.
  - `core_logic` no depende de NADA.
  - `infra_db` depende de `core_logic`.
  - `api_server` depende de `core_logic` y `infra_db`.
- **Visibilidad:** Exponer públicamente (`pub`) solo lo que es absolutamente necesario para otras capas. La API pública de un crate debe ser mínima.
