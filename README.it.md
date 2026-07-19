\# 🚀 Tari NFT Template



\[!\[Docker Pulls](https://img.shields.io/docker/pulls/myzubster/tari-nft-template)](https://hub.docker.com/repository/docker/myzubster/tari-nft-template)

\[!\[GitHub stars](https://img.shields.io/github/stars/DanielIoni-creator/tari-nft-template)](https://github.com/DanielIoni-creator/tari-nft-template/stargazers)

\[!\[License: BSD-3-Clause](https://img.shields.io/badge/License-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)



🌍 \*\*Lingue\*\*: \[English](README.md) | \[Italiano](README.it.md) | \[Français](README.fr.md) | \[Deutsch](README.de.md)



\*\*Template NFT completo per Tari – la sidechain privacy-by-default di Monero.\*\*



Scritto in \*\*Rust\*\*, compilato in \*\*WASM\*\*, dockerizzato e pronto per il deploy sulla rete Tari.



\---



\## ✨ Funzionalità



\- ✅ \*\*Creazione di una collezione NFT\*\* – `new(name, symbol)`

\- ✅ \*\*Minting di NFT\*\* – `mint(id, immutable\_data, mutable\_data)`

\- ✅ \*\*Metadata immutabili\*\* – non modificabili dopo il mint (es. URL immagine)

\- ✅ \*\*Metadata mutabili\*\* – modificabili dal proprietario (es. stato)

\- ✅ \*\*Scritto in Rust\*\* – con `tari\_template\_lib` v0.29.0

\- ✅ \*\*Compilato in WASM\*\* – pronto per il deploy su Tari

\- ✅ \*\*Dockerizzato\*\* – immagine pubblica su Docker Hub

\- ✅ \*\*Open Source\*\* – BSD-3-Clause



\---



\## 🛠️ Tecnologie



| Tecnologia | Descrizione |

| :--- | :--- |

| \*\*Rust\*\* 1.97+ | Linguaggio di programmazione |

| \*\*Tari\*\* | Sidechain privacy-by-default di Monero |

| \*\*WASM\*\* | WebAssembly – formato eseguibile per Tari |

| \*\*Docker\*\* | Containerizzazione e deploy |

| \*\*GitHub\*\* | Versionamento e open source |



\---



\## 📦 Installazione



\### Con Rust (compilazione locale)



```bash

git clone https://github.com/DanielIoni-creator/tari-nft-template.git

cd tari-nft-template

cargo build

cargo test

Con Docker (immagine pronta)

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



Immagine pubblica: https://hub.docker.com/repository/docker/myzubster/tari-nft-template

🔗 Link Utili

Risorsa	Link

GitHub	https://github.com/DanielIoni-creator/tari-nft-template

Docker Hub	https://hub.docker.com/repository/docker/myzubster/tari-nft-template

Articolo Dev.to	https://dev.to/danielioni/i-built-an-nft-template-for-tari-monero-sidechain-heres-how-33k

Documentazione Tari	https://tari.com/lessons

API Tari (Rust)	https://docs.rs/tari\_template\_lib

📄 Licenza



BSD-3-Clause

🤝 Contribuire



&#x20;   Fork il repository



&#x20;   Crea un branch (git checkout -b feature/amazing-feature)



&#x20;   Commit (git commit -m 'Add some amazing feature')



&#x20;   Push (git push origin feature/amazing-feature)



&#x20;   Apri una Pull Request



⭐ Supporta il Progetto



Lascia una stella su GitHub! ⭐

🌐 Connettiti con Me



Segui lo sviluppo di MyZubster e connettiti con me sui social:



&#x20;   📖 Blog \& Articoli: DEV.to - Daniel Ioni



&#x20;   🐦 X (Twitter): @myzubster



&#x20;   💼 LinkedIn: Daniel Ioni



&#x20;   🐙 GitHub: DanielIoni-creator



&#x20;   🎵 TikTok: @h4x0r\_23



Rimani aggiornato sul viaggio! 🚀



Costruito con ❤️ per la community di Monero e Tari.



