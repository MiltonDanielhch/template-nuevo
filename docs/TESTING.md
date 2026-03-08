# 🧪 Manual de Pruebas - API 3026

**Objetivo:** Guía maestra para validar cada capa del sistema, desde el código hasta el despliegue en producción.

---

## 🏗️ Nivel 1: Pruebas de Código (Unitarias e Integración)

Antes de levantar cualquier servidor, el código debe probarse a sí mismo.

### Comando Maestro
```bash
cargo test --workspace
```

**Qué verifica:**
1.  **Dominio:** Que las reglas de negocio (emails válidos, contraseñas fuertes) se cumplan.
2.  **Casos de Uso:** Que el flujo de registro y login funcione en memoria.
3.  **Integración API:** Levanta un servidor Axum efímero y lanza peticiones reales contra una base de datos SQLite en memoria o archivo temporal.

**Resultado Esperado:**
```text
running 11 tests
test login_user_success ... ok
test register_user_success ... ok
test auth_middleware_rejects_no_token ... ok
...
test result: ok. 11 passed; 0 failed
```

---

## � Nivel 2: Pruebas de API en Local (Dev Server)

Validación manual de los endpoints con el servidor corriendo en tu máquina.

**Requisitos:**
- Servidor corriendo: `just dev` (o `cargo run -p api_server`)
- URL: `http://localhost:8080`

### 🔧 Flujo de Autenticación Completo (cURL)

**1. Registro de Usuario**
```bash
curl -v -X POST http://localhost:8080/register \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"dani@lab3026.com\",\"password\":\"SecurePass123!\"}"
```
✅ **201 Created**: `{"id":"...","email":"admin@lab3026.com"}`

**2. Login (Obtener Token)**
```bash
# Guardamos el token en una variable (PowerShell)
$response = curl -s -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"admin@lab3026.com\",\"password\":\"SecurePass123!\"}"
$token = ($response | ConvertFrom-Json).token
echo "Token: $token"
```
✅ **200 OK**: `{"token":"uuid-v7-token", ...}`

**3. Acceso Protegido (/me)**
```bash
curl -v -X GET http://localhost:8080/me \
  -H "Authorization: Bearer $token"
```
✅ **200 OK**: `{"id":"...","email":"admin@lab3026.com", ...}`

**4. Acceso Denegado (Sin Token)**
```bash
curl -v -X GET http://localhost:8080/me
```
❌ **401 Unauthorized**: `{"error":"Token de autorización requerido"}`

**5. Cerrar Sesión (Logout)**
```bash
curl -v -X POST http://localhost:8080/logout \
  -H "Authorization: Bearer $token"
```
✅ **200 OK**: (Sin contenido o mensaje de éxito)

**6. Acceso Revocado (Token Expirado/Revocado)**
```bash
curl -v -X GET http://localhost:8080/me \
  -H "Authorization: Bearer $token"
```
❌ **401 Unauthorized**: `{"error":"Token inválido o expirado"}`

---

## 🚀 Nivel 3: Pruebas de Despliegue (Producción Simulada)

Validación del contenedor Docker/Podman y el proxy Caddy.

**Requisitos:**
- Docker o Podman instalado.
- Puertos 80 y 443 libres.

### 1. Levantar la Infraestructura
```bash
cd deploy
podman-compose up --build -d
```

### 2. Verificar Contenedores
```bash
podman ps
```
✅ Debes ver `lab3026_api` (Backend) y `lab3026_caddy` (Proxy).

### 3. Prueba de Conectividad HTTPS
Caddy genera un certificado autofirmado para `localhost`. Debes usar `-k` (insecure) en curl localmente.

```bash
curl -k -v https://localhost/me
```
✅ **401 Unauthorized** (Esto es bueno, significa que Caddy recibió la petición HTTPS y se la pasó a Rust, quien la rechazó por falta de token).

### 4. Prueba de Persistencia (Resiliencia)
Si reinicias el contenedor, los usuarios NO deben borrarse.

1. Registra un usuario (ver paso 1 arriba).
2. Reinicia el pod: `podman-compose restart`
3. Intenta hacer login con ese usuario.
✅ **200 OK**: Si funciona, el volumen de SQLite está montado correctamente.

---

## 🗄️ Comandos de Diagnóstico (Forensics)

Si algo falla, usa esto para mirar dentro del cerebro del sistema.

**Ver Logs en Vivo:**
```bash
podman-compose logs -f
```

**Inspeccionar Base de Datos (Local):**
```bash
sqlite3 backend.db "SELECT * FROM users;"
sqlite3 backend.db "SELECT * FROM sessions;"
```

**Inspeccionar Base de Datos (Dentro del Contenedor):**
```bash
podman exec -it lab3026_api sqlite3 /app/data/backend.db "SELECT count(*) FROM users;"
```

---

## 📊 Matriz de Errores Comunes

| Código | Mensaje | Causa Probable | Solución |
|--------|---------|----------------|----------|
| **400** | `Error en los datos` | JSON malformado o email inválido. | Revisa el cuerpo del request. |
| **401** | `Token inválido` | Token expirado, manipulado o inexistente. | Haz login de nuevo. |
| **409** | `El email ya existe` | Intentas registrar un email duplicado. | Usa otro email o haz login. |
| **500** | `Internal Server Error` | Pánico en Rust o fallo de DB. | Revisa `cargo run` logs o `podman logs`. |
| **502** | `Bad Gateway` | Caddy no puede conectar con Rust. | El contenedor de Rust se cayó o no ha iniciado. |
