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

# SctgDesk Server Programm

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**Binär-Download**](https://github.com/sctg-development/sctgdesk-server/releases)

[**API-Dokumentation**](https://sctg-development.github.io/sctgdesk-api-server/)

Dies ist eine modifizierte Version von RustDesk Server, die kostenlos und Open Source ist.

*   Der erste Unterschied ist, dass diese Version die neue Version enthält*TCP*Modus, der in der RustDesk Server Pro-Version enthalten ist.
*   Der zweite Unterschied besteht darin, dass diese Version eine vorläufige Implementierung des Rustdesk Server Pro API-Servers enthält.
    *   Unterstützung für persönliches Adressbuch
    *   Unterstützung für gemeinsames Adressbuch auf Gruppenebene
        *   Nur lesen, lesen-schreiben, admin
    *   Unterstützung für gemeinsames Adressbuch auf Benutzerebene
        *   Nur lesen, lesen-schreiben, admin
*   Der dritte Unterschied besteht darin, dass diese Version eine vorläufige Implementierung einer einfachen Webkonsole enthält.

Die Webkonsole ist unter der Adresse zugänglich`http://<server-ip>:21114/`mit Login "admin" und Passwort "Hallo, Welt!" .\
Sie können die API-Dokumentation im integrierten API-Server unter der Adresse durchsuchen`http://<server-ip>:21114/api/doc/`.

Eine nicht-interaktive API-Dokumentation ist verfügbar unter[sctgdesk-api-server repo](https://sctg-development.github.io/sctgdesk-api-server/).

## Starten Sie das Projekt

**Wenn du meine Arbeit schätzt, überlege ihr bitte, ihr einen Stern zu geben! 🤩 oder einen** [![](https://img.shields.io/static/v1?label=Sponsor\&message=%E2%9D%A4\&logo=GitHub\&color=%23fe8e86)](https://github.com/sponsors/sctg-development)

## TL; DR

Sie können Folgendes verwenden`docker-compose.yml`Datei zum Starten des Servers:

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

und den Server mit:

```bash
mkdir -p data
docker-compose up 
```

## Doppelsterne

Binärdateien sind für folgende Plattformen verfügbar:

*   Linux x86\_64 statisch verknüpft
*   Linux arm64 statisch verknüpft
*   Linux armv7 statisch verknüpft
*   MacOS Intel
*   MacOS Apple Silicon
*   Windows x86\_64

### Standard-Admin-Benutzer

Der Standard-Admin-Benutzer wird mit dem Benutzernamen erstellt`admin`und das Passwort`Hello,world!`. Du kannst das Passwort nach dem ersten Login in der Webkonsole ändern.

## API Standalone-Version

Die API-Standalone-Version ist eine Version des Servers, die den API-Server und die Webkonsole enthält, aber nicht den Rendez-vous-Server.\
Die eigenständige Version ist in einem eigenen Repository verfügbar[sctgdesk-api-server](https://github.com/sctg-development/sctgdesk-api-server).\
Für alle API- oder Webconsole-bezogenen Probleme siehe bitte die[sctgdesk-api-server](https://github.com/sctg-development/sctgdesk-api-server)Repository.

## Screenshots

### Webkonsole

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### API-Dokumentation

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### Verwendung im Rustdesk-Client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Generierung von Autoupdate-Links

Wir haben unseren Client so modifiziert, dass er die Autoupdate-Links vom API-Server abruft, statt von Github-Releases.\
Damit die Autoupdate-Links funktionieren, musst du deinen Client so modifizieren, dass er die Autoupdate-Links vom API-Server abruft. Das[Wie du es machen kannst](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

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

# Sicherheit

Der eingebettete API-Server ist weder gesichert noch gegen DDOS-Angriffe geschützt. Eine gute Praxis ist es, einen Reverse-Proxy vor dem API-Server zu verwenden. NGINX ist für diesen Zweck eine gute Wahl. HAProxy ist ebenfalls eine gute Wahl.\
Wir verwenden HAProxy vor dem API-Server in unserer Produktionsumgebung. 
Dies ist unsere Konfigurationsdatei für HAProxy, sie wird nur als Beispiel bereitgestellt. Sie sollten sie an Ihre eigenen Bedürfnisse anpassen:

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

Der hbbs-Server wird gestartet mit

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

# Begrenze unerwünschten Zugriff

Um den Zugriff auf deinen Server zu beschränken, kannst du die`--logged-in-only`Option oder setze die`LOGGED_IN_ONLY=Y`Umgebungsvariable für die`hbbs`Server. Dadurch beschränkt sich die Kontrolle nur auf eingeloggte Nutzer.

Selbst mit dieser Option können sich Nutzer weiterhin auf dem Rendez-vous-Server registrieren, können aber nicht den Peer eines anderen Benutzers steuern. Wenn jemand versucht, einen Peer zu steuern, ohne eingeloggt zu sein, erhält er eine Fehlermeldung:

<img width="524" alt="Error message for unauthenticated control attempt" src="https://github.com/user-attachments/assets/cfa46504-39d8-46a7-9072-3ece6818b4a3">

Durch die Aktivierung dieser Funktion können Sie Ihrem Server eine zusätzliche Sicherheitsebene hinzufügen und unbefugten Zugriff verhindern.

**Konfiguration`LOGGED_IN_ONLY`**

Um diese Funktion zu aktivieren:

1.  Setze die`LOGGED_IN_ONLY=Y`Umgebungsvariable für die`hbbs`Server.
2.  Alternativ kannst du die`--logged-in-only`Option beim Ausführen der`hbbs`Server.

**Beispiel**

Um das zu setzen`LOGGED_IN_ONLY`Umgebungsvariable kannst du die folgende Zeile zu deiner`~/.bashrc`Datei oder Äquivalent:

```bash
export LOGGED_IN_ONLY=Y
```

# RustDesk Server-Programm

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**Download**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**Manuell**](https://rustdesk.com/docs/en/self-host/)

[**FAQ**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

[**Wie man OSS auf Pro migriert**](https://rustdesk.com/docs/en/self-host/rustdesk-server-pro/installscript/#convert-from-open-source)

Hosten Sie Ihren eigenen RustDesk-Server selbst, er ist kostenlos und Open Source.

## Wie man manuell baut

Zuerst brauchst du eine funktionierende Rust-Entwicklungs-Toolchain und eine Node ≥ 20 funktionierende Installation.

*   Unices (Linux, MacOS usw.):

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   Fenster mit cmd.exe Hülle:

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

Drei ausführbare Dateien werden in Target/Release generiert.

*   hbbs – RustDesk ID/Rendezvous-Server mit API-Server
*   hbbr – RustDesk-Relaisserver
*   rustdesk-utils – RustDesk CLI-Utilities

Aktualisierte Binärdateien finden Sie auf dem[Veröffentlichungen](https://github.com/sctg-development/sctgdesk-server/releases)Page.

Alle veröffentlichten Binärdateien nach Release v1.1.99-40 werden mit Github Actions attestiert. Sie können die Attestation überprüfen, indem Sie die sha256-Summe der Binärdatei mit`https://search.sigstore.dev/?hash=<sha256>`Zum Beispiel.

Wenn du zusätzliche Funktionen möchtest,[RustDesk Server Pro](https://rustdesk.com/pricing.html)Vielleicht passt es dir besser.

Wenn du deinen eigenen Server entwickeln möchtest,[rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo)Vielleicht ist es für dich ein besserer und einfacherer Einstieg als dieses Depot.

## Installation

Docker-Images werden automatisch auf jeder GitHub-Version generiert und veröffentlicht.

Diese Bilder sind gegen die Konstruktion aufgebaut`ubuntu-22.04`mit der einzigen Ergänzung der Hauptbinäre (`hbbr`und`hbbs`). Sie sind verfügbar auf[Docker-Hub](https://hub.docker.com/r/sctg/sctgdesk-server/)Mit diesen Tags:

| Architektur | Bild:Tag |
| --- | --- |
| amd64 |`sctg/sctgdesk-server:latest`|
| arm64v8 |`sctg/sctgdesk-server:latest`|
| arm32v7 |`sctg/sctgdesk-server:latest`|

Sie können diese Bilder direkt mit beginnen`docker run`mit diesen Befehlen:

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

oder ohne`--net=host`, aber P2P-Direktverbindung kann nicht funktionieren.

Für Systeme, die SELinux verwenden, Ersatz`/root`von`/root:z`ist erforderlich, damit die Container korrekt laufen. Alternativ kann die Containertrennung von SELinux komplett deaktiviert werden, indem die Option hinzugefügt wird`--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

Die`relay-server-ip`Parameter ist die IP-Adresse (oder DNS-Name) des Servers, der diese Container ausführt. Der**optional** `port`Der Parameter muss verwendet werden, wenn man einen anderen Port verwendet**21117**für`hbbr`.

Sie können auch docker-compose verwenden, wobei diese Konfiguration als Vorlage dient:

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

Bearbeiten Sie Zeile 16 so, dass sie auf Ihren Relaisserver verweist (den, der auf Port 21117 zuhört). Sie können auch die Lautstärkelinien (Zeile 18 und Zeile 33) bearbeiten, wenn Sie möchten.

(Docker-Compose-Credit geht an @lukebarone und @QuiGonLeong)

> \[! HINWEIS]\
> Der rustdesk/rustdesk-server:latest in China kann durch die neueste Versionsnummer auf Docker Hub ersetzt werden, wie zum Beispiel`rustdesk-server:1.1.10-3`. Andernfalls könnte die alte Version aufgrund der Bildbeschleunigung entfernt werden.

> Beachten Sie, dass hier die sctg/sctgdesk-server-server:latest in China durch die neueste Versionsnummer auf dockerhub ersetzt werden kann, wie sctg/sctgdesk-server-server:1.1.99-37. Andernfalls kann die alte Version aufgrund der Bildbeschleunigung entfernt werden.

## Wie man ein Schlüsselpaar erstellt

Für die Verschlüsselung wird ein Schlüsselpaar benötigt; Sie können es bereitstellen, wie bereits erklärt, aber Sie brauchen eine Möglichkeit, eines zu erstellen.

Du kannst diesen Befehl verwenden, um ein Schlüsselpaar zu generieren:

```bash
/usr/bin/rustdesk-utils genkeypair
```

Wenn du das nicht hast (oder nicht willst)`rustdesk-utils`Auf Ihrem System installiert, können Sie denselben Befehl mit Docker aufrufen:

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

Die Ausgabe wird ungefähr so aussehen:

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## Pakete

Für jede Binärdatei gibt es separate .deb-Pakete, die Sie in der[Veröffentlichungen](https://github.com/sctg-development/sctgdesk-server/releases). 
Diese Pakete sind für die folgenden Distributionen gedacht:

*   Ubuntu 22.04 LTS
*   MacOS Intel oder Apple Silicon
*   Windows x86\_64 oder i686

## ENV-Variablen

`hbbs`und`hbbr`können mit diesen ENV-Variablen konfiguriert werden. 
Du kannst die Variablen wie gewohnt angeben oder ein verwenden`.env`Akte.

| Variable | binär | Beschreibung |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | wenn gesetzt auf**"Y"**Erlaubt keine direkte Peer-Verbindung |
| DOWNGRADE_START_CHECK | hbbr | Verzögerung (in Sekunden) vor Downgrade-Check |
| DOWNGRADE_THRESHOLD | HBBR | Schwelle der Downgrade-Prüfung (bit/ms) |
| SCHLÜSSEL | hbbs/hbbr | wenn gesetzt ist, erzwingen Sie die Verwendung eines bestimmten Schlüssels, wenn gesetzt auf**"\_"**Erzwingen Sie die Verwendung eines beliebigen Schlüssels |
| LIMIT_SPEED | HBBR | Geschwindigkeitsbegrenzung (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | Pfad für OAUTH2 Konfigurationsdatei |
| OAUTH2\_CREATE_USER | hbbs | wenn gesetzt auf**"1"**Erstelle einen Nutzer, falls es ihn nicht gibt |
| PORT | hbbs/hbbr | Hörport (21116 für HBBS - 21117 für HBBR) |
| RELAY | hbbs | IP-Adresse/DNS-Name der Maschinen, die HBBR ausführen (durch Komma getrennt) |
| RUST_LOG | alle | Set Debug Level (fehler|warn|info|debug|trace) |
| S3CONFIG_FILE | hbbs | Pfad für s3-Konfigurationsdatei |
| SINGLE_BANDWIDTH | hbbr | maximale Bandbreite für eine einzelne Verbindung (in Mb/s) |
| TOTAL_BANDWIDTH | hbbr | maximale Gesamtbandbreite (in Mb/s) |
