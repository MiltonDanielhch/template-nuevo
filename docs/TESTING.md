# 🧪 Manual de Pruebas - API 3026

**Objetivo:** Documentar todos los comandos para probar los endpoints de la API y los resultados esperados.

**Requisitos previos:**
- Servidor corriendo en `http://localhost:8080`
- Base de datos SQLite inicializada

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
