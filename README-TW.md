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

SCTGDesk 是一款自架設的遠端桌面伺服器，擁有自身客戶端，源自[RustDesk](https://github.com/rustdesk/rustdesk).它是自由軟體，屬於**AGPL-3.0**.

> **SCTGDesk 是一個獨立專案。它與 RustDesk 專案並無關聯、背書或贊助
> 或 Purslane Ltd。**「RustDesk」是該專案的名稱;此處僅用來說明此軟體是什麼
> 衍生自並相容於。

這個儲存庫就是整個專案：伺服器、API 伺服器、網頁主控台以及用戶端覆蓋層。它取代
前任資料庫`sctgdesk-api-server`以及`sctgdesk`這些資料已被封存。

## 你得到的是什麼

*   RustDesk 伺服器：`hbbs`（ID 與會合伺服器）以及`hbbr`（中繼），加`rustdesk-utils`（金鑰對，
    連線檢查）。與官方 RustDesk 用戶端相容。
*   伺服器端的**加密的 TCP 與 WebSocket 連線**（金鑰交換版本 0 與 1），其中
    開源上游伺服器無法實作。這就是讓**網頁用戶端**工作。
*   A**REST API**嵌入於`hbbs`，與用戶端相容：登入（密碼，OpenID Connect / OAuth2：
    GitHub、GitLab、Google、Dex......）、能在重新啟動後存活的工作階段、個人與共享通訊錄（每位使用者及
    每組、唯讀/讀寫/管理員）、使用者與群組、軟體更新端點。
*   A**網頁主控台**管理所有這些，以及互動式 API 文件。
*   A**連接歷史**每個連線都有備註（完成了什麼），可搜尋並匯出成 CSV，由 RustDesk 用戶端填寫。
*   拒絕未驗證控制項的選項（`--logged-in-only`).
*   A**客戶**(`client/`）：上游 RustDesk 加上一個小型覆蓋層，在建置時以您的名稱、伺服器及
    key;以及**網頁用戶端**.

相容性是第一位：伺服器必須持續與官方 RustDesk 用戶端及我們上游的變更相容
檔案會保持較小，以便合併新的上游版本。

## 安裝

選一個。不管你選什麼，你都需要一個**金鑰對**以及下方的港口。

### Linux 套件（deb、rpm）及套件儲存庫

每個版本都有`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`以及`sctgdesk-server-utils`對於**amd64**,**arm64**以及**armv7**（32-bit ARM，`armhf`），如`.deb`以及`.rpm`，由靜態 musl 二進位檔建構（無函式庫依賴）。他們
取代前者`rustdesk-server-*`包裹。

**來自已簽署的資料庫**（系統更新）：

```bash
# Debian, Ubuntu
sudo mkdir -p /etc/apt/keyrings
sudo curl -fsSL https://sctg-development.github.io/sctgdesk-server/apt/sctgdesk.gpg -o /etc/apt/keyrings/sctgdesk.gpg
echo "deb [signed-by=/etc/apt/keyrings/sctgdesk.gpg] https://sctg-development.github.io/sctgdesk-server/apt stable main" |
  sudo tee /etc/apt/sources.list.d/sctgdesk.list
sudo apt update && sudo apt install sctgdesk-server-hbbs sctgdesk-server-hbbr

# Fedora, RHEL and clones: see docs/repositories.md for the .repo file, then
sudo dnf install sctgdesk-server-hbbs sctgdesk-server-hbbr
```

**或是從某個版本的檔案中：**

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

在空資料目錄上首次安裝時，會以隨機金鑰對啟動兩個服務;選項會放入`/etc/default/sctgdesk-hbbs`以及`-hbbr`（參見[部署指南](docs/deployment.md)，這也解釋了
從前者遷移`rustdesk-server-*`套件：**不要清除它們**).升級會重新啟動服務
正在執行的。資料已輸入`/var/lib/rustdesk-server`，登入`/var/log/rustdesk-server`;這些單元以 root 執行。

