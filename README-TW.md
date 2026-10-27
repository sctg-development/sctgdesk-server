<p align="center">
  <a href="#how-to-build-manually">Manually</a> •
  <a href="#docker-images">Docker</a> •
  <a href="#how-to-create-a-keypair">Keypair</a> •
  <a href="#packages">Binaries</a> •
  <a href="#env-variables">Variables</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>] | [<a href="README-RU.md">Русский</a>]<br>
</p>

![](https://tokeisrv.sctg.eu.org/b1/github.com/sctg-development/sctgdesk-server?rust\&category=code)
![](https://tokeisrv.sctg.eu.org/b1/github.com/sctg-development/sctgdesk-server?rust\&category=comments)

# SctgDesk 伺服器程式

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**二進位下載**](https://github.com/sctg-development/sctgdesk-server/releases)

[**API 文件**](https://sctg-development.github.io/sctgdesk-api-server/)

這是 RustDesk Server 的修改版本，RustDesk 是免費且開源的。

*   第一個差異是這個版本包含了新的*TCP*RustDesk Server Pro 版本中包含的模式。
*   第二個差異是此版本包含 Rustdesk Server Pro API 伺服器的初步實作。
    *   個人通訊錄支援
    *   群組層級對共用通訊錄的支持
        *   唯讀、讀寫、管理
    *   使用者層級對共用通訊錄的支援
        *   唯讀、讀寫、管理
*   第三個差異是此版本包含一個簡單網頁控制台的初步實作。

網頁控制台可於地址存取`http://<server-ip>:21114/`登入方式為「admin」，密碼為「Hello， world！」。\
你可以在 Builtins API 伺服器的 API 文件中瀏覽該地址`http://<server-ip>:21114/api/doc/`.

非互動式 API 文件可於[SctgDesk-API-Server Repo](https://sctg-development.github.io/sctgdesk-api-server/).

## 為專案加星

**如果你欣賞我的作品，請考慮給它一顆星！🤩或者一顆星**[![](https://img.shields.io/static/v1?label=Sponsor\&message=%E2%9D%A4\&logo=GitHub\&color=%23fe8e86)](https://github.com/sponsors/sctg-development)

## 簡短摘要;摘要

你可以使用以下工具`docker-compose.yml`用來啟動伺服器的檔案：

```yaml
version: '3'

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

並以以下方式開始伺服器：

```bash
mkdir -p data
docker-compose up 
```

## 二進位

以下平台提供雙進制：

*   Linux x86\_64靜態連結
*   Linux arm64 靜態連結
*   Linux armv7 靜態連結
*   MacOS Intel
*   MacOS Apple Silicon
*   Windows x86\_64

### 預設管理員使用者

預設管理員是用該使用者名稱建立的`admin`以及密碼`Hello,world!`.你可以在第一次登入後在網頁控制台更改密碼。

## API 獨立版本

API 獨立版本是指包含 API 伺服器和網頁主控台，但不包含 rendez-vous 伺服器的版本。\
獨立版本則有其獨立的儲存庫[Sctgdesk-API-server](https://github.com/sctg-development/sctgdesk-api-server).\
關於所有與 API 或網頁控制台相關的問題，請參考[Sctgdesk-API-server](https://github.com/sctg-development/sctgdesk-api-server)資料庫。

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

### 在 Rustdesk 用戶端的使用

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## 產生自動更新連結

我們修改了客戶端，讓自動更新連結從 API 伺服器取得，而非從 Github 版本中取得。\
為了讓自動更新連結正常運作，你需要修改你的客戶端，讓它能從 API 伺服器取得自動更新連結。這個[你該怎麼做](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

```rust
// src/common.rs
#[tokio::main(flavor = "current_thread")]
async fn check_software_update_() -> hbb_common::ResultType<()> {
    let url=format!("{}/api/software/releases/latest",get_api_server("".to_owned(), "".to_owned())).to_owned();
    log::info!("URL for checking software updates: {}", url);
    //let url = "https://github.com/rustdesk/rustdesk/releases/latest";
    let latest_release_response = create_http_client_async().get(url).send().await?;
    let latest_release_version = latest_release_response
        .url()
        .path()
        .rsplit('/')
        .next()
        .unwrap_or_default();

    let response_url = latest_release_response.url().to_string();

    if get_version_number(&latest_release_version) > get_version_number(crate::VERSION) {
        *SOFTWARE_UPDATE_URL.lock().unwrap() = response_url;
    }
    Ok(())
}
```

# 安全性

嵌入式 API 伺服器並未受到 DDOS 攻擊的保護或保護。一個好做法是在 API 伺服器前方使用反向代理。NGINX 是這個目的的好選擇。HAProxy 也是不錯的選擇。\
我們在生產環境中使用 HAProxy 放在 API 伺服器前方。
這是我們 HAProxy 的設定檔，僅作為範例提供。你應該根據自己的需求調整它。

```haproxy
global
    log /dev/log    local0
    log /dev/log    local1 notice
    chroot /var/lib/haproxy
    stats socket /run/haproxy/admin.sock mode 660 level admin expose-fd listeners
    stats timeout 30s
    user haproxy
    group haproxy
    daemon

defaults
    log global
    retries 2
    timeout connect 3000ms
    timeout server 5000ms
    timeout client 5000ms

frontend hbbs_wss
    bind 0.0.0.0:21120 ssl crt /etc/haproxy/hbb.pem
    default_backend hbbs_wss_backend

frontend hbbs_api
    mode http
    option forwardfor
    bind 0.0.0.0:21114 ssl crt /etc/haproxy/api.pem
    http-request set-header X-Forwarded-Proto https
    default_backend hbbs_api_backend

frontend hbbs_api_443
    mode http
    option forwardfor
    bind 0.0.0.0:443 ssl crt /etc/haproxy/api.pem
    http-request set-header X-Forwarded-Proto https
    filter compression
    compression algo gzip
    compression type text/css text/html text/javascript application/javascript text/plain text/xml application/json
    compression offload
    default_backend hbbs_api_backend

frontend hbbr_wss
    bind 0.0.0.0:21121 ssl crt /etc/haproxy/hbb.pem
    default_backend hbbr_wss_backend

backend hbbs_api_backend
    mode http
    server srv_main 127.0.0.1:21113

backend hbbs_wss_backend
    server srv_main 127.0.0.1:21118

backend hbbr_wss_backend
    server srv_main 127.0.0.1:21119
```

HBBS 伺服器啟動時會以

```service
[Unit]
Description=Rustdesk Signal Server

[Service]
Type=simple
LimitNOFILE=1000000
ExecStart=/usr/bin/hbbs --api-port=21113 -k AucFCOYVWNHRkJnx13FFh7C0tmUZ3nei5wXKmlfK6WPYthz65fRavaA5HO/OIz2kq+bCSlAqBkZgvikwVGqw/Q== --mask=10.10.0.235/24 -r rendez-vous.example.org -R rendez-vous.example.org
#Environment="RUST_LOG=debug"
Environment="ALWAYS_USE_RELAY=Y"
Environment="OAUTH2_CREATE_USER=1"
Environment="S3CONFIG_FILE=s3config.toml"
Environment="OAUTH2_CONFIG_FILE=oauth2.toml"
WorkingDirectory=/var/lib/rustdesk-server/
User=
Group=
Restart=always
StandardOutput=append:/var/log/rustdesk-server/hbbs.log
StandardError=append:/var/log/rustdesk-server/hbbs.error
# Restart service after 10 seconds if node service crashes
RestartSec=10

[Install]
WantedBy=multi-user.target
```

# 限制不受歡迎的存取

要限制對伺服器的存取，你可以使用`--logged-in-only`選項或設定`LOGGED_IN_ONLY=Y`環境變數`hbbs`伺服器。這將限制只限登入用戶控制。

即使啟用此選項，使用者仍可在 Rendez-vous 伺服器註冊，但無法控制其他使用者的同儕。若有人未登入嘗試控制同儕，會收到錯誤訊息：

<img width="524" alt="Error message for unauthenticated control attempt" src="https://github.com/user-attachments/assets/cfa46504-39d8-46a7-9072-3ece6818b4a3">

啟用此功能後，您可以為伺服器增加額外安全層，防止未經授權的存取。

**配置`LOGGED_IN_ONLY`**

啟用此功能：

1.  設定`LOGGED_IN_ONLY=Y`環境變數`hbbs`伺服器。
2.  或者，你也可以使用`--logged-in-only`執行`hbbs`伺服器。

**範例**

設定`LOGGED_IN_ONLY`環境變數，你可以在你的`~/.bashrc`檔案或同等文件：

```bash
export LOGGED_IN_ONLY=Y
```

# RustDesk 伺服器程式

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**下載**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**手排**](https://rustdesk.com/docs/en/self-host/)

[**常見問題**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

[**如何將 OSS 遷移到 Pro 版**](https://rustdesk.com/docs/en/self-host/rustdesk-server-pro/installscript/#convert-from-open-source)

你可以自行架設 RustDesk 伺服器，它是免費且開源的。

## 如何手動建構

首先你需要有一個可運作的 Rust 開發工具鏈和一個可運作的 Node ≥20 安裝。

*   Unics（Linux、MacOS 等）：

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   帶有cmd.exe殼的視窗：

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

目標/發佈中會產生三個執行檔。

*   hbbs - RustDesk ID/Rendezvous server with API server
*   hbbr - RustDesk 中繼伺服器
*   rustdesk-utils - RustDesk CLI 工具

你可以在[發行](https://github.com/sctg-development/sctgdesk-server/releases)佩奇。

所有在 v1.1.99-40 之後釋出的二進位檔都會用 Github Actions 來認證。你可以用 java 檢查 sha256sum 來檢查認證`https://search.sigstore.dev/?hash=<sha256>`舉例來說。

如果你想要額外功能，[RustDesk 伺服器專業版](https://rustdesk.com/pricing.html)可能更適合你。

如果你想自己開發伺服器，[Rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo)這可能比這個倉庫更簡單、更適合你。

## 安裝

Docker 映像檔會自動生成並在每個 GitHub 版本中發布。

這些影像是建立在這些基礎上的`ubuntu-22.04`唯一加入主要二進位的方法是 （`hbbr`以及`hbbs`).它們可以在[Docker 樞紐](https://hub.docker.com/r/sctg/sctgdesk-server/)使用以下標籤：

|建築 |圖片：標籤 |
|--- |--- |
|AMD64 |`sctg/sctgdesk-server:latest`|
|arm64v8 |`sctg/sctgdesk-server:latest`|
|arm32v7 |`sctg/sctgdesk-server:latest`|

你可以直接從這些圖片開始`docker run`以下指令：

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

或不帶`--net=host`但 P2P 直接連線無法運作。

對於使用 SELinux 的系統，替換`/root`作者`/root:z`是容器正常運行所必須的。或者，也可以完全關閉 SELinux 容器分離，加入這個選項`--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

該`relay-server-ip`參數是運行這些容器的伺服器的 IP 位址（或 DNS 名稱）。該**可選**`port`如果你使用不同的埠，必須使用 參數。**21117**對於`hbbr`.

你也可以使用 docker-compose，並以此設定作為範本：

```yaml
version: '3'

networks:
  sctgdesk-net:
    external: false

services:
  hbbs:
    container_name: hbbs
    ports:
      - 21114:21114
      - 21115:21115
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
    image: sctg/sctgdesk-server-server:latest
    command: hbbr
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    restart: unless-stopped
```

編輯第 16 行，指向你的中繼伺服器（也就是在 21117 埠監聽的那台）。如果需要，也可以編輯音量線（第 18 行和第 33 行）。

（docker-compose 的功勞歸於 @lukebarone 和 @QuiGonLeong）

> \[!註]\
> 在中國，rustdesk/rustdesk-server：latest 可能會被 Docker Hub 上的最新版本號取代，例如`rustdesk-server:1.1.10-3`.否則，舊版本可能會因影像加速而被拉取。

> 請注意，中國的 sctg/sctgdesk-server-server：latest 可能會被 dockerhub 上的最新版本號取代，例如 sctg/sctgdesk-server-server：1.1.99-37。否則，舊版本可能會因影像加速而被移除。

## 如何建立金鑰對

加密需要一對金鑰對;你可以提供，如前所述，但你需要一種方法來建立。

你可以使用此指令產生一組金鑰對：

```bash
/usr/bin/rustdesk-utils genkeypair
```

如果你沒有（或不想要）`rustdesk-utils`安裝在你系統上的套件，你也可以用 Docker 呼叫相同的指令：

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

輸出會是這樣的：

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## 套裝

每個二進位檔都有獨立的.deb套件，你可以在[發行](https://github.com/sctg-development/sctgdesk-server/releases).
這些套件適用於以下發行版：

*   Ubuntu 22.04 LTS
*   MacOS、Intel 或 Apple Silicon
*   Windows x86\_64 或 i686

## ENV 變數

`hbbs`以及`hbbr`可透過這些 ENV 變數進行配置。
你可以像平常一樣指定變數，或使用`.env`檔案。

|變數 |二元值 |描述 |
|--- |--- |--- |
|ALWAYS_USE_RELAY |HBBS |若設為**「Y」**禁止直接對等連接 |
|DOWNGRADE_START_CHECK |HBBR |延遲（以秒計）在降級檢查前 |
|DOWNGRADE_THRESHOLD |HBBBR |降級檢查門檻（bit/ms） |
|KEY |HBBS/HBBR |若設定為 ，強制使用特定鍵，則若設為**"\_"**強制使用任意鍵 |
|LIMIT_SPEED |hbbr |速限（以 Mb/s） |
|OAUTH2\_CONFIG_FILE |hbbs |OUs2 設定檔路徑 |
|OAUTH2\_CREATE_USER |HBBS |若設定為**"1"**如果沒有使用者，請建立 |
|埠口 |HBBS/HBBR |聆聽埠（21116 對 HBBS - 21117 對 HBBR）|
|中繼 |HBBS |運行 hbbr 的機器 IP 位址/DNS 名稱（以逗號分隔） |
|RUST_LOG |全部 |設定除錯層級（錯誤|警告|資訊|除錯|追蹤）|
|S3CONFIG_FILE |HBBS |S3 設定檔路徑 |
|SINGLE_BANDWIDTH |hbbr |單次連線的最大頻寬（以 Mb/s 計）|
|TOTAL_BANDWIDTH |hbbr |最大總頻寬（以 Mb/s） |
