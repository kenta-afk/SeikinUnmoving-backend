If you clone this repository, please command $lefthook install.

## Docker Setup

### Prerequisites
- Docker
- Docker Compose

### Running with Docker

Start all services (apiroute, Valkey 9.0):
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
- Valkey: localhost:6379

### Database
This project uses SQLite as the database. The database file is stored in a Docker volume and will persist across container restarts.