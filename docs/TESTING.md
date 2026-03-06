# 🧪 Manual de Pruebas - API 3026

**Objetivo:** Documentar todos los comandos para probar los endpoints de la API y los resultados esperados.

**Requisitos previos:**
- Servidor corriendo en `http://localhost:8080`
- Base de datos SQLite: `backend.db`

---

## 📋 Endpoints Disponibles

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| POST | `/register` | Registrar nuevo usuario |
| POST | `/login` | Autenticar usuario |

---

## ✅ Tests Automatizados

### Ejecutar todos los tests
```bash
cargo test --workspace
```

**Resultado esperado:**
```
running 2 tests
test register_user_duplicate_email ... ok
test register_user_success ... ok

test result: ok. 2 passed; 0 failed
```

---

## 🔧 Pruebas Manuales con cURL

### 1. Registrar Usuario (POST /register)

**Comando:**
```bash
curl -X POST http://localhost:8080/register ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```

**Resultado esperado (200 OK):**
```json
{
  "id": "01HV...",
  "email": "test@test.com"
}
```

---

### 2. Registrar Usuario con Email Duplicado (POST /register)

**Comando:**
```bash
curl -X POST http://localhost:8080/register ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```

**Resultado esperado (409 Conflict):**
```json
{
  "error": "El email ya está en uso."
}
```

---

### 3. Login Exitoso (POST /login)

**Prerrequisito:** Tener un usuario registrado (ver test 1)

**Comando:**
```bash
curl -X POST http://localhost:8080/login ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```

**Resultado esperado (200 OK):**
```json
{
  "id": "01HV...",
  "email": "test@test.com"
}
```

---

### 4. Login con Email Incorrecto (POST /login)

**Comando:**
```bash
curl -X POST http://localhost:8080/login ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"noexiste@test.com\",\"password\":\"password123\"}"
```

**Resultado esperado (401 Unauthorized):**
```json
{
  "error": "Credenciales inválidas."
}
```

---

### 5. Login con Contraseña Incorrecta (POST /login)

**Prerrequisito:** Tener un usuario registrado

**Comando:**
```bash
curl -X POST http://localhost:8080/login ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"test@test.com\",\"password\":\"passwordMALO\"}"
```

**Resultado esperado (401 Unauthorized):**
```json
{
  "error": "Credenciales inválidas."
}
```

---

### 6. Validar Email Inválido (POST /register)

**Comando:**
```bash
curl -X POST http://localhost:8080/register ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"no-es-valido\",\"password\":\"password123\"}"
```

**Resultado esperado (400 Bad Request):**
```json
{
  "error": "El email 'no-es-valido' no es válido."
}
```

---

## 🐛 Comandos de Desarrollo

### Iniciar servidor en desarrollo
```bash
just dev
```

### Verificar compilación
```bash
cargo check
```

### Compilar todo el workspace
```bash
cargo build
```

### Ejecutar con output verbose
```bash
RUST_LOG=debug cargo run
```

---

## 📊 Flujo Completo de Prueba

```bash
# 1. Iniciar servidor
just dev

# 2. En otra terminal - Registrar usuario
curl -X POST http://localhost:8080/register -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"

# 3. Login exitoso
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"

# 4. Login con contraseña incorrecta
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"malacontraseña\"}"
```

---

## 🎯 Casos de Prueba Resumidos

| # | Escenario | Input | Output Esperado |
|---|-----------|-------|-----------------|
| 1 | Registro exitoso | email + password válidos | 200 + {id, email} |
| 2 | Registro email duplicado | email ya usado | 409 Conflict |
| 3 | Login exitoso | credenciales válidas | 200 + {id, email} |
| 4 | Login email no existe | email no registrado | 401 Unauthorized |
| 5 | Login password incorrecto | password wrong | 401 Unauthorized |
| 6 | Registro email inválido | email sin @ | 400 Bad Request |

---

## 🗄️ Verificar Base de Datos SQLite

Los datos se guardan en `backend.db`. Aquí hay varias formas de verificar:

### Opción 1: Ver archivo directamente con SQLite CLI

```bash
# Ver todos los usuarios registrados
sqlite3 backend.db "SELECT id, email, email_verified, created_at FROM users;"
```

**Resultado esperado:**
```
01HV...|test@test.com|0|2026-03-06 12:00:00
```

---

### Opción 2: Ver estructura de la tabla

```bash
sqlite3 backend.db ".schema users"
```

---

### Opción 3: Contar registros

```bash
sqlite3 backend.db "SELECT COUNT(*) FROM users;"
```

---

### Opción 4: Ver información detallada de un usuario

```bash
sqlite3 backend.db "SELECT * FROM users WHERE email = 'test@test.com';"
```

---

### Opción 5: Usar DBeaver o SQLite Viewer (GUI)

Abre el archivo `backend.db` con:
- **DBeaver** (recomendado)
- **SQLite Browser** (https://sqlitebrowser.org/)
- **VS Code** + extensión "SQLite"

---

## 🔍 Otras Formas de Probar el Proyecto

### Tests de integración (ya configurados)
```bash
cargo test --workspace
```

### Ver logs del servidor
```bash
# Con debug
RUST_LOG=debug cargo run -p api_server

# Solo errores
RUST_LOG=error cargo run -p api_server
```

### Probar con HTTPie (alternativa a curl)
```bash
# Instalar: pip install httpie

# Registrar
http POST localhost:8080/register email="test2@test.com" password="password123"

# Login
http POST localhost:8080/login email="test2@test.com" password="password123"
```

### Probar con Postman

1. Descarga Postman
2. Crea una nueva colección
3. Añade requests:
   - `POST localhost:8080/register` (Body: JSON)
   - `POST localhost:8080/login` (Body: JSON)
4. Envía y verifica respuestas

---

## 🚀 Flujo Completo de Prueba (Recomendado)

```bash
# 1. Iniciar servidor (terminal 1)
just dev

# 2. Registrar usuario (terminal 2)
curl -X POST http://localhost:8080/register -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"

# 3. Verificar que se guardó en la base de datos
sqlite3 backend.db "SELECT id, email, created_at FROM users;"

# 4. Login exitoso
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"

# 5. Login con contraseña incorrecta
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"WRONG\"}"
```
