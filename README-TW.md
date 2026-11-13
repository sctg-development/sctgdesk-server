<p align="center">
  <a href="#install">Install</a> •
  <a href="#first-run">First run</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#clients">Clients</a> •
  <a href="#build-it-yourself">Build</a> •
  <a href="https://sctg-development.github.io/sctgdesk-server/">Manual</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>] | [<a href="README-RU.md">Русский</a>]<br>
</p>

# SCTGDesk

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![CI](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml)

SCTGDesk 是一台自架的遠端桌面伺服器，擁有自己的客戶端，源自[鏽桌](https://github.com/rustdesk/rustdesk).它是自由軟體，屬於**AGPL-3.0**.

> **SCTGDesk 是一個獨立專案。它與 RustDesk 專案無關聯、不被其背書或贊助
> 或稱 Purslane Ltd。**「RustDesk」是該專案的名稱;此處僅用來說明這軟體是什麼
> 衍生自並相容於。

這個儲存庫包含整個專案：伺服器、API 伺服器、網頁主控台和用戶端覆蓋層。它取代了
前任倉庫`sctgdesk-api-server`以及`sctgdesk`這些資料已被存檔。

## 你得到的是什麼

*   RustDesk 伺服器：`hbbs`（ID 與約會伺服器）以及`hbbr`（中繼），加`rustdesk-utils`（鑰匙對，
    連線檢查）。與官方 RustDesk 用戶端相容。
*   伺服器端**加密的 TCP 與 WebSocket 連線**（金鑰交換版本 0 與 1），其中
    開源上游伺服器不實作。這就是使**網頁用戶端**工作。
*   A**REST API**嵌入於`hbbs`，與用戶端相容：登入（密碼，OpenID Connect / OAuth2：
    GitHub、GitLab、Google、Dex......）、重啟後仍存的工作階段、個人與共享通訊錄（每位使用者及
    每組，唯讀/讀寫/管理員）、使用者與群組，軟體更新端點。
*   A**網頁主控台**管理這些，以及互動式 API 文件。
*   A**連接歷史**每個連線都會附上備註（完成了什麼），可搜尋並匯出成 CSV，由 RustDesk 用戶端填寫。
*   拒絕未經認證控制的選項（`--logged-in-only`).
*   A**客戶端**(`client/`）：上游的 RustDesk 加上一個小型覆蓋層，在建置時標示你的名稱、伺服器和
    鍵;以及**網頁用戶端**.

相容性優先：伺服器必須持續與官方 RustDesk 客戶端運作，並持續進行上游的變更
檔案會保持小巧，以便合併新的上游版本。

## 安裝

選一個。不管你選什麼，都需要一個**金鑰對**以及下方的港口。

### Linux 套件（deb、rpm）

每個版本都有`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`以及`sctgdesk-server-utils`對於**AMD64**以及**arm64**，作為`.deb`以及`.rpm`，由靜態 musl 二進位構成（無函式庫依賴）。它們取代了前者`rustdesk-server-*`包裹。

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

首次安裝於空資料目錄時，會以隨機的金鑰對啟動兩個服務;選項會被放入`/etc/default/sctgdesk-hbbs`以及`-hbbr`（參見[部署指南](docs/deployment.md)，這也解釋了
從前者遷移`rustdesk-server-*`套裝：**不要清除它們**).升級後服務會重新啟動
正在運行。資料已輸入`/var/lib/rustdesk-server`，日誌`/var/log/rustdesk-server`;這些單元以 root 方式執行。

### 二進位（Linux、Windows、macOS）

