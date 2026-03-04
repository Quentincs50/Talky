# Talky
Application de messagerie instantanée avec serveurs et channel

## Stack technique 
- **Frontend** : React 19 / ViteJS / Tailwind CSS
- **Backend** : Rust / Axum
- **Bases de données** : PostgreSQL 17 + MongoDB 8
- **Infra** : Docker

## Prérequis
- Docker

## Installation

- cloner le repo
- installer les dépendances dans le dossier backend avec la commande "cargo install" et dans le dossier frontend avec la commande "npm install"
- créer le .env à la racine du projet avec les variables d'environnement suivantes :

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

- Ajouter un .env dans le frontend avec les variables d'environnement suivantes :

VITE_API_URL=http://localhost:3000
VITE_PORT=5173

- Lancer le projet avec docker compose up --build

### Ports et services
| Service     | URL                    |
|-------------|------------------------|
| Frontend    | http://localhost:5173  |
| Backend     | http://localhost:3000  |
| PostgreSQL  | localhost:5432         |
| MongoDB     | localhost:27017        |
