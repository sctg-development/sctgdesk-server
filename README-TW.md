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

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)並以服務形式運行
([![CI](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml)

，以 創建[）： 參見](https://github.com/rustdesk/rustdesk)在 Windows 上**.**Docker

> **圖片發佈於**Docker 樞紐

如`sctgdesk-api-server`.
另存為`sctgdesk`，取代

## 附上您的伺服器地址：

*   對於 SELinux，請附加`hbbs`到卷，或使用`hbbr`.沒有`rustdesk-utils`直接點對點
    連線無法正常運作（所有設備都經過繼電器）。舵手圖已在內
*   （待重構）以及
    範例顯示於**.**移植版本**|埠 |協定 |使用者 |目的 |
    |---|---|---|---|
    |21114 |TCP |**|REST API、網頁控制台、API 文件（
*   ) |
    |21115 |TCP |**|NAT 類型測試 |
    |21116 |TCP +**UDP`hbbs`|
*   |ID / 會合 |
    |21117 |TCP |**|中繼 |
    |21118 |TCP |**|WebSocket（網頁用戶端）|
    |21119 |TCP |
*   |WebSocket（網頁用戶端）|**請在防火牆上開啟它們。API 和 WebSocket 連接埠應該位於網際網路上的 TLS 反向代理後方：請參閱
    該**部署指南
*   .`--logged-in-only`首次演出
*   以及**將他們的資料（金鑰對、SQLite 資料庫）保存在**現行目錄`client/`.**首次起跑**創造

（秘密） 以及

## （

公鑰**，你將它交給
您的客戶）。若要自行建立金鑰配對，或檢查其中一組：**若要拒絕沒有金鑰的用戶端，請以以下方式啟動兩台伺服器

### （或

).檢查一下
伺服器在所有連接埠上回應如下：`sctgdesk-server-hbbs`.`sctgdesk-server-hbbr`預設管理員。`sctgdesk-server-utils`網頁主控台位於**以及 API 文件**.一個新的資料庫包含使用者**使用密碼**，即
公眾：**首次登入時更改**.`armhf`配置`.deb`以及`.rpm`在命令列上取得選項（`rustdesk-server-*`）以及來自環境變數，或

**檔案。**|變數 |二進位 |說明 |
|--- |--- |--- |
|ALWAYS_USE_RELAY |hbbs |如果設定為

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

**「Y」**

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

禁止直接連線 |
|DB_URL |hbbs |SQLite 資料庫路徑（預設）`/etc/default/sctgdesk-hbbs`) |
|LOGGED_IN_ONLY |hbbs |如果設定為`-hbbr`「Y」[只有已登入的使用者才能控制對等端（與](docs/deployment.md)) |
|ALLOW_REMOTE_BACKUP |hbbs |如果設定為`rustdesk-server-*`「Y」**，管理員可以下載備份（資料庫、金鑰對、設定），並搭配**或從主控台（與`/var/lib/rustdesk-server`);預設關閉，請參閱`/var/log/rustdesk-server`部署指南

|
|REQUIRE_KX_V1 |hbbs |如果設定為[「Y」](docs/repositories.md)拒絕使用舊版（版本 0）TCP 金鑰交換的用戶端（與

### ) |&#xA;|DOWNGRADE_START_CHECK |HBBR |降級檢查前的延遲（秒數） |&#xA;|DOWNGRADE_THRESHOLD |HBBR |降級檢查門檻（bit/ms） |&#xA;|KEY |hbbs/hbbr |如果設定為 ，則強制使用特定鍵，若設定為

"\_"`apk`強制使用任意金鑰 |
|LIMIT_SPEED |hbbr |速限（以 Mb/s） |
|OAUTH2\_CONFIG_FILE |hbbs |OAuth2 / OpenID Connect 提供者檔案的路徑（`mips_24kc`) |
|OAUTH2\_CREATE_USER |hbbs |如果設定為`mipsel_24kc`"1"

```sh
wget -O /etc/apk/keys/sctgdesk.rsa.pub https://sctg-development.github.io/sctgdesk-server/apk/sctgdesk.rsa.pub
echo "https://sctg-development.github.io/sctgdesk-server/apk/$(apk --print-arch)/APKINDEX.tar.gz" > /etc/apk/repositories.d/sctgdesk.list
apk update && apk add sctgdesk-server-hbbs sctgdesk-server-hbbr
```

首次登入時使用提供者建立使用者 |
|埠 |HBBS/HBBR |聆聽埠（HBBS 為 21116 - HBBR 為 21117）|
|HEALTH_PORT |HBBR |如果設定，HBBR 會回應`procd`以及`/etc/config/sctgdesk-hbbs`（HTTP）在此埠上;預設關閉 |
|中繼 |hbbs |執行 hbbr 的機器的 IP 位址/DNS 名稱（以逗號分隔）|
|LOG_FORMAT |HBBS， HBBR |`hbbs`“json”[：每條日誌行一個 JSON 物件 （](docs/repositories.md),

### ,

,[,](https://github.com/sctg-development/sctgdesk-server/releases),`hbbs`);預設：文字 |
|RUST_LOG |全部 |設定除錯等級（錯誤|警告|資訊|除錯|追蹤）|
|ROCKET_SECRET_KEY |hbbs |用於簽署 API 伺服器 Cookie 的金鑰（base64,64 位元組）。預設值：於首次啟動時產生`hbbr`資料庫旁 |
|S3CONFIG_FILE |hbbs |軟體更新檔案路徑（`rustdesk-utils`) |
|SINGLE_BANDWIDTH |hbbr |單一連線的最大頻寬（單位：Mb/s）|
|TOTAL_BANDWIDTH |hbbr |最大總頻寬（單位：Mb/s） |`SHA256SUMS-*`只有已登入的使用者才能控制[（或](#first-run)）

詢問 API 伺服器用戶端的權杖是否為有效
在連線兩個節點之前，會話。任何人仍然可以註冊自己的裝置，但沒有人能控制裝置
未登入。未登入的用戶端會看到「此連線不被允許。您尚未登入。」;一
其會話結束時顯示「您的會話已過期」。**使用 GitHub、GitLab、Google、Dex 登入......**複製`winget install SCTG.SCTGDeskServer --scope machine`並填寫您使用的提供者（每個提供者都需要在該提供者註冊的應用程式，該應用程式的提供者
回調是你的 API 伺服器，然後設定`hbbs.exe --windows-service`, created with `sc.exe`): see [On Windows](docs/windows.md).

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
