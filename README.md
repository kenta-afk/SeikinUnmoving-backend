If you clone this repository, please command $lefthook install.

## Docker Setup

### Prerequisites
- Docker
- Docker Compose

### Running with Docker

Start all services (apiroute, PostgreSQL 18, Valkey 9.0):
```bash
docker compose up -d
```

Stop all services:
```bash
docker compose down
```

View logs:
```bash
docker compose logs -f
```

### Services
- apiroute: http://localhost:8080
- PostgreSQL: localhost:5432 (user: seikin_user, db: seikin)
- Valkey: localhost:6379
- migration: Runs once on startup to execute SQL files in `db/init/`

### Database Migrations
The migration container automatically runs all SQL files in `db/init/` directory in alphabetical order when you run `docker compose up -d`. The container exits after completing migrations.

To add new migrations, create SQL files in `db/init/` (e.g., `02_add_users_table.sql`).

To re-run migrations:
```bash
docker compose up migration
```