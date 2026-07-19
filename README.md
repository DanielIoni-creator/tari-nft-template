# 🚀 Tari NFT Template

[![Docker Pulls](https://img.shields.io/docker/pulls/myzubster/tari-nft-template)](https://hub.docker.com/repository/docker/myzubster/tari-nft-template)
[![GitHub stars](https://img.shields.io/github/stars/DanielIoni-creator/tari-nft-template)](https://github.com/DanielIoni-creator/tari-nft-template/stargazers)
[![License: BSD-3-Clause](https://img.shields.io/badge/License-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

🌍 **Languages**: [English](README.md) | [Italiano](README.it.md) | [Français](README.fr.md) | [Deutsch](README.de.md)

**Complete NFT template for Tari – Monero's privacy-by-default sidechain.**

Written in **Rust**, compiled to **WASM**, dockerized, and ready for deployment on the Tari network.

---

## ✨ Features

- ✅ **Create an NFT collection** – `new(name, symbol)`
- ✅ **Mint NFTs** – `mint(id, immutable_data, mutable_data)`
- ✅ **Immutable metadata** – cannot be changed after mint (e.g., image URL)
- ✅ **Mutable metadata** – can be changed by the owner (e.g., status)
- ✅ **Written in Rust** – using `tari_template_lib` v0.29.0
- ✅ **Compiled to WASM** – ready for Tari deployment
- ✅ **Dockerized** – public image on Docker Hub
- ✅ **Open Source** – BSD-3-Clause

---

## 🛠️ Tech Stack

| Technology | Description |
| :--- | :--- |
| **Rust** 1.97+ | Programming language |
| **Tari** | Privacy-by-default sidechain of Monero |
| **WASM** | WebAssembly – executable format for Tari |
| **Docker** | Containerization and deployment |
| **GitHub** | Version control and open source |

---

## 📦 Installation

### With Rust (local compilation)

```bash
git clone https://github.com/DanielIoni-creator/tari-nft-template.git
cd tari-nft-template
cargo build
cargo test
With Docker (ready-to-use image)
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

Public Image: https://hub.docker.com/repository/docker/myzubster/tari-nft-template
🔗 Useful Links
Resource	Link
GitHub	https://github.com/DanielIoni-creator/tari-nft-template
Docker Hub	https://hub.docker.com/repository/docker/myzubster/tari-nft-template
Dev.to Article	https://dev.to/danielioni/i-built-an-nft-template-for-tari-monero-sidechain-heres-how-33k
Tari Documentation	https://tari.com/lessons
Tari API (Rust)	https://docs.rs/tari_template_lib
📄 License

BSD-3-Clause
🤝 Contributing

    Fork the repository

    Create a branch (git checkout -b feature/amazing-feature)

    Commit (git commit -m 'Add some amazing feature')

    Push (git push origin feature/amazing-feature)

    Open a Pull Request

⭐ Support the Project

Leave a star on GitHub! ⭐
🌐 Connect with Me

Follow the development of MyZubster and connect with me on social media:

    📖 Blog & Articles: DEV.to - Daniel Ioni

    🐦 X (Twitter): @myzubster

    💼 LinkedIn: Daniel Ioni

    🐙 GitHub: DanielIoni-creator

    🎵 TikTok: @h4x0r_23

Stay updated on the journey! 🚀

Built with ❤️ for the Monero and Tari community.