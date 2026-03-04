<a id="readme-top"></a>

<div align="center">
  <a href="https://github.com/Quentincs50/Talky>
    <img src="images/logo.png" alt="Logo" width="30" height="30">
  </a>
  <h3 align="center">Talky</h3>

  <p align="center">
    Application de messagerie instantanée avec serveurs et channels pour discuter en temps réel.
    <br />
    <a href="https://github.com/Quentincs50/DevLab"><strong>Explore my other projects»</strong></a>
    <br />
    <br />
    <a href="quentin-sanchez.vercel.app">View Demo</a>
    &middot;
    <a href=""/>Linkedin</a>
    &middot;
  </p>
</div>

<!-- SOMMAIRE -->
<details>
  <summary>Table de contenu</summary>
  <ol>
    <li>
      <a href="#le-projet">A propos du projet</a>
      <ul>
        <li><a href="#stack-technique">Stack Technique</a></li>
      </ul>
    </li>
    <li>
      <a href="#commencement">Commencer</a>
      <ul>
        <li><a href="#prerequis">Prerequis</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#port-et-services">Port et Services</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- A Propos du Projet -->
## Le Projet

Talky est une messagerie  Discord like dans le but d'apprendre plus en profondeur la gestion des sockets et l'API en rust. 

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Stack Technique

| Technologie | Badge / Icon |
|------------|--------------|
| **Frontend** | ![React](https://img.shields.io/badge/React-61DAFB?style=for-the-badge&logo=react&logoColor=white) ![Vite](https://img.shields.io/badge/Vite-646CFF?style=for-the-badge&logo=vite&logoColor=white) ![TailwindCSS](https://img.shields.io/badge/TailwindCSS-06B6D4?style=for-the-badge&logo=tailwind-css&logoColor=white) |
| **Backend**  | ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-ffffff?style=for-the-badge) |
| **Databases** | ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white) ![MongoDB](https://img.shields.io/badge/MongoDB-47A248?style=for-the-badge&logo=mongodb&logoColor=white) |
| **Infrastructure** | ![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white) |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Commencement

Voici les instructions à suivre pour commmencer le projet

### Prérequis
- [Docker](https://www.docker.com/) installé sur votre machine

### Installation

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
4. **Lancer le projet avec Docker Compose :**
```bash
docker compose up --build
```
<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Ports et services

| Service    | URL                                            |
| ---------- | ---------------------------------------------- |
| Frontend   | [http://localhost:5173](http://localhost:5173) |
| Backend    | [http://localhost:3000](http://localhost:3000) |
| PostgreSQL | localhost:5432                                 |
| MongoDB    | localhost:27017                                |

## Contact

Your Name - [@Quentin_Sanchez](https://www.linkedin.com/in/quentin-sanchez-9b6741b6) - Linkedin

Project Link: [https://github.com/Quentincs50/DevLab](https://github.com/Quentincs50/DevLab)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

Use this space to list resources you find helpful and would like to give credit to. I've included a few of my favorites to kick things off!

* [Aceternity](https://ui.aceternity.com/)
* [MagicUi](https://magicui.design/)
* [SocketIO example](https://github.com/Totodore/socketioxide/blob/main/examples)
* [Icons](tabler.io/icons)
* [Javascript Mastery Video](https://www.youtube.com/@javascriptmastery)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