儲存庫與簽章金鑰的描述如下[套件儲存庫](docs/repositories.md).

### OpenWrt

適用於 OpenWrt 25.12 及更新版本 （`apk`），在 x86\_64、ARM（aarch64、armv7）及 MIPS（`mips_24kc`對於 ath79，`mipsel_24kc`對於
RAMIPS）路由器：

```sh
wget -O /etc/apk/keys/sctgdesk.rsa.pub https://sctg-development.github.io/sctgdesk-server/apk/sctgdesk.rsa.pub
. /etc/openwrt_release        # DISTRIB_ARCH: the architecture of the packages of your router, for example mips_24kc
echo "https://sctg-development.github.io/sctgdesk-server/apk/$DISTRIB_ARCH/APKINDEX.tar.gz" > /etc/apk/repositories.d/sctgdesk.list
apk update && apk add sctgdesk-server-hbbs sctgdesk-server-hbbr
```

每個計畫都是`procd`以 UCI 配置的服務（`/etc/config/sctgdesk-hbbs`).`hbbs`大約 30 MB：輸入，然後
其資料，儲存在 USB 磁碟或 extroot 上。MIPS 建置版本未經真實路由器測試。較舊的 OpenWrt 版本及其他版本
架構可以使用版本的靜態二進位檔。詳細資訊：[套件儲存庫](docs/repositories.md).

### 二進位檔（Linux、Windows、macOS）

該[發行作品](https://github.com/sctg-development/sctgdesk-server/releases)頁面也包含壓縮檔，具備`hbbs`,`hbbr`以及`rustdesk-utils`適用於 Windows（amd64、arm64）及 macOS（Intel、Apple Silicon），搭配`SHA256SUMS-*`檔案。
解壓縮後，從資料應該存放的目錄執行（參見[首次演出](#first-run)).

關於**Windows**它們可以安裝於以下方式`winget install SCTG.SCTGDeskServer --scope machine` and run as services
(`hbbs.exe --windows-service`, created with `sc.exe`): see [On Windows](docs/windows.md).

### Docker

Images are published on [Docker hub](https://hub.docker.com/r/sctg/sctgdesk-server/) as `sctg/sctgdesk-server`.
Save as `docker-compose.yml`, replacing `sctgdesk.example.com` with the address of your server:

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

For SELinux, append `:z` to the volume, or use `--security-opt label=disable`. Without `--net=host` direct peer-to-peer
connections cannot work (everything goes through the relay). A Helm chart is in `deploy/helm` (to be refactored) and
an example manifest in `kubernetes/`.

### Ports

| Port | Protocol | Used by | Purpose |
|---|---|---|---|
| 21114 | TCP | `hbbs` | REST API, web console, API documentation (`--api-port`) |
| 21115 | TCP | `hbbs` | NAT type test |
| 21116 | TCP + **UDP** | `hbbs` | ID / rendezvous |
| 21117 | TCP | `hbbr` | relay |
| 21118 | TCP | `hbbs` | WebSocket (web client) |
| 21119 | TCP | `hbbr` | WebSocket (web client) |

Open them on your firewall. The API and WebSocket ports should be behind a TLS reverse proxy on the Internet: see
the [deployment guide](docs/deployment.md).

## First run

`hbbs` and `hbbr` keep their data (key pair, SQLite database) in the **current directory**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

On the first start `hbbs` creates `id_ed25519` (secret) and `id_ed25519.pub` (the **public key**, which you give to
your clients). To make a key pair yourself, or to check one:

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

To refuse clients that do not have the key, start both servers with `-k <key>` (or `KEY=<key>`). Check that a
server answers on all its ports with `rustdesk-utils doctor <server>`.

**Default admin.** The web console is at `http://<server>:21114/` and the API documentation at
`http://<server>:21114/api/doc/`. A new database has the user `admin` with the password `Hello,world!`, which is
public: **change it at the first login**.

## Configuration

`hbbs` and `hbbr` take options on the command line (`hbbs --help`) and from environment variables, or an `.env`
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
