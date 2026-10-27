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

# SctgDesk 服务器程序

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**二进制下载**](https://github.com/sctg-development/sctgdesk-server/releases)

[**API 文档**](https://sctg-development.github.io/sctgdesk-api-server/)

这是RustDesk Server的修改版，RustDesk是免费且开源的。

*   第一个区别是这个版本包含了新的*TCP*RustDesk Server Pro 版本中包含的模式。
*   第二个区别是该版本包含了 Rustdesk Server Pro API 服务器的初步实现。
    *   个人通讯录的支持
    *   群组层面对共享通讯录的支持
        *   只读、读写、管理员
    *   用户级对共享通讯录的支持
        *   只读、读写、管理员
*   第三个区别是该版本包含了一个简单的网页控制台的初步实现。

网页控制台可在地址访问`http://<server-ip>:21114/`登录名为“admin”，密码为“Hello， world！”。\
你可以在 Builtins 的 API 服务器上浏览 API 文档，地址是`http://<server-ip>:21114/api/doc/`.

非交互式API文档可在以下网站获取[SctgDesk-API-Server Repo](https://sctg-development.github.io/sctgdesk-api-server/).

## 为项目加星

**如果你喜欢我的作品，请考虑给它一个星！🤩或者**[![](https://img.shields.io/static/v1?label=Sponsor\&message=%E2%9D%A4\&logo=GitHub\&color=%23fe8e86)](https://github.com/sponsors/sctg-development)

## 简而言之;总结

你可以使用以下内容`docker-compose.yml`用来启动服务器的文件：

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

并以以下方式开始服务器：

```bash
mkdir -p data
docker-compose up 
```

## 二元论

以下平台提供双进制：

*   Linux x86\_64静态链接
*   Linux arm64 静态链接
*   Linux armv7 静态链接
*   MacOS Intel
*   MacOS 苹果硅片
*   Windows x86\_64

### 默认管理员用户

默认管理员用户是用该用户名创建的`admin`以及密码`Hello,world!`.你可以在网页控制台首次登录后更改密码。

## API 独立版本

API 独立版本是包含 API 服务器和网页控制台，但不包含 rendez-vous 服务器的版本。\
独立版本可在其独立仓库中获取[SctgDesk-API-Server](https://github.com/sctg-development/sctgdesk-api-server).\
关于所有与API或网页控制台相关的问题，请参见[SctgDesk-API-Server](https://github.com/sctg-development/sctgdesk-api-server)仓库。

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

### 在 Rustdesk 客户端中的应用

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## 生成自动更新链接

我们修改了客户端，使其从API服务器中获取自动更新链接，而非从Github发布中获取。\
为了让自动更新链接正常工作，你需要修改客户端，从API服务器获取自动更新链接。就是这样[你该如何做到](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

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

嵌入式API服务器既不安全，也不具备DDOS攻击的保护。一个好做法是在API服务器前使用反向代理。NGINX是这个目的的好选择。HAProxy也是不错的选择。\
我们在生产环境中使用 HAProxy 来设置 API 服务器。
这是我们为 HAProxy 提供的配置文件，仅作为示例提供。你应该根据自己的需求进行调整。

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

HBBS服务器启动时

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

# 限制不受欢迎的访问

要限制对服务器的访问，你可以使用`--logged-in-only`选项或设置`LOGGED_IN_ONLY=Y`环境变量`hbbs`服务器。这将限制控制权仅限登录用户。

即使开启了这个选项，用户仍然可以在Rendez-vous服务器注册，但他们无法控制其他用户的对等节点。如果有人在未登录的情况下尝试控制对等节点，他们会收到错误提示：

<img width="524" alt="Error message for unauthenticated control attempt" src="https://github.com/user-attachments/assets/cfa46504-39d8-46a7-9072-3ece6818b4a3">

启用此功能后，您可以为服务器增加额外安全层，防止未经授权的访问。

**配置`LOGGED_IN_ONLY`**

启用此功能：

1.  设置`LOGGED_IN_ONLY=Y`环境变量`hbbs`服务器。
2.  或者，你也可以使用`--logged-in-only`运行`hbbs`服务器。

**示例**

要设置`LOGGED_IN_ONLY`环境变量，你可以在你的`~/.bashrc`文件或同等文件：

```bash
export LOGGED_IN_ONLY=Y
```

# RustDesk 服务器程序

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**下载**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**手动**](https://rustdesk.com/docs/en/self-host/)

[**常见问题**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

[**如何将OSS迁移到Pro版**](https://rustdesk.com/docs/en/self-host/rustdesk-server-pro/installscript/#convert-from-open-source)

你可以自己托管RustDesk服务器，它是免费且开源的。

## 如何手动构建

首先你需要有一个可用的 Rust 开发工具链和一个 Node ≥20 的安装。

*   Unics（Linux、MacOS等）：

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   带 cmd.exe shell 的 Windows：

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

目标/发布中将生成三个可执行文件。

*   hbbs - RustDesk ID/Rendezvous server with API server
*   hbbr - RustDesk 中继服务器
*   rustdesk-utils - RustDesk CLI 工具

You can find updated binaries on the [releases](https://github.com/sctg-development/sctgdesk-server/releases) page.

All released binaries after release v1.1.99-40 are attestated with Github Actions. You can check the attestation by checking the sha256sum of the binary with `https://search.sigstore.dev/?hash=<sha256>` for example.

If you want extra features, [RustDesk Server Pro](https://rustdesk.com/pricing.html) might suit you better.

If you want to develop your own server, [rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo) might be a better and simpler start for you than this repo.

## Installation

Docker images are automatically generated and published on every github release.

These images are build against `ubuntu-22.04` with the only addition of the main binaries (`hbbr` and `hbbs`). They're available on [Docker hub](https://hub.docker.com/r/sctg/sctgdesk-server/) with these tags:

| architecture | image:tag |
| --- | --- |
| amd64 | `sctg/sctgdesk-server:latest` |
| arm64v8 | `sctg/sctgdesk-server:latest` |
| arm32v7 | `sctg/sctgdesk-server:latest` |

You can start these images directly with `docker run` with these commands:

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

or without `--net=host`, but P2P direct connection can not work.

For systems using SELinux, replacing `/root` by `/root:z` is required for the containers to run correctly. Alternatively, SELinux container separation can be disabled completely adding the option `--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

The `relay-server-ip` parameter is the IP address (or dns name) of the server running these containers. The **optional** `port` parameter has to be used if you use a port different than **21117** for `hbbr`.

You can also use docker-compose, using this configuration as a template:

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

Edit line 16 to point to your relay server (the one listening on port 21117). You can also edit the volume lines (line 18 and line 33) if you need.

(docker-compose credit goes to @lukebarone and @QuiGonLeong)

> \[!NOTE]\
> The rustdesk/rustdesk-server:latest in China may be replaced with the latest version number on Docker Hub, such as `rustdesk-server:1.1.10-3`. Otherwise, the old version may be pulled due to image acceleration.

> Note that here, the sctg/sctgdesk-server-server:latest in China may be replaced with the latest version number on dockerhub, such as sctg/sctgdesk-server-server:1.1.99-37. Otherwise, the old version may be pulled due to image acceleration.

## How to create a keypair

A keypair is needed for encryption; you can provide it, as explained before, but you need a way to create one.

You can use this command to generate a keypair:

```bash
/usr/bin/rustdesk-utils genkeypair
```

If you don't have (or don't want) the `rustdesk-utils` package installed on your system, you can invoke the same command with docker:

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

The output will be something like this:

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## Packages

Separate .deb packages are available for each binary, you can find them in the [releases](https://github.com/sctg-development/sctgdesk-server/releases).
These packages are meant for the following distributions:

*   Ubuntu 22.04 LTS
*   MacOS Intel or Apple Silicon
*   Windows x86\_64 or i686

## ENV variables

`hbbs` and `hbbr` can be configured using these ENV variables.
You can specify the variables as usual or use an `.env` file.

| variable | binary | description |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | if set to **"Y"** disallows direct peer connection |
| DOWNGRADE_START_CHECK | hbbr | delay (in seconds) before downgrade check |
| DOWNGRADE_THRESHOLD | hbbr | threshold of downgrade check (bit/ms) |
| KEY | hbbs/hbbr | if set force the use of a specific key, if set to **"\_"** force the use of any key |
| LIMIT_SPEED | hbbr | speed limit (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | path for oauth2 config file |
| OAUTH2\_CREATE_USER | hbbs | if set to **"1"** create a user if it doesn't exist |
| PORT | hbbs/hbbr | listening port (21116 for hbbs - 21117 for hbbr) |
| RELAY | hbbs | IP address/DNS name of the machines running hbbr (separated by comma) |
| RUST_LOG | all | set debug level (error|warn|info|debug|trace) |
| S3CONFIG_FILE | hbbs | path for s3 config file |
| SINGLE_BANDWIDTH | hbbr | max bandwidth for a single connection (in Mb/s) |
| TOTAL_BANDWIDTH | hbbr | max total bandwidth (in Mb/s) |
