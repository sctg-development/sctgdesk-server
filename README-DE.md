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

SCTGDesk ist ein selbstgehosteter Remote-Desktop-Server mit eigenen Clients, abgeleitet von[RustDesk](https://github.com/rustdesk/rustdesk). Es handelt sich um freie Software unter der**AGPL-3.0**.

> **SCTGDesk ist ein unabhängiges Projekt. Es ist weder mit dem RustDesk-Projekt verbunden, davon unterstützt noch gefördert
> oder Purslane Ltd.**"RustDesk" ist der Name dieses Projekts; er wird hier nur verwendet, um auszudrücken, was diese Software ist
> abgeleitet von und kompatibel mit.

Dieses Repository ist das gesamte Projekt: der Server, der API-Server, die Webkonsole und das Client-Overlay. Es ersetzt
Die ehemaligen Repositories`sctgdesk-api-server`und`sctgdesk`, die archiviert sind.

## Was du bekommst

*   Der RustDesk-Server:`hbbs`(ID und Rendezvous-Server) und`hbbr`(Staffel), plus`rustdesk-utils`(Schlüsselpaare, 
    Konnektivitätsprüfung). Kompatibel mit den offiziellen RustDesk-Clients.
*   Die Serverseite der**verschlüsselte TCP- und WebSocket-Verbindung**(Schlüsselaustauschversionen 0 und 1), die die
    Open-Source-Upstream-Server wird nicht implementiert. Das ist es, was die**Webclient**Arbeit.
*   A**REST API**eingebettet in`hbbs`, kompatibel mit den Clients: Login (Passwort, OpenID Connect / OAuth2: 
    GitHub, GitLab, Google, Dex...), Sitzungen, die einen Neustart überstehen, persönliche und gemeinsame Adressbücher (pro Nutzer und
    pro Gruppe, nur Lese-/Schreib- / Admin-Optionen), Benutzer und Gruppen, Software-Update-Endpunkte.
*   A**Webkonsole**um all das zu verwalten, sowie interaktive API-Dokumentation.
*   A**Verbindungshistorie**mit einer Anmerkung zu jeder Verbindung (was gemacht wurde), durchsuchbar und exportierbar in CSV, ausgefüllt von den RustDesk-Clients.
*   Eine Option, eine nicht authentifizierte Kontrolle abzulehnen (`--logged-in-only`).
*   A**Client**(`client/`): Upstream RustDesk plus ein kleines Overlay, das zum Build-Zeitpunkt mit Ihrem Namen, Servern und
    Schlüssel; und eine**Webclient**.

Kompatibilität steht an erster Stelle: Der Server muss weiterhin mit offiziellen RustDesk-Clients und unseren Änderungen im Upstream funktionieren
Dateien werden klein gehalten, damit neue Upstream-Releases zusammengeführt werden können.

## Installieren

Such dir einen aus. Was auch immer du wählst, du brauchst einen**Schlüsselpaar**und die Ports darunter.

### Linux-Pakete (deb, rpm) und Paket-Repositories

Jede Veröffentlichung hat`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`und`sctgdesk-server-utils`für**amd64**,**arm64**und**armv7**(32-Bit-ARM,`armhf`), als`.deb`und`.rpm`, aufgebaut aus statischen Musl-Binärdateien (keine Bibliotheksabhängigkeit). Sie
ersetzen Sie den erstgenannten`rustdesk-server-*`Pakete.

**Aus den signierten Repositories**(Updates mit dem System):

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

**Oder aus den Dateien einer Veröffentlichung:**

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

Eine erste Installation in einem leeren Datenverzeichnis startet beide Dienste mit einem zufälligen Schlüsselpaar; Optionen gehen in`/etc/default/sctgdesk-hbbs`und`-hbbr`(siehe[Der Bereitstellungsleitfaden](docs/deployment.md), was auch die
Migration aus dem ehemaligen`rustdesk-server-*`Pakete:**Lösche sie nicht**). Ein Upgrade startet die Dienste neu
die laufen. Daten sind drin`/var/lib/rustdesk-server`, anmeldet sich`/var/log/rustdesk-server`; Die Einheiten laufen als Root.

Die Repositories und die Signierschlüssel sind beschrieben in[Paket-Repositories](docs/repositories.md).

### OpenWrt

Für OpenWrt 25.12 und höher (`apk`), auf x86\_64, ARM (aarch64, armv7) und MIPS (`mips_24kc`Für ath79,`mipsel_24kc`für
RAMIPS)-Router:

```sh
wget -O /etc/apk/keys/sctgdesk.rsa.pub https://sctg-development.github.io/sctgdesk-server/apk/sctgdesk.rsa.pub
. /etc/openwrt_release        # DISTRIB_ARCH: the architecture of the packages of your router, for example mips_24kc
echo "https://sctg-development.github.io/sctgdesk-server/apk/$DISTRIB_ARCH/APKINDEX.tar.gz" > /etc/apk/repositories.d/sctgdesk.list
apk update && apk add sctgdesk-server-hbbs sctgdesk-server-hbbr
```

Jedes Programm ist ein`procd`Service, konfiguriert mit UCI (`/etc/config/sctgdesk-hbbs`).`hbbs`sind etwa 30 MB: setze es ein, und
es sind Daten auf einer USB-Festplatte oder einem Extroot. Die MIPS-Builds sind auf echten Routern ungetestet. Ältere OpenWrt-Versionen und andere
Architekturen können die statischen Binärdateien einer Version verwenden. Details:[Paket-Repositories](docs/repositories.md).

### Binärdateien (Linux, Windows, macOS)

Die[Veröffentlichungen](https://github.com/sctg-development/sctgdesk-server/releases)Die Seite verfügt außerdem über ZIP-Archive mit`hbbs`,`hbbr`und`rustdesk-utils`für Windows (amd64, arm64) und macOS (Intel, Apple Silicon), mit`SHA256SUMS-*`Dateien. 
Entpacken Sie sie und führen Sie sie dann aus dem Verzeichnis aus, in dem sich die Daten befinden sollten (siehe[Erster Lauf](#first-run)).

On**Windows**Sie können installiert werden mit`winget install SCTG.SCTGDeskServer --scope machine`und als Dienste betrieben werden
(`hbbs.exe --windows-service`, erstellt mit`sc.exe`): siehe[Unter Windows](docs/windows.md).

### Docker

Bilder werden veröffentlicht am[Docker-Hub](https://hub.docker.com/r/sctg/sctgdesk-server/)als`sctg/sctgdesk-server`. 
Speichern als`docker-compose.yml`, als Ersatz`sctgdesk.example.com`Mit der Adresse Ihres Servers:

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

Für SELinux fügen Sie hinzu`:z`auf das Volumen oder Verwenden`--security-opt label=disable`. Ohne`--net=host`Direkte Peer-to-Peer
Verbindungen funktionieren nicht (alles läuft durch das Relais). Ein Helmdiagramm ist in`deploy/helm`(zu refaktorisieren) und
ein Beispielmanifest in`kubernetes/`.

### Ports

| Port | Protokoll | Verwendet von | Zweck |
|---|---|---|---|
| 21114 | TCP |`hbbs`| REST API, Webkonsole, API-Dokumentation (`--api-port`) |
| 21115 | TCP |`hbbs`| NAT-Typtest |
| 21116 | TCP +**UDP**|`hbbs`| ID / Rendezvous |
| 21117 | TCP |`hbbr`| Relay |
| 21118 | TCP |`hbbs`| WebSocket (Webclient) |
| 21119 | TCP |`hbbr`| WebSocket (Webclient) |

Öffnen Sie sie in Ihrer Firewall. Die API- und WebSocket-Ports sollten hinter einem TLS-Reverse-Proxy im Internet liegen: siehe
Die[Bereitstellungsleitfaden](docs/deployment.md).

## Erster Lauf

`hbbs`und`hbbr`Bewahren Sie ihre Daten (Schlüsselpaar, SQLite-Datenbank) in der**Aktuelles Verzeichnis**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

Beim ersten Start`hbbs`erstellt`id_ed25519`(geheim) und`id_ed25519.pub`(die**Öffentlicher Schlüssel**, die du gibst
Ihre Kunden). Um selbst ein Schlüsselpaar zu erstellen oder eines zu überprüfen:

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

Um Clients abzulehnen, die den Schlüssel nicht haben, starten Sie beide Server mit`-k <key>`(oder`KEY=<key>`). Überprüfen Sie, dass ein
Der Server antwortet auf allen seinen Ports mit`rustdesk-utils doctor <server>`.

**Standard-Admin.**Die Webkonsole ist bei`http://<server>:21114/`und die API-Dokumentation unter`http://<server>:21114/api/doc/`. Eine neue Datenbank hat den Benutzer`admin`mit dem Passwort`Hello,world!`, was ist
Öffentlichkeit:**Ändern Sie es beim ersten Login**.

## Konfiguration

`hbbs`und`hbbr`Optionen auf der Befehlszeile wählen (`hbbs --help`) und aus Umweltvariablen, oder einem`.env`Akte.

| Variable | Binärsystem | Beschreibung |
| --- | --- | --- |
| ALWAYS_USE_RELAY | HBBS | wenn auf eingestellt**"Y"**Direkte Peer-Verbindung wird nicht erlaubt |
| DB_URL | hbbs | Pfad der SQLite-Datenbank (Standard`./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | HBBS | wenn auf eingestellt**"Y"**nur angemeldete Benutzer können einen Peer steuern (wie auch`--logged-in-only`) |
| ALLOW_REMOTE_BACKUP | HBBS | wenn eingestellt auf**"Y"**, ein Administrator kann ein Backup herunterladen (Datenbank, Schlüsselpaar, Konfiguration) mit`GET /api/backup`oder von der Konsole aus (gleich wie`--allow-remote-backup`); standardmäßig ausgeschaltet, siehe[Der Bereitstellungsleitfaden](docs/deployment.md#download-a-backup)|
| REQUIRE_KX_V1 | hbbs | wenn auf eingestellt**"Y"**lehnt Clients ab, die den alten (Version 0) TCP-Schlüsselaustausch verwenden (wie`--require-kx-v1`) |
| DOWNGRADE_START_CHECK | HBBR | Verzögerung (in Sekunden) vor Downgrade-Überprüfung |
| DOWNGRADE_THRESHOLD | HBBR | Schwellenwert für eine Downgrade-Überprüfung (Bit/ms) |
| SCHLÜSSEL | hbbs/hbbr | wenn gesetzt, erzwingen Sie die Verwendung eines bestimmten Schlüssels, wenn auf gesetzt ist**"\_"**Die Verwendung beliebiger Taste erzwingen |
| LIMIT_SPEED | hbbr | Geschwindigkeitsbegrenzung (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | Pfad der OAuth2 / OpenID Connect Providerdatei (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | HBBS | wenn auf eingestellt**"1"**Einen Benutzer beim ersten Login mit einem Anbieter erstellen |
| PORT | HBBS/HBBR | Hörport (21116 für HBBS - 21117 für HBBR) |
| HEALTH_PORT | hbbr | Wenn aktiviert, antwortet HBBR`GET /livez`und`GET /readyz`(HTTP) auf diesem Port; standardmäßig deaktiviert |
| RELAY | HBBS | IP-Adresse/DNS-Name der Maschinen, die HBBR ausführen (durch Komma getrennt) |
| LOG_FORMAT | HBBS, HBB |**"json"**: ein JSON-Objekt pro Logzeile (`time`,`level`,`target`,`file`,`line`,`msg`); Standard: Text |
| RUST_LOG | alle | Debug-Level festlegen (Fehler|Warnung|Info|Debug|Trace) |
| ROCKET_SECRET_KEY | hbbs | Schlüssel, der die Cookies des API-Servers signiert (base64, 64 Bytes). Standard: beim ersten Start generiert in`secret_key.txt`neben der Datenbank |
| S3CONFIG_FILE | hbbs | Pfad der Software-Update-Datei (`s3config.toml`) |
| SINGLE_BANDWIDTH | hbbr | maximale Bandbreite für eine einzelne Verbindung (in Mb/s) |
| TOTAL_BANDWIDTH | HBBR | maximale Gesamtbandbreite (in Mb/s) |

### Nur angemeldete Benutzer können die Kontrolle übernehmen

`--logged-in-only`(oder`LOGGED_IN_ONLY=Y`) macht`hbbs`fragen Sie den API-Server, ob das Token des Clients ein Live ist
Sitzung, bevor es zwei Peers verbindet. Jeder kann sein Gerät weiterhin registrieren, aber niemand kann ein Gerät steuern
ohne sich anzumelden. Ein Client, der nicht eingeloggt ist, sieht "Die Verbindung ist nicht erlaubt. Sie haben sich nicht angemeldet."; eins
deren Sitzung beendet ist: "Ihre Sitzung ist abgelaufen."

### Melden Sie sich mit GitHub, GitLab, Google, Dex... an

Kopiert`oauth2.toml`und füllen die von Ihnen verwendeten Anbieter aus (jeder benötigt einen Antrag, der beim Anbieter registriert ist, dessen Anbieter
Callback ist dein API-Server), dann setze`OAUTH2_CONFIG_FILE=oauth2.toml`, und`OAUTH2_CREATE_USER=1`um die
Benutzer beim ersten Login. Siehe die[Bereitstellungsleitfaden](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Updates des Clients

Die Clients können auf Ihrem API-Server nach neuen Versionen suchen statt auf GitHub: Das antwortet`POST /api/software/version/latest`(die Überprüfung der aktuellen RustDesk-Kunden) und`GET /api/software/releases/latest`. 
Die neueste Version und die Download-Links werden aus`s3config.toml`(S3-kompatibler Speicher), siehe[die REST API](docs/api.md#software-update-endpoints).

## Kunden

*   **Offizielle RustDesk-Kunden**Arbeiten Sie mit diesem Server: Geben Sie ihnen Ihren Server und Ihren öffentlichen Schlüssel.
*   **Der SCTGDesk-Client**ist Upstream-RustDesk plus unser Overlay (Name, Icons, Standardserver und Schlüssel). Du baust es selbst
    Für Ihre eigenen Server siehe[Benutzerdefinierte Builds](docs/custom-builds.md#the-client). Die**Webclient**ist gebaut aus
    Das gleiche Overlay.

## Bau es selbst

Du brauchst eine Rust-Toolchain und für die Webkonsole Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Set`SCTGDESK_SKIP_WEB_BUILD=1`ohne Node.js zu bauen (die Konsole dient dann als Platzhalter). Die[Entwicklerhandbuch](https://sctg-development.github.io/sctgdesk-server/)umfasst Aufbau, Test, benutzerdefinierte Server und
Client-Builds, Packaging, Releases und Synchronisierung mit Upstream.

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

### Im Client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Sicherheit

*   Der eingebettete API-Server ist nicht vor Missbrauch geschützt: Stellen Sie einen TLS-Reverse-Proxy (HAProxy, NGINX, Caddy...) vor
    davon und von den WebSocket-Ports. Ein HAProxy-Beispiel findet sich in der[Bereitstellungsleitfaden](docs/deployment.md#tls-reverse-proxy).
*   Standard-Admin-Passwort ändern, behalten`id_ed25519`Geheim und sichern: Ändern führt dazu, dass jeder Client ihn ablehnt
    Deinen Server.
*   Melden Sie Schwachstellen wie beschrieben in[SECURITY.md](SECURITY.md).

## Lizenz und Quellcode

SCTGDesk wird unter der[GNU Affero General Public Lizenz v3](LICENSE). Der Quellcode jeder Veröffentlichung
ist in diesem Repository, unter dem Tag dieser Version. Wenn Sie diese Software ändern und anderen Menschen erlauben, sie über einen
Network, Abschnitt 13 der Lizenz verlangt, dass Sie ihnen den entsprechenden Quellcode Ihrer modifizierten Version bereitstellen.

Überprüfen Sie die mit jeder Version veröffentlichten SHA-256-Summen, bevor Sie eine heruntergeladene Binärdatei ausführen.

## Beitrag

[Entwicklerhandbuch](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md). Für allgemeine Fragen zum Self-Hosting, die[RustDesk-Dokumentation](https://rustdesk.com/docs/en/self-host/)gilt ebenfalls.

Wenn dir dieses Werk gefällt, gib ihm einen Stern oder[Sponsoring](https://github.com/sponsors/sctg-development).
