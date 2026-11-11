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

SCTGDesk ist ein selbstgehosteter Remote-Desktop-Server mit eigenen Clients, abgeleitet von[RustDesk](https://github.com/rustdesk/rustdesk). Es ist freie Software unter dem**AGPL-3.0**.

> **SCTGDesk ist ein unabhängiges Projekt. Es ist weder mit dem RustDesk-Projekt verbunden, unterstützt noch von diesem gesponsert
> oder Purslane Ltd.**"RustDesk" ist der Name dieses Projekts; hier wird er nur verwendet, um anzugeben, was diese Software ist
> abgeleitet von und kompatibel mit.

Dieses Repository ist das gesamte Projekt: der Server, der API-Server, die Webkonsole und das Client-Overlay. Es ersetzt
Die ehemaligen Lagerstätten`sctgdesk-api-server`und`sctgdesk`, die archiviert sind.

## Was du bekommst

*   Der RustDesk-Server:`hbbs`(ID und Rendezvous-Server) und`hbbr`(Relais), plus`rustdesk-utils`(Schlüsselpaare, 
    Verbindungskontrolle). Kompatibel mit den offiziellen RustDesk-Kunden.
*   Die Serverseite der**verschlüsselte TCP- und WebSocket-Verbindung**(Schlüsselaustausch-Versionen 0 und 1), wobei die
    Der Open-Source-Upstream-Server implementiert nicht. Das ist es, was die**Webclient**Arbeit.
*   A**REST API**eingebettet in`hbbs`, kompatibel mit den Clients: Login (Passwort, OpenID Connect / OAuth2: 
    GitHub, GitLab, Google, Dex...), Sitzungen, die einen Neustart überstehen, persönliche und geteilte Adressbücher (pro Nutzer und
    pro Gruppe, Schreibgeschützt / Lesen-Schreiben / Administrator), Benutzer und Gruppen, Software-Update-Endpunkte.
*   A**Webkonsole**um all das zu verwalten, und interaktive API-Dokumentation.
*   A**Verbindungsgeschichte**mit einer Notiz zu jeder Verbindung (was gemacht wurde), durchsuchbar und exportierbar in CSV, ausgefüllt von den RustDesk-Clients.
*   Eine Option, um nicht authentifizierte Kontrolle abzulehnen (`--logged-in-only`).
*   A**Auftraggeber**(`client/`): Upstream RustDesk plus ein kleines Overlay, das beim Bau mit deinem Namen, Servern und
    Schlüssel; und ein**Webclient**.

Die Kompatibilität steht an erster Stelle: Der Server muss weiterhin mit offiziellen RustDesk-Clients funktionieren, und unsere Änderungen auf Upstream
Dateien werden klein gehalten, damit neue Upstream-Veröffentlichungen zusammengeführt werden können.

## Installation

Such dir eine aus. Was auch immer du wählst, du brauchst eine**Schlüsselpaar**und die darunterliegenden Anschlüsse.

### Linux-Pakete (deb, rpm)

Jede Veröffentlichung hat`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`und`sctgdesk-server-utils`für**amd64**und**arm64**, als`.deb`und`.rpm`, gebaut aus statischen Musl-Binärdateien (keine Bibliotheksabhängigkeit). Sie ersetzen die ersteren`rustdesk-server-*`Pakete.

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

Eine erste Installation startet nichts: Bearbeite die Geräte (`systemctl edit --full rustdesk-hbbs`, siehe[Der Einsatzleitfaden](docs/deployment.md)), dann`sudo systemctl enable --now rustdesk-hbbs rustdesk-hbbr`. Ein
Das Upgrade startet die laufenden Dienste neu. Die Daten sind drin`/var/lib/rustdesk-server`, loggt sich ein`/var/log/rustdesk-server`; Die Einheiten laufen als Root.

### Binärdateien (Linux, Windows, macOS)

Die[Veröffentlichungen](https://github.com/sctg-development/sctgdesk-server/releases)Die Seite verfügt außerdem über ZIP-Archive mit`hbbs`,`hbbr`und`rustdesk-utils`für Windows (amd64, arm64) und macOS (Intel, Apple Silicon), mit`SHA256SUMS-*`Akten. 
Entpacken Sie sie und führen Sie sie dann aus dem Verzeichnis aus, in dem sich die Daten befinden sollten (siehe[Erster Lauf](#first-run)).

### Docker

Bilder werden veröffentlicht auf[Docker-Hub](https://hub.docker.com/r/sctg/sctgdesk-server/)als`sctg/sctgdesk-server`. 
Speichern als`docker-compose.yml`, ersetzen`sctgdesk.example.com`Mit der Adresse Ihres Servers:

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

Für SELinux anhängen`:z`auf das Volumen oder die Nutzung`--security-opt label=disable`. Ohne`--net=host`Direkter Peer-to-Peer
Verbindungen funktionieren nicht (alles läuft über das Relais). Ein Helm-Diagramm ist installiert`deploy/helm`(noch umstrukturiert) und
Ein Beispiel, das sich manifestiert in`kubernetes/`.

### Ports

| Port | Protokoll | Verwendet von | Zweck |
|---|---|---|---|
| 21114 | TCP |`hbbs`| REST-API, Webkonsole, API-Dokumentation (`--api-port`) |
| 21115 | TCP |`hbbs`| NAT-Typtest |
| 21116 | TCP +**UDP**|`hbbs`| ID / Rendezvous |
| 21117 | TCP |`hbbr`| Staffel |
| 21118 | TCP |`hbbs`| WebSocket (Web-Client) |
| 21119 | TCP |`hbbr`| WebSocket (Web-Client) |

Öffne sie auf deiner Firewall. Die API- und WebSocket-Ports sollten hinter einem TLS-Reverse-Proxy im Internet liegen: siehe
die[Einsatzleitfaden](docs/deployment.md).

## Erster Lauf

`hbbs`und`hbbr`bewahren Sie ihre Daten (Schlüsselpaar, SQLite-Datenbank) in der**Aktuelles Verzeichnis**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

Beim ersten Start`hbbs`Schafft`id_ed25519`(geheim) und`id_ed25519.pub`(die**Öffentlicher Schlüssel**, die du gibst
Ihre Kunden). Um selbst ein Schlüsselpaar herzustellen oder eines zu überprüfen:

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

Um Clients, die den Schlüssel nicht besitzen, abzulehnen, starten Sie beide Server mit`-k <key>`(oder`KEY=<key>`). Überprüfen Sie, dass ein
Der Server antwortet auf allen seinen Ports mit`rustdesk-utils doctor <server>`.

**Standard-Admin.**Die Webkonsole ist bei`http://<server>:21114/`und die API-Dokumentation bei`http://<server>:21114/api/doc/`. Eine neue Datenbank hat den Benutzer`admin`mit dem Passwort`Hello,world!`, was ist
Öffentlich:**Ändere es beim ersten Login**.

## Konfiguration

`hbbs`und`hbbr`Wähle Optionen in der Kommandozeile (`hbbs --help`) und aus Umweltvariablen, oder einem`.env`Akte.

| Variable | binär | Beschreibung |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | wenn gesetzt auf**"Y"**Erlaubt keine direkte Peer-Verbindung |
| DB_URL | hbbs | Pfad der SQLite-Datenbank (Standard`./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | hbbs | wenn gesetzt auf**"Y"**Nur eingeloggte Nutzer können einen Peer steuern (wie auch`--logged-in-only`) |
| REQUIRE_KX_V1 | hbbs | wenn gesetzt auf**"Y"**lehnt Clients ab, die den alten (Version 0) TCP-Schlüsselaustausch verwenden (gleich wie`--require-kx-v1`) |
| DOWNGRADE_START_CHECK | hbbr | Verzögerung (in Sekunden) vor Downgrade-Check |
| DOWNGRADE_THRESHOLD | HBBR | Schwelle der Downgrade-Prüfung (bit/ms) |
| SCHLÜSSEL | hbbs/hbbr | wenn gesetzt ist, erzwingen Sie die Verwendung eines bestimmten Schlüssels, wenn gesetzt auf**"\_"**Erzwingen Sie die Verwendung eines beliebigen Schlüssels |
| LIMIT_SPEED | HBBR | Geschwindigkeitsbegrenzung (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | Pfad der OAuth2 / OpenID Connect Provider-Datei (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | hbbs | wenn gesetzt auf**"1"**Einen Benutzer beim ersten Login mit einem Anbieter erstellen |
| PORT | hbbs/hbbr | Hörport (21116 für HBBS - 21117 für HBBR) |
| HEALTH_PORT | hbbr | wenn gesetzt, antwortet hbbr`GET /livez`und`GET /readyz`(HTTP) an diesem Port; standardmäßig ausgeschaltet |
| RELAY | hbbs | IP-Adresse/DNS-Name der Maschinen, die HBBR ausführen (durch Komma getrennt) |
| RUST_LOG | alle | Set Debug Level (fehler|warn|info|debug|trace) |
| ROCKET_SECRET_KEY | hbbs | Schlüssel, der die Cookies des API-Servers signiert (base64, 64 Bytes). Standard: beim ersten Start in`secret_key.txt`neben der Datenbank |
| S3CONFIG_FILE | hbbs | Pfad der Software-Update-Datei (`s3config.toml`) |
| SINGLE_BANDWIDTH | hbbr | maximale Bandbreite für eine einzelne Verbindung (in Mb/s) |
| TOTAL_BANDWIDTH | hbbr | maximale Gesamtbandbreite (in Mb/s) |

### Nur angemeldete Nutzer können steuern

`--logged-in-only`(oder`LOGGED_IN_ONLY=Y`) macht`hbbs`fragen Sie den API-Server, ob das Token des Clients live ist
Sitzung, bevor es zwei Peers verbindet. Jeder kann sein Gerät weiterhin registrieren, aber niemand kann ein Gerät steuern
ohne sich einzuloggen. Ein Client, der nicht eingeloggt ist, sieht "Die Verbindung ist nicht erlaubt. Sie haben sich nicht eingeloggt."; eins
deren Sitzung beendet wurde, sieht "Ihre Sitzung abgelaufen."

### Melden Sie sich bei GitHub, GitLab, Google, Dex an...

Verstanden`oauth2.toml`und fülle die von dir genutzten Anbieter aus (jeder benötigt eine Anmeldung beim jeweiligen Anbieter, dessen Anbieter registriert ist
callback ist dein API-Server), dann setzen`OAUTH2_CONFIG_FILE=oauth2.toml`, und`OAUTH2_CREATE_USER=1`um die zu schaffen
Benutzer beim ersten Login. Siehe die[Einsatzleitfaden](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Aktualisierungen des Clients

Die Clients können auf deinem API-Server nach neuen Versionen suchen, statt auf GitHub: Es antwortet`POST /api/software/version/latest`(die Überprüfung aktueller RustDesk-Kunden) und`GET /api/software/releases/latest`. 
Die neueste Version und die Download-Links werden gelesen`s3config.toml`(S3-kompatibler Speicher), siehe[die REST-API](docs/api.md#software-update-endpoints).

## Kunden

*   **Offizielle RustDesk-Kunden**Arbeiten Sie mit diesem Server zusammen: Geben Sie ihnen Ihren Server und Ihren öffentlichen Schlüssel.
*   **Der SCTGDesk-Client**ist Upstream RustDesk plus unser Overlay (Name, Icons, Standardserver und Schlüssel). Du baust es selbst
    Für Ihre eigenen Server siehe[Eigene Builds](docs/custom-builds.md#the-client). Die**Webclient**wird aufgebaut aus
    Das gleiche Overlay.

## Bau es selbst

Du brauchst eine Rust-Toolchain und für die Webkonsole Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Set`SCTGDESK_SKIP_WEB_BUILD=1`ohne Node.js zu bauen (die Konsole ist dann ein Platzhalter). Die[Entwicklerhandbuch](https://sctg-development.github.io/sctgdesk-server/)Deckt den Bau, das Testen, den kundenspezifischen Server und
Client-Builds, Packaging, Releases und Synchronisation mit Upstream.

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

*   Der eingebettete API-Server ist nicht vor Missbrauch geschützt: Setze einen TLS-Reverse-Proxy (HAProxy, NGINX, Caddy ...) vorne
    von ihm, und von den WebSocket-Ports. Ein Beispiel von HAProxy findet sich im[Einsatzleitfaden](docs/deployment.md#tls-reverse-proxy).
*   Das Standard-Admin-Passwort ändern, behalten`id_ed25519`geheim und sichern Sie es: Eine Änderung führt dazu, dass jeder Kunde ablehnt
    Dein Server.
*   Melden Sie Schwachstellen wie beschrieben in[SECURITY.md](SECURITY.md).

## Lizenz und Quellcode

SCTGDesk wird unter dem[GNU Affero General Public License v3](LICENSE). Der Quellcode jeder Veröffentlichung
ist in diesem Repository, am Tag dieser Veröffentlichung. Wenn du diese Software modifizierst und anderen erlaubst, sie über eine Veröffentlichung zu verwenden.
Network verlangt Abschnitt 13 der Lizenz, dass du ihnen den entsprechenden Quellcode deiner modifizierten Version vorlegst.

Überprüfen Sie die mit jeder Version veröffentlichten SHA-256-Summen, bevor Sie eine heruntergeladene Binärdatei ausführen.

## Beitrag

[Entwicklerhandbuch](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md). Für allgemeine Fragen zum Selbsthosting gilt die[RustDesk-Dokumentation](https://rustdesk.com/docs/en/self-host/)Gilt auch.

Wenn dir dieses Werk gefällt, überlege, ihm einen Stern zu geben, oder[Sponsoring](https://github.com/sponsors/sctg-development).
