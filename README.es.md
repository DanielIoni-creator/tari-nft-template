# 🚀 Tari NFT Template

[![Docker Pulls](https://img.shields.io/docker/pulls/myzubster/tari-nft-template)](https://hub.docker.com/repository/docker/myzubster/tari-nft-template)
[![GitHub stars](https://img.shields.io/github/stars/DanielIoni-creator/tari-nft-template)](https://github.com/DanielIoni-creator/tari-nft-template/stargazers)
[![License: BSD-3-Clause](https://img.shields.io/badge/License-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

🌍 **Idiomas**: [English](README.md) | [Italiano](README.it.md) | [Français](README.fr.md) | [Español](README.es.md)

**Plantilla NFT completa para Tari – la sidechain privacy-by-default de Monero.**

Escrito en **Rust**, compilado a **WASM**, dockerizado y listo para desplegar en la red Tari.

---

## ✨ Características

- ✅ **Crear una colección NFT** – `new(name, symbol)`
- ✅ **Mintear NFTs** – `mint(id, immutable_data, mutable_data)`
- ✅ **Metadatos inmutables** – no se pueden cambiar después del mint (ej. URL de la imagen)
- ✅ **Metadatos mutables** – pueden ser cambiados por el propietario (ej. estado)
- ✅ **Escrito en Rust** – usando `tari_template_lib` v0.29.0
- ✅ **Compilado a WASM** – listo para desplegar en Tari
- ✅ **Dockerizado** – imagen pública en Docker Hub
- ✅ **Open Source** – BSD-3-Clause

---

## 🛠️ Tecnologías

| Tecnología | Descripción |
| :--- | :--- |
| **Rust** 1.97+ | Lenguaje de programación |
| **Tari** | Sidechain privacy-by-default de Monero |
| **WASM** | WebAssembly – formato ejecutable para Tari |
| **Docker** | Contenerización y despliegue |
| **GitHub** | Control de versiones y open source |

---

## 📦 Instalación

### Con Rust (compilación local)

```bash
git clone https://github.com/DanielIoni-creator/tari-nft-template.git
cd tari-nft-template
cargo build
cargo test
Con Docker (imagen lista para usar)
bash

docker pull myzubster/tari-nft-template:latest
docker run --rm myzubster/tari-nft-template:latest
docker create --name temp myzubster/tari-nft-template:latest
docker cp temp:/app/my_first_nft.wasm .
docker rm temp
🐳 Docker
bash

docker build -t tari-nft-template .
docker tag tari-nft-template:latest myzubster/tari-nft-template:latest
docker push myzubster/tari-nft-template:latest

Imagen pública: https://hub.docker.com/repository/docker/myzubster/tari-nft-template
🔗 Enlaces Útiles
Recurso	Enlace
GitHub	https://github.com/DanielIoni-creator/tari-nft-template
Docker Hub	https://hub.docker.com/repository/docker/myzubster/tari-nft-template
Artículo Dev.to	https://dev.to/danielioni/i-built-an-nft-template-for-tari-monero-sidechain-heres-how-33k
Documentación Tari	https://tari.com/lessons
API Tari (Rust)	https://docs.rs/tari_template_lib
📄 Licencia

BSD-3-Clause
🤝 Contribuir

    Haz un fork del repositorio

    Crea una rama (git checkout -b feature/amazing-feature)

    Haz commit (git commit -m 'Add some amazing feature')

    Haz push (git push origin feature/amazing-feature)

    Abre una Pull Request

⭐ Apoya el Proyecto

¡Deja una estrella en GitHub! ⭐
🌐 Conéctate conmigo

Sigue el desarrollo de MyZubster y conéctate conmigo en redes sociales:

    📖 Blog y Artículos: DEV.to - Daniel Ioni

    🐦 X (Twitter): @myzubster

    💼 LinkedIn: Daniel Ioni

    🐙 GitHub: DanielIoni-creator

    🎵 TikTok: @h4x0r_23

¡Mantente al tanto del viaje! 🚀

Construido con ❤️ para la comunidad de Monero y Tari.