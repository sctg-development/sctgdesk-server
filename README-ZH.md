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

SCTGDesk 是一个自托管的远程桌面服务器，拥有自己的客户端，源自[锈桌](https://github.com/rustdesk/rustdesk).它是自由软件，属于**AGPL-3.0**.

> **SCTGDesk 是一个独立项目。它与 RustDesk 项目没有任何关联、认可或赞助
> 或称Purslane有限公司。**“RustDesk”是该项目的名称;这里仅用来说明该软件是什么
> 源自并兼容。

这个仓库包含了整个项目：服务器、API 服务器、网页控制台和客户端覆盖层。它取代了
前仓库`sctgdesk-api-server`以及`sctgdesk`这些都已存档。

## 你能得到什么

*   RustDesk服务器：`hbbs`（ID和约会服务器）以及`hbbr`（中继），加`rustdesk-utils`（密钥对，
    连接性检查）。兼容官方 RustDesk 客户端。
*   服务器端**加密的 TCP 和 WebSocket 连接**（密钥交换版本0和1），其中
    开源上游服务器不实现。这正是使**网页客户端**工作。
*   A**REST API**嵌入于`hbbs`，与客户端兼容：登录（密码，OpenID Connect / OAuth2：
    GitHub、GitLab、Google、Dex等）、重启后能存活的会话、个人和共享通讯录（每个用户和
    按组，只读/读写/管理员），用户和组，软件更新端点。
*   A**网页控制台**管理所有这些，以及交互式API文档。
*   拒绝未认证控制的选项（`--logged-in-only`).
*   A**客户端**(`client/`）：上游的RustDesk加上一个小叠加层，在构建时用你的品牌、服务器和
    键;以及**网页客户端**.

兼容性优先：服务器必须持续与官方 RustDesk 客户端协作，并对上游进行更改
文件保持较小，以便合并新的上游版本。

## 安装

选一个。无论你选什么，都需要一个**密钥对**以及下方的端口。

### Linux 软件包（deb，rpm）

每个版本都包括`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`以及`sctgdesk-server-utils`对于**AMD64**以及**arm64**， 作为`.deb`以及`.rpm`，由静态的musl二进制构建（无库依赖）。它们取代了前者`rustdesk-server-*`包裹。

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

第一次安装什么都不启动：编辑单元 （`systemctl edit --full rustdesk-hbbs`，见[部署指南](docs/deployment.md)），则`sudo systemctl enable --now rustdesk-hbbs rustdesk-hbbr`.一个
升级会重新启动正在运行的服务。数据已经进入`/var/lib/rustdesk-server`，日志`/var/log/rustdesk-server`;这些单元以根节点运行。

### 二进制（Linux、Windows、macOS）

该[发行](https://github.com/sctg-development/sctgdesk-server/releases)页面还包含压缩包`hbbs`,`hbbr`以及`rustdesk-utils`适用于Windows（amd64，arm64）和macOS（英特尔，苹果硅），由`SHA256SUMS-*`文件。
解压后，从数据应存放的目录运行它们（参见[首次运行](#first-run)).

### Docker

图片发布于[Docker 枢纽](https://hub.docker.com/r/sctg/sctgdesk-server/)作为`sctg/sctgdesk-server`.
保存为`docker-compose.yml`，替换`sctgdesk.example.com`附上你的服务器地址：

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

关于SELinux，请补充`:z`对体积或使用`--security-opt label=disable`.没有`--net=host`直接点对点
连接无法正常工作（所有设备都通过继电器）。舵手图已输入`deploy/helm`（待重构）以及
一个例子体现在`kubernetes/`.

### 移植版本

|端口 |协议 |使用 |目的 |
|---|---|---|---|
|21114 |TCP |`hbbs`|REST API、网页控制台、API 文档（`--api-port`) |
|21115 |TCP |`hbbs`|NAT 类型测试 |
|21116 |TCP +**UDP**|`hbbs`|身份证 / 集合点 |
|21117 |TCP |`hbbr`|接力 |
|21118 |TCP |`hbbs`|WebSocket（网页客户端） |
|21119 |TCP |`hbbr`|WebSocket（网页客户端） |

在防火墙上打开它们。API 和 WebSocket 端口应该放在互联网上的 TLS 反向代理后面：参见
该[部署指南](docs/deployment.md).

## 首次运行

`hbbs`以及`hbbr`将他们的数据（密钥对、SQLite 数据库）保存在**现行目录**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

首次起跑`hbbs`创造`id_ed25519`（秘密）和`id_ed25519.pub`（**公钥**，你给
你的客户）。自己制作密钥对，或检查一个：

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

要拒绝没有密钥的客户端，请以`-k <key>`（或`KEY=<key>`).检查一下
服务器在所有端口上都回复如下：`rustdesk-utils doctor <server>`.

**默认管理员。**网页控制台位于`http://<server>:21114/`以及API文档`http://<server>:21114/api/doc/`.一个新的数据库中，用户`admin`用密码`Hello,world!`，即
公众：**首次登录时更改**.

## 配置

`hbbs`以及`hbbr`在命令行中选择选项（`hbbs --help`）以及来自环境变量，或`.env`档案。

|变量 |二进制 |描述 |
|--- |--- |--- |
|ALWAYS_USE_RELAY |HBBS |如果设置为**“Y”**禁止直接对等连接 |
|DB_URL |hbbs |SQLite 数据库路径（默认）`./db_v2.sqlite3`) |
|LOGGED_IN_ONLY |HBBS |如果设置为**“Y”**只有登录用户才能控制对等端（同理`--logged-in-only`) |
|REQUIRE_KX_V1 |HBBS |如果设置为**“Y”**拒绝使用旧版（版本0）TCP密钥交换的客户端（与`--require-kx-v1`) |
|DOWNGRADE_START_CHECK |HBRR |延迟（秒数）在降级检查前 |
|DOWNGRADE_THRESHOLD |hbbr |降级检查阈值（bit/ms） |
|密钥 |HBBS/HBBR |如果设置为强制使用特定密钥，则**"\_"**强制使用任意密钥 |
|LIMIT_SPEED |hbbr |限速（单位为Mb/s） |
|OAUTH2\_CONFIG_FILE |hbbs |OAuth2 / OpenID Connect provider 文件的路径（`oauth2.toml`) |
|OAUTH2\_CREATE_USER |HBBS |如果设置为**"1"**首次登录时创建用户 |
|端口 |HBBS/HBBR |监听端口（21116用于HBBS- 21117用于HBBR） |
|中继 |HBBS |运行 hbbr 的机器的 IP 地址/DNS 名称（用逗号分隔） |
|RUST_LOG |全部 |设置调试级别（错误|警告|信息|调试|trace）|
|S3CONFIG_FILE |HBBS |软件更新文件的路径（`s3config.toml`) |
|SINGLE_BANDWIDTH |hbbr |单次连接的最大带宽（单位为Mb/s） |
|TOTAL_BANDWIDTH |hbbr |最大总带宽（单位为Mb/s） |

