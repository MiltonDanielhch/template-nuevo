#!/bin/bash

echo "==== Registro de Usuario ===="
curl -X POST http://localhost:8080/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"password123"}'

echo ""
echo "==== Login Exitoso ===="
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"password123"}'

echo ""
echo "==== Login Wrong Password ===="
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"WRONG"}'

echo ""
echo "==== Ver Base de Datos ===="
sqlite3 backend.db "SELECT id, email, created_at FROM users;"