該[發行](https://github.com/sctg-development/sctgdesk-server/releases)頁面也有壓縮檔`hbbs`,`hbbr`以及`rustdesk-utils`適用於 Windows（amd64、arm64）及 macOS（Intel、Apple Silicon），且`SHA256SUMS-*`檔案。
解壓後，從資料應該存放的目錄執行它們（參見[首次運行](#first-run)).

### Docker

圖片刊登於[Docker 樞紐](https://hub.docker.com/r/sctg/sctgdesk-server/)作為`sctg/sctgdesk-server`.
另存為`docker-compose.yml`，取代`sctgdesk.example.com`附上你的伺服器地址：

```yaml
networks:
  sctgdesk-net:
    external: false

services:
  hbbs:
    container_name: hbbs
    ports:
      - 21114:21114
      - 21115:21115
      - 21116:21116
      - 21116:21116/udp
      - 21118:21118
    image: sctg/sctgdesk-server:latest
    command: hbbs -r sctgdesk.example.com:21117
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    depends_on:
      - hbbr
    restart: unless-stopped

  hbbr:
    container_name: hbbr
    ports:
      - 21117:21117
      - 21119:21119
    image: sctg/sctgdesk-server:latest
    command: hbbr
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    restart: unless-stopped
```

```bash
mkdir -p data && docker compose up -d
```

關於 SELinux，請附錄`:z`到體積或使用`--security-opt label=disable`.沒有`--net=host`直接點對點
連線無法正常運作（所有東西都經過繼電器）。Helm 圖表已輸入`deploy/helm`（待重構）以及
一個例子`kubernetes/`.

### 移植版本

|埠 |協定 |使用 |目的 |
|---|---|---|---|
|21114 |TCP |`hbbs`|REST API、網頁控制台、API 文件（`--api-port`) |
|21115 |TCP |`hbbs`|NAT 類型測試 |
|21116 |TCP +**UDP**|`hbbs`|身分證 / 約會地點 |
|21117 |TCP |`hbbr`|接力 |
|21118 |TCP |`hbbs`|WebSocket（網頁客戶端） |
|21119 |TCP |`hbbr` | WebSocket (web client) |

API 文件[在客戶端](docs/deployment.md)安全性

## 嵌入式 API 伺服器沒有防止濫用：在前面放置一個 TLS 反向代理（HAProxy、NGINX、Caddy 等）&#xA;以及 WebSocket 埠口。HAProxy 的一個範例是

`hbbs`部署指南`hbbr`.**更改預設的管理員密碼，保持不變**秘密並備份：改變它會讓每個客戶都拒絕
你的伺服器。

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

如描述的漏洞報告`hbbs`SECURITY.md`id_ed25519`.`id_ed25519.pub`授權與原始碼**SCTGDesk 以**GNU Affero 通用公共授權條款 v3

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

.每個版本的原始碼
就在這個倉庫裡，該版本的標籤中。如果你修改了這個軟體，並讓其他人使用它，而不是
Network，授權條款第 13 條要求你提供修改版本的對應原始碼。`-k <key>`在執行下載的二進位檔前，請檢查每次發行時公布的 SHA-256 總和。`KEY=<key>`貢獻`rustdesk-utils doctor <server>`開發者手冊

**·**CONTRIBUTING.md`http://<server>:21114/`·`http://<server>:21114/api/doc/`CODE_OF_CONDUCT.md`admin`·`Hello,world!`SECURITY.md**.關於一般自家主機的問題，請參考**RustDesk 文件

## 同樣適用。

`hbbs`如果你喜歡這部作品，請考慮給它一顆星，或`hbbr`贊助`hbbs --help`.`.env`
file.

| variable | binary | description |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | if set to **"Y"** disallows direct peer connection |
| DB_URL | hbbs | path of the SQLite database (default `./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | hbbs | if set to **"Y"** only logged-in users can control a peer (same as `--logged-in-only`) |
| ALLOW_REMOTE_BACKUP | hbbs | if set to **"Y"**, an administrator can download a backup (database, key pair, configuration) with `GET /api/backup` or from the console (same as `--allow-remote-backup`); off by default, see [the deployment guide](docs/deployment.md#download-a-backup) |
| REQUIRE_KX_V1 | hbbs | if set to **"Y"** refuses clients that use the legacy (version 0) TCP key exchange (same as `--require-kx-v1`) |
| DOWNGRADE_START_CHECK | hbbr | delay (in seconds) before downgrade check |
| DOWNGRADE_THRESHOLD | hbbr | threshold of downgrade check (bit/ms) |
| KEY | hbbs/hbbr | if set force the use of a specific key, if set to **"\_"** force the use of any key |
| LIMIT_SPEED | hbbr | speed limit (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | path of the OAuth2 / OpenID Connect providers file (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | hbbs | if set to **"1"** create a user at the first login with a provider |
| PORT | hbbs/hbbr | listening port (21116 for hbbs - 21117 for hbbr) |
| HEALTH_PORT | hbbr | if set, hbbr answers `GET /livez` and `GET /readyz` (HTTP) on this port; off by default |
| RELAY | hbbs | IP address/DNS name of the machines running hbbr (separated by comma) |
| LOG_FORMAT | hbbs, hbbr | **"json"**: one JSON object per log line (`time`, `level`, `target`, `file`, `line`, `msg`); default: text |
| RUST_LOG | all | set debug level (error|warn|info|debug|trace) |
| ROCKET_SECRET_KEY | hbbs | key that signs the cookies of the API server (base64, 64 bytes). Default: generated at the first start in `secret_key.txt` next to the database |
| S3CONFIG_FILE | hbbs | path of the software-update file (`s3config.toml`) |
| SINGLE_BANDWIDTH | hbbr | max bandwidth for a single connection (in Mb/s) |
| TOTAL_BANDWIDTH | hbbr | max total bandwidth (in Mb/s) |

### Only logged-in users can control

`--logged-in-only` (or `LOGGED_IN_ONLY=Y`) makes `hbbs` ask the API server whether the token of the client is a live
session before it connects two peers. Anyone can still register their device, but nobody can control a device
without logging in. A client that is not logged in sees "The connection is not allowed. You have not logged in."; one
whose session ended sees "Your session expired."

### Login with GitHub, GitLab, Google, Dex...

Copy `oauth2.toml` and fill in the providers you use (each needs an application registered with the provider, whose
callback is your API server), then set `OAUTH2_CONFIG_FILE=oauth2.toml`, and `OAUTH2_CREATE_USER=1` to create the
user at the first login. See the [deployment guide](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Updates of the client

The clients can look for new versions on your API server instead of GitHub: it answers
`POST /api/software/version/latest` (the check of current RustDesk clients) and `GET /api/software/releases/latest`.
The latest version and the download links are read from `s3config.toml` (S3-compatible storage), see
[the REST API](docs/api.md#software-update-endpoints).

## Clients

*   **Official RustDesk clients** work with this server: give them your server and your public key.
*   **The SCTGDesk client** is upstream RustDesk plus our overlay (name, icons, default servers and key). You build it
    for your own servers, see [custom builds](docs/custom-builds.md#the-client). The **web client** is built from
    the same overlay.

## Build it yourself

You need a Rust toolchain and, for the web console, Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Set `SCTGDESK_SKIP_WEB_BUILD=1` to build without Node.js (the console is then a placeholder). The
[developer manual](https://sctg-development.github.io/sctgdesk-server/) covers building, testing, custom server and
client builds, packaging, releases and syncing with upstream.

## Screenshots

### Webconsole

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### Api documentation

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### In the client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Security

*   The embedded API server is not protected against abuse: put a TLS reverse proxy (HAProxy, NGINX, Caddy...) in front
    of it, and of the WebSocket ports. A HAProxy example is in the [deployment guide](docs/deployment.md#tls-reverse-proxy).
*   Change the default admin password, keep `id_ed25519` secret and back it up: changing it makes every client reject
    your server.
*   Report vulnerabilities as described in [SECURITY.md](SECURITY.md).

## License and source code

SCTGDesk is distributed under the [GNU Affero General Public License v3](LICENSE). The source code of every release
is in this repository, at the tag of that release. If you modify this software and let other people use it over a
network, section 13 of the license requires you to offer them the corresponding source code of your modified version.

Check the SHA-256 sums published with each release before running a downloaded binary.

## Contributing

[Developer manual](https://sctg-development.github.io/sctgdesk-server/) · [CONTRIBUTING.md](CONTRIBUTING.md) ·
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) · [SECURITY.md](SECURITY.md). For general self-hosting questions, the
[RustDesk documentation](https://rustdesk.com/docs/en/self-host/) also applies.

If you like this work, consider giving it a star, or
[sponsoring it](https://github.com/sponsors/sctg-development).
