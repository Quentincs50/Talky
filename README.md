# Talky
Application de messagerie instantanée avec serveurs et channel

## Stack technique 
- **Frontend** : React 19 / ViteJS / Tailwind CSS
- **Backend** : Rust / Axum
- **Bases de données** : PostgreSQL 17 + MongoDB 8
- **Infrastructure** : Docker

## Prérequis
- [Docker](https://www.docker.com/) installé sur votre machine

## Installation

1. **Cloner le dépôt :**

```bash
git clone https://github.com/Quentincs50/Talky.git
```
2. **Installer les dépendances**
```bash
cd backend
cargo install
```
```bash
cd frontend
npm install
```
3. **Configurer les variables d'environnement**

```bash
cd backend
touch .env
```

```env
POSTGRES_USER                                                                                                                                                             
POSTGRES_PASSWORD                                                                                                                                                           
POSTGRES_DB                                                                                                                                                           
                                                                                                                                                                                       
MONGO_INITDB_ROOT_USERNAME                                                                                                                                                     
MONGO_INITDB_ROOT_PASSWORD                                                                                                                                                     
MONGO_INITDB_DATABASE                                                                                                                                                            
                                                                                                                                                                                       
DATABASE_URL=postgres://postgres:postgres@db:5432/app                                                                                                                                
MONGODB_URL=mongodb://mongo:mongo@mongo:27017/app?authSource=admin

JWT_SECRET
```

```bash
cd frontend
touch .env
```

```env
VITE_API_URL=http://localhost:3000
VITE_PORT=5173
```

```bash
docker compose up --build
```

### Ports et services

| Service    | URL                                            |
| ---------- | ---------------------------------------------- |
| Frontend   | [http://localhost:5173](http://localhost:5173) |
| Backend    | [http://localhost:3000](http://localhost:3000) |
| PostgreSQL | localhost:5432                                 |
| MongoDB    | localhost:27017                                |

