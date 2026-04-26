# API REST Rust CRUD

API REST de alta performance desenvolvida em Rust com Axum, PostgreSQL e Redis.

## Funcionalidades

- **CRUD de Usuários**: Criar, Listar, Obter, Atualizar e Deletar usuários
- **Autenticação**: JWT com hash de senha Argon2
- **Cache**: Redis para melhor performance de leitura
- **Paginação**: Paginação baseada em cursor
- **Logging**: Logging estruturado com tracing
- **Métricas**: Endpoint /metrics compatível com Prometheus

## Tecnologias

- **Linguagem**: Rust 1.75+
- **Web Framework**: Axum 0.7+
- **Banco de Dados**: PostgreSQL 15+
- **Cache**: Redis 7+
- **ORM**: SQLx
- **Auth**: JWT + Argon2

## Executando o Projeto

### Com Docker

```bash
docker-compose up --build
```

A API estará disponível em `http://localhost:8080`

### Executando Localmente

1. Configure as variáveis de ambiente:

```bash
export APP_DATABASE__HOST=localhost
export APP_DATABASE__PORT=5432
export APP_DATABASE__USERNAME=postgres
export APP_DATABASE__PASSWORD=postgres
export APP_DATABASE__NAME=rust_crud
export APP_REDIS__HOST=localhost
export APP_REDIS__PORT=6379
export APP_JWT__SECRET=sua-chave-secreta
```

2. Execute o servidor:

```bash
cargo run --release
```

## Endpoints da API

| Método | Endpoint | Tipo | Descrição |
|--------|----------|------|-----------|
| GET | /health | Público | Health check |
| POST | /auth/register | Público | Criar usuário |
| POST | /auth/login | Público | Login (retorna token) |
| GET | /users | Protegido | Listar usuários |
| GET | /users/:id | Protegido | Obter usuário |
| PUT | /users/:id | Protegido | Atualizar usuário |
| DELETE | /users/:id | Protegido | Deletar usuário |
| GET | /metrics | Público | Métricas |

## Exemplos com curl

### Health Check
```bash
curl http://localhost:8080/health
```

### Registrar Usuário
```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@exemplo.com","password":"senha123"}'
```

### Login
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@exemplo.com","password":"senha123"}'
```

### Listar Usuários (protegido)
```bash
curl http://localhost:8080/users \
  -H "Authorization: Bearer TOKEN_AQUI"
```

### Obter Usuário por ID (protegido)
```bash
curl http://localhost:8080/users/ID_DO_USUARIO \
  -H "Authorization: Bearer TOKEN_AQUI"
```

### Atualizar Usuário (protegido)
```bash
curl -X PUT http://localhost:8080/users/ID_DO_USUARIO \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer TOKEN_AQUI" \
  -d '{"email":"novo@email.com"}'
```

### Deletar Usuário (protegido)
```bash
curl -X DELETE http://localhost:8080/users/ID_DO_USUARIO \
  -H "Authorization: Bearer TOKEN_AQUI"
```

## Variáveis de Configuração

| Variável | Padrão | Descrição |
|----------|--------|-------------|
| APP_SERVER__PORT | 8080 | Porta do servidor |
| APP_SERVER__HOST | 0.0.0.0 | Host do servidor |
| APP_DATABASE__HOST | localhost | Host do PostgreSQL |
| APP_DATABASE__PORT | 5432 | Porta do PostgreSQL |
| APP_DATABASE__USERNAME | postgres | Usuário do PostgreSQL |
| APP_DATABASE__PASSWORD | postgres | Senha do PostgreSQL |
| APP_DATABASE__NAME | rust_crud | Nome do banco de dados |
| APP_REDIS__HOST | localhost | Host do Redis |
| APP_REDIS__PORT | 6379 | Porta do Redis |
| APP_JWT__SECRET | - | Chave secreta do JWT (obrigatória) |

## Estrutura do Projeto

```
src/
├── handlers/      # Handlers HTTP (controllers)
├── services/     # Lógica de negócio
├── db/           # Camada de banco (SQLx)
├── cache/        # Camada de cache Redis
├── models/       # Modelos de dados
├── config/       # Configuração
├── errors/      # Tipos de erro
├── middlewares/  # Middlewares HTTP
└── utils/       # Utilitários
```

## Licença

MIT