### 只有登录用户才能控制

`--logged-in-only`（或`LOGGED_IN_ONLY=Y`）`hbbs`询问 API 服务器客户端的令牌是否为活体
在连接两个节点之前，会话。任何人仍然可以注册自己的设备，但没人能控制设备
未登录。未登录的客户端看到“连接不被允许。你尚未登录。”;一
其会话结束时显示“您的会话已过期”。

### 用GitHub、GitLab、Google、Dex等登录。

收到`oauth2.toml`并填写你使用的服务提供者（每个服务商都需要注册一个应用程序，该应用的提供者
回调是你的 API 服务器），然后设置`OAUTH2_CONFIG_FILE=oauth2.toml`， 和`OAUTH2_CREATE_USER=1`以创建
用户首次登录。参见[部署指南](docs/deployment.md#login-with-openid-connect-and-oauth2).

### 客户端更新

客户端可以在你的API服务器上寻找新版本，而不是GitHub：它回答`POST /api/software/version/latest`（当前RustDesk客户端的检查）以及`GET /api/software/releases/latest`.
最新版本和下载链接可从以下阅读`s3config.toml`（兼容S3的存储），参见[REST API](docs/api.md#software-update-endpoints).

## 客户

*   **官方 RustDesk 客户端**与这台服务器合作：把你的服务器和公钥交给他们。
*   **SCTGDesk 客户端**是上游的RustDesk加上我们的覆盖层（名称、图标、默认服务器和密钥）。你自己构建它
    关于你自己的服务器，请参见[定制车](docs/custom-builds.md#the-client).该**网页客户端**由
    同样的叠加层。

## 自己组装吧

你需要一个Rust工具链，网页控制台需要Node.js 20。

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

赛场`SCTGDESK_SKIP_WEB_BUILD=1`无需Node.js构建（控制台则是占位符）。该[开发者手册](https://sctg-development.github.io/sctgdesk-server/)涵盖构建、测试、定制服务器和
客户端构建、打包、发布以及与上游同步。

## 截图

### 网页控制台

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### API 文档

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### 在客户端

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## 安全性

*   嵌入式API服务器没有防止滥用：在前面放置一个TLS反向代理（HAProxy、NGINX、Caddy等）
    以及WebSocket端口。HAProxy的一个示例见于[部署指南](docs/deployment.md#tls-reverse-proxy).
*   更改默认管理员密码，保持不变`id_ed25519`秘密并备份：更改它会让每个客户都拒绝
    你的服务器。
*   报告以下描述的漏洞[SECURITY.md](SECURITY.md).

## 许可与源代码

SCTGDesk 分发在[GNU Affero 通用公共许可证 v3](LICENSE).每个版本的源代码
就在这个仓库里，在该版本的标签下。如果你修改了这个软件，并让其他人使用它，而不是
Network，许可证第13条要求你向他们提供修改版本的相应源代码。

在运行下载的二进制文件前，请检查每次发布时公布的SHA-256和数据。

## 贡献

[开发者手册](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md).关于一般的自主机问题，请使用[RustDesk 文档](https://rustdesk.com/docs/en/self-host/)同样适用。

如果你喜欢这部作品，请考虑给它一颗星，或者[赞助](https://github.com/sponsors/sctg-development).
