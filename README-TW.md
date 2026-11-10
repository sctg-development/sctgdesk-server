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

第一次安裝不會啟動任何東西：編輯單位 （`systemctl edit --full rustdesk-hbbs`，請見[部署指南](docs/deployment.md)），則`sudo systemctl enable --now rustdesk-hbbs rustdesk-hbbr`.一個
升級會重新啟動正在執行的服務。資料已經進入`/var/lib/rustdesk-server`，日誌`/var/log/rustdesk-server`;這些單元以 root 方式執行。

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
|21119 |TCP |`hbbr`|WebSocket（網頁客戶端） |

在防火牆上開啟它們。API 和 WebSocket 埠應該放在網際網路上的 TLS 反向代理後面：詳見
該[部署指南](docs/deployment.md).

## 首次運行

`hbbs`以及`hbbr`將他們的資料（金鑰對、SQLite 資料庫）保存在**現行名錄**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

首次起跑`hbbs`創造`id_ed25519`（秘密） 以及`id_ed25519.pub`（**公鑰**，你給
你的客戶）。自己製作金鑰對，或檢查其中一組：

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

要拒絕沒有金鑰的客戶端，請啟動兩台伺服器`-k <key>`（或`KEY=<key>`).檢查一下
伺服器在所有埠上回應為`rustdesk-utils doctor <server>`.

**預設管理員。**網頁主控台位於`http://<server>:21114/`以及 API 文件`http://<server>:21114/api/doc/`.一個新的資料庫會擁有使用者`admin`用密碼`Hello,world!`，即
公開：**第一次登入時更改**.

## 配置

`hbbs`以及`hbbr`在命令列上取得選項（`hbbs --help`）以及來自環境變數，或`.env`檔案。

|變數 |二元值 |描述 |
|--- |--- |--- |
|ALWAYS_USE_RELAY |HBBS |若設為**「Y」**禁止直接對等連接 |
|DB_URL |hbbs |SQLite 資料庫路徑（預設值）`./db_v2.sqlite3`) |
|LOGGED_IN_ONLY |HBBS |若設為**「Y」**只有登入的使用者才能控制對等端（同理`--logged-in-only`) |
|REQUIRE_KX_V1 |HBBS |若設定為**「Y」**拒絕使用舊有（版本 0）TCP 金鑰交換的用戶端（與`--require-kx-v1`) |
|DOWNGRADE_START_CHECK |HBBR |延遲（以秒計）在降級檢查前 |
|DOWNGRADE_THRESHOLD |HBBBR |降級檢查門檻（bit/ms） |
|KEY |HBBS/HBBR |若設定為 ，強制使用特定鍵，則若設為**"\_"**強制使用任意鍵 |
|LIMIT_SPEED |hbbr |速限（以 Mb/s） |
|OAUTH2\_CONFIG_FILE |hbbs |OAuth2 / OpenID Connect 提供者檔案的路徑（`oauth2.toml`) |
|OAUTH2\_CREATE_USER |HBBS |若設定為**"1"**首次登入時建立使用者 |
|埠口 |HBBS/HBBR |聆聽埠（21116 對 HBBS - 21117 對 HBBR）|
|中繼 |HBBS |運行 hbbr 的機器 IP 位址/DNS 名稱（以逗號分隔） |
|RUST_LOG |全部 |設定除錯層級（錯誤|警告|資訊|除錯|追蹤）|
|S3CONFIG_FILE |HBBS |軟體更新檔案的路徑（`s3config.toml`) |
|SINGLE_BANDWIDTH |hbbr |單次連線的最大頻寬（以 Mb/s 計）|
|TOTAL_BANDWIDTH |hbbr |最大總頻寬（以 Mb/s） |

### 只有登入用戶才能控制

`--logged-in-only`（或`LOGGED_IN_ONLY=Y`）`hbbs`詢問 API 伺服器客戶端的權杖是否為活體
在連接兩個節點之前，會話。任何人仍然可以註冊自己的裝置，但沒有人能控制裝置
未登入。未登入的客戶端會看到「連線不被允許。您尚未登入。」;一
其會話結束時顯示「你的會話已過期」。

### 使用 GitHub、GitLab、Google、Dex 登入......

收到`oauth2.toml`並填寫你使用的服務提供者（每個提供者都需註冊一個應用程式，該服務提供者的應用程式
回調是你的 API 伺服器），然後設定`OAUTH2_CONFIG_FILE=oauth2.toml`， 以及`OAUTH2_CREATE_USER=1`以創造
使用者首次登入。請參閱[部署指南](docs/deployment.md#login-with-openid-connect-and-oauth2).

### 客戶端更新

客戶端可以在你的 API 伺服器上尋找新版本，而不是在 GitHub 上：它回答了`POST /api/software/version/latest`（目前 RustDesk 用戶端的檢查）以及`GET /api/software/releases/latest`.
最新版本及下載連結皆可閱讀`s3config.toml`（S3相容儲存），參見[REST API](docs/api.md#software-update-endpoints).

## 客戶

*   **官方 RustDesk 客戶端**與這台伺服器合作：提供你的伺服器和公鑰給他們。
*   **SCTGDesk 用戶端**是上游的 RustDesk 加上我們的覆蓋層（名稱、圖示、預設伺服器和金鑰）。你自己建置它
    關於你自己的伺服器，請參見[客製化組裝](docs/custom-builds.md#the-client).該**網頁用戶端**由
    同樣的疊加層。

## 自己動手做

你需要一個 Rust 工具鏈，網頁控制台則需要 Node.js 20。

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

場景`SCTGDESK_SKIP_WEB_BUILD=1`沒有Node.js來建造（控制台則是佔位符）。這個[開發者手冊](https://sctg-development.github.io/sctgdesk-server/)涵蓋建置、測試、自訂伺服器及
客戶端建置、打包、發布以及與上游同步。

## 截圖

### 網頁控制台

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### API 文件

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### 在客戶端

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## 安全性

*   嵌入式 API 伺服器沒有防止濫用：在前面放置一個 TLS 反向代理（HAProxy、NGINX、Caddy 等）
    以及 WebSocket 埠口。HAProxy 的一個範例是[部署指南](docs/deployment.md#tls-reverse-proxy).
*   更改預設的管理員密碼，保持不變`id_ed25519`秘密並備份：改變它會讓每個客戶都拒絕
    你的伺服器。
*   如描述的漏洞報告[SECURITY.md](SECURITY.md).

## 授權與原始碼

SCTGDesk 以[GNU Affero 通用公共授權條款 v3](LICENSE).每個版本的原始碼
就在這個倉庫裡，該版本的標籤中。如果你修改了這個軟體，並讓其他人使用它，而不是
Network，授權條款第 13 條要求你提供修改版本的對應原始碼。

在執行下載的二進位檔前，請檢查每次發行時公布的 SHA-256 總和。

## 貢獻

[開發者手冊](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md).關於一般自家主機的問題，請參考[RustDesk 文件](https://rustdesk.com/docs/en/self-host/)同樣適用。

如果你喜歡這部作品，請考慮給它一顆星，或[贊助](https://github.com/sponsors/sctg-development).
