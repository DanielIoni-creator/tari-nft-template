\# 🚀 Tari NFT Template



\[!\[Docker Pulls](https://img.shields.io/docker/pulls/myzubster/tari-nft-template)](https://hub.docker.com/repository/docker/myzubster/tari-nft-template)

\[!\[GitHub stars](https://img.shields.io/github/stars/DanielIoni-creator/tari-nft-template)](https://github.com/DanielIoni-creator/tari-nft-template/stargazers)

\[!\[License: BSD-3-Clause](https://img.shields.io/badge/License-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)



🌍 \*\*Langues\*\*: \[English](README.md) | \[Italiano](README.it.md) | \[Français](README.fr.md) | \[Deutsch](README.de.md)



\*\*Template NFT complet pour Tari – la sidechain privacy-by-default de Monero.\*\*



Écrit en \*\*Rust\*\*, compilé en \*\*WASM\*\*, dockerisé et prêt pour le déploiement sur le réseau Tari.



\---



\## ✨ Fonctionnalités



\- ✅ \*\*Création d'une collection NFT\*\* – `new(name, symbol)`

\- ✅ \*\*Minting de NFT\*\* – `mint(id, immutable\_data, mutable\_data)`

\- ✅ \*\*Métadonnées immuables\*\* – non modifiables après le mint (ex. URL de l'image)

\- ✅ \*\*Métadonnées mutables\*\* – modifiables par le propriétaire (ex. état)

\- ✅ \*\*Écrit en Rust\*\* – avec `tari\_template\_lib` v0.29.0

\- ✅ \*\*Compilé en WASM\*\* – prêt pour le déploiement sur Tari

\- ✅ \*\*Dockerisé\*\* – image publique sur Docker Hub

\- ✅ \*\*Open Source\*\* – BSD-3-Clause



\---



\## 🛠️ Technologies



| Technologie | Description |

| :--- | :--- |

| \*\*Rust\*\* 1.97+ | Langage de programmation |

| \*\*Tari\*\* | Sidechain privacy-by-default de Monero |

| \*\*WASM\*\* | WebAssembly – format exécutable pour Tari |

| \*\*Docker\*\* | Containerisation et déploiement |

| \*\*GitHub\*\* | Versionnement et open source |



\---



\## 📦 Installation



\### Avec Rust (compilation locale)



```bash

git clone https://github.com/DanielIoni-creator/tari-nft-template.git

cd tari-nft-template

cargo build

cargo testAvec Docker (image prête)

bash



docker pull myzubster/tari-nft-template:latest

docker run --rm myzubster/tari-nft-template:latest

docker create --name temp myzubster/tari-nft-template:latest

docker cp temp:/app/my\_first\_nft.wasm .

docker rm temp



🐳 Docker

bash



docker build -t tari-nft-template .

docker tag tari-nft-template:latest myzubster/tari-nft-template:latest

docker push myzubster/tari-nft-template:latest



Image publique: https://hub.docker.com/repository/docker/myzubster/tari-nft-template

🔗 Liens Utiles

Ressource	Lien

GitHub	https://github.com/DanielIoni-creator/tari-nft-template

Docker Hub	https://hub.docker.com/repository/docker/myzubster/tari-nft-template

Article Dev.to	https://dev.to/danielioni/i-built-an-nft-template-for-tari-monero-sidechain-heres-how-33k

Documentation Tari	https://tari.com/lessons

API Tari (Rust)	https://docs.rs/tari\_template\_lib

📄 Licence



BSD-3-Clause

🤝 Contribuer



&#x20;   Fork le repository



&#x20;   Crée une branche (git checkout -b feature/amazing-feature)



&#x20;   Commit (git commit -m 'Add some amazing feature')



&#x20;   Push (git push origin feature/amazing-feature)



&#x20;   Ouvre une Pull Request



⭐ Soutenir le Projet



Laisse une étoile sur GitHub ! ⭐

🌐 Connecte-toi avec Moi



Suis le développement de MyZubster et connecte-toi avec moi sur les réseaux sociaux :



&#x20;   📖 Blog \& Articles: DEV.to - Daniel Ioni



&#x20;   🐦 X (Twitter): @myzubster



&#x20;   💼 LinkedIn: Daniel Ioni



&#x20;   🐙 GitHub: DanielIoni-creator



&#x20;   🎵 TikTok: @h4x0r\_23



Restez informé sur le parcours ! 🚀

Construit avec ❤️ pour la communauté Monero et Tari.

