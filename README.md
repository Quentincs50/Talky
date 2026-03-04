| # Talky | Stack Technique |
|------------|-------------------|
| **Talky** – Application de messagerie instantanée avec serveurs et channels pour discuter en temps réel. | **Frontend**: ![React](https://img.shields.io/badge/React-61DAFB?style=for-the-badge&logo=react&logoColor=white) ![Vite](https://img.shields.io/badge/Vite-646CFF?style=for-the-badge&logo=vite&logoColor=white) ![TailwindCSS](https://img.shields.io/badge/TailwindCSS-06B6D4?style=for-the-badge&logo=tailwind-css&logoColor=white) <br> **Backend**: ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-ffffff?style=for-the-badge) <br> **Databases**: ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white) ![MongoDB](https://img.shields.io/badge/MongoDB-47A248?style=for-the-badge&logo=mongodb&logoColor=white) <br> **Infrastructure**: ![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white) |

---


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
4. **Lancer le projet avec Docker Compose:
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

