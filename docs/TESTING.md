# 🧪 Manual de Pruebas - API 3026

**Objetivo:** Documentar todos los comandos para probar los endpoints de la API.

**Requisitos:**
- Servidor corriendo en `http://localhost:8080`
- Base de datos: `backend.db`

---

## 📋 Endpoints

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| POST | `/register` | Registrar usuario |
| POST | `/login` | Autenticar (retorna token) |
| GET | `/me` | Datos del usuario autenticado |
| POST | `/logout` | Cerrar sesión |

---

## ✅ Tests Automatizados

```bash
cargo test --workspace
```

**Resultado:**
```
running 6 tests
test login_user_email_not_found ... ok
test login_user_success ... ok
test login_user_wrong_password ... ok
test register_user_duplicate_email ... ok
test register_user_invalid_email ... ok
test register_user_success ... ok

test result: ok. 6 passed; 0 failed
```

---

## 🔧 Comandos cURL (todos probados)

### 1. Registro
```bash
curl -X POST http://localhost:8080/register -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```
**Resultado:** `{"id":"...","email":"test@test.com"}`

### 2. Login (ahora retorna token)
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```
**Resultado:** `{"id":"...","email":"test@test.com","token":"uuid-v7"}`

---

## 🗄️ Ver Base de Datos

```bash
sqlite3 backend.db "SELECT id, user_id, session_token, expires_at FROM sessions;"
```

## 🔧 Comandos cURL (todos probados)

**⚠️ IMPORTANTE:** Usar una sola línea (NO usar `^` para continuar).

### 1. Registro
```bash
curl -X POST http://localhost:8080/register -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```
**Resultado:** `{"id":"01HV...","email":"test@test.com"}`

### 2. Email Duplicado
```bash
curl -X POST http://localhost:8080/register -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```
**Resultado:** `{"error":"El email ya está en uso."}`

### 3. Login Exitoso
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"password123\"}"
```
**Resultado:** `{"id":"01HV...","email":"test@test.com"}`

### 4. Login Email No Existe
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"noexiste@test.com\",\"password\":\"password123\"}"
```
**Resultado:** `{"error":"Credenciales inválidas."}`

### 5. Login Password Incorrecto
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"email\":\"test@test.com\",\"password\":\"WRONG\"}"
```
**Resultado:** `{"error":"Credenciales inválidas."}`

### 6. Email Inválido
```bash
curl -X POST http://localhost:8080/register -H "Content-Type: application/json" -d "{\"email\":\"no-es-valido\",\"password\":\"password123\"}"
```
**Resultado:** `{"error":"Error en los datos proporcionados."}`

### 7. GET /me (Con token)
```bash
curl -X GET http://localhost:8080/me -H "Authorization: Bearer <TOKEN>"
```
**Resultado:** `{"id":"...","email":"test@test.com","username":null,"email_verified":false}`

### 8. GET /me (Sin token)
```bash
curl -X GET http://localhost:8080/me
```
**Resultado:** `{"error":"Token de autorización requerido"}`

### 9. Logout
```bash
curl -X POST http://localhost:8080/logout -H "Authorization: Bearer <TOKEN>"
```
**Resultado:** 204 No Content

---

## 🗄️ Ver Base de Datos

```bash
sqlite3 backend.db "SELECT id, email, email_verified, created_at FROM users;"
```

**Resultado:** `01HV...|test@test.com|0|2026-03-06 20:48:29`

---

## 🎬 Scripts

### Windows
```bash
test-api.bat
```

### Linux/Mac/Git Bash
```bash
chmod +x test-api.sh && ./test-api.sh
```

---

## 🚀 Iniciar Servidor

```bash
just dev
```

**Output esperado:**
```
🚀 Servidor API 3026 listo para la sintonía en 0.0.0.0:8080
```

---

## 📊 Resumen de Casos

| # | Escenario | Output |
|---|-----------|--------|
| 1 | Registro exitoso | 200 + {id, email} |
| 2 | Email duplicado | 409 Conflict |
| 3 | Login exitoso | 200 + {id, email, token} |
| 4 | Email no existe | 401 Unauthorized |
| 5 | Password incorrecto | 401 Unauthorized |
| 6 | Email inválido | 400 Bad Request |
| 7 | GET /me con token | 200 + {id, email, username, email_verified} |
| 8 | GET /me sin token | 401 Unauthorized |
| 9 | Logout | 204 No Content |
