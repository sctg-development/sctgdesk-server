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

SCTGDesk is een zelfgehoste remote-desktop server, met eigen clients, afgeleid van[RustDesk](https://github.com/rustdesk/rustdesk). Het is gratis software onder de**AGPL-3.0**.

> **SCTGDesk is een onafhankelijk project. Het is niet verbonden aan, goedgekeurd door of gesponsord door het RustDesk-project
> of Purslane Ltd.**"RustDesk" is de naam van dat project; het wordt hier alleen gebruikt om aan te geven wat deze software is
> afgeleid van en compatibel met.

Deze repository is het hele project: de server, de API-server, de webconsole en de clientoverlay. Het vervangt
De voormalige bewaarplaatsen`sctgdesk-api-server`en`sctgdesk`, die gearchiveerd zijn.

## Wat krijg je

*   De RustDesk-server:`hbbs`(ID en rendezvous-server) en`hbbr`(relais), plus`rustdesk-utils`(sleutelparen, 
    connectiviteitscontrole). Compatibel met de officiële RustDesk-clients.
*   De serverzijde van de**versleutelde TCP- en WebSocket-verbinding**(sleutelwisselversies 0 en 1), waarbij de
    een open-source upstream server implementeert niet. Dit is wat de**Webclient**werk.
*   A**REST API**Ingesloten in`hbbs`, compatibel met de clients: login (wachtwoord, OpenID Connect / OAuth2: 
    GitHub, GitLab, Google, Dex...), sessies die een herstart overleven, persoonlijke en gedeelde adresboeken (per gebruiker en
    per groep, alleen-lezen / lezen-schrijven / beheerders), gebruikers en groepen, software-update endpoints.
*   A**Webconsole**om dat allemaal te beheren, en interactieve API-documentatie.
*   A**Verbindingsgeschiedenis**met een notitie bij elke verbinding (wat er gedaan is), doorzoekbaar en exporteerbaar naar CSV, ingevuld door de RustDesk-clients.
*   Een optie om niet-geauthenticeerde controle te weigeren (`--logged-in-only`).
*   A**Opdrachtgever**(`client/`): upstream RustDesk plus een kleine overlay, gebrandmerkt bij het bouwen met je naam, servers en
    sleutel; en een**Webclient**.

Compatibiliteit komt eerst: de server moet blijven werken met officiële RustDesk-clients, en onze wijzigingen naar upstream
Bestanden worden klein gehouden zodat nieuwe upstream-releases kunnen worden samengevoegd.

## Installatie

Kies er één. Wat je ook kiest, je hebt een**Sleutelpaar**en de poorten eronder.

### Linux-pakketten (deb, rpm)

Elke release heeft`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`en`sctgdesk-server-utils`voor**amd64**en**arm64**, als`.deb`en`.rpm`, opgebouwd uit statische musl-binaire bestanden (geen bibliotheekafhankelijkheid). Ze vervangen de vorige`rustdesk-server-*`pakketten.

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

Een eerste installatie start niets: bewerk de units (`systemctl edit --full rustdesk-hbbs`, zie[De Implementatiegids](docs/deployment.md)), dan`sudo systemctl enable --now rustdesk-hbbs rustdesk-hbbr`. Een
De upgrade start de diensten die draaiden opnieuw op. Data is erin`/var/lib/rustdesk-server`, logt in`/var/log/rustdesk-server`; De units draaien als root.

### Binaire bestanden (Linux, Windows, macOS)

De[Uitgaven](https://github.com/sctg-development/sctgdesk-server/releases)De pagina heeft ook zip-archieven met`hbbs`,`hbbr`en`rustdesk-utils`voor Windows (amd64, arm64) en macOS (Intel, Apple Silicon), met`SHA256SUMS-*`dossiers. 
Pak uit en voer ze vervolgens uit vanuit de map waar de data zou moeten staan (zie[Eerste run](#first-run)).

### Docker

Afbeeldingen worden gepubliceerd op[Docker-hub](https://hub.docker.com/r/sctg/sctgdesk-server/)als`sctg/sctgdesk-server`. 
Sla op als`docker-compose.yml`, vervangen`sctgdesk.example.com`met het adres van je server:

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

Voor SELinux, voeg je toe`:z`naar het volume, of gebruik`--security-opt label=disable`. Zonder`--net=host`directe peer-to-peer
Verbindingen werken niet (alles gaat via het relais). Een helm-kaart staat erin`deploy/helm`(te herstructureren) en
Een voorbeeld manifesteert zich in`kubernetes/`.

### Ports

| Port | Protocol | Gebruikt door | Doel |
|---|---|---|---|
| 21114 | TCP |`hbbs`| REST API, webconsole, API-documentatie (`--api-port`) |
| 21115 | TCP |`hbbs`| NAT-typetest |
| 21116 | TCP +**UDP**|`hbbs`| ID / rendez-vous |
| 21117 | TCP |`hbbr`| relais |
| 21118 | TCP |`hbbs`| WebSocket (webclient) |
| 21119 | TCP |`hbbr`| WebSocket (webclient) |

Open ze op je firewall. De API- en WebSocket-poorten zouden achter een TLS reverse proxy op het internet moeten staan: zie
de[Implementatiegids](docs/deployment.md).

## Eerste run

`hbbs`en`hbbr`bewaar hun gegevens (sleutelpaar, SQLite-database) in de**Huidige directory**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

Bij de eerste start`hbbs`creëert`id_ed25519`(geheim) en`id_ed25519.pub`(de**publieke sleutel**, die je geeft aan
je klanten). Om zelf een sleutelpaar te maken, of er een te controleren:

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

Om clients te weigeren die de sleutel niet hebben, start je beide servers met`-k <key>`(of`KEY=<key>`). Controleer dat een
Server reageert op al zijn poorten met`rustdesk-utils doctor <server>`.

**Standaard beheerder.**De webconsole staat op`http://<server>:21114/`en de API-documentatie op`http://<server>:21114/api/doc/`. Een nieuwe database heeft de gebruiker`admin`met het wachtwoord`Hello,world!`, wat is
Publiek:**Verander het bij de eerste login**.

## Configuratie

`hbbs`en`hbbr`Neem opties op de commandoregel (`hbbs --help`) en uit omgevingsvariabelen, of een`.env`Bestand.

| variabele | binair | beschrijving |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | als ingesteld op**"J"**Staat directe peerverbinding niet toe |
| DB_URL | hbbs | pad van de SQLite-database (standaard`./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | hbbs | als ingesteld op**"J"**Alleen ingelogde gebruikers kunnen een peer bedienen (hetzelfde als`--logged-in-only`) |
| REQUIRE_KX_V1 | hbbs | als ingesteld op**"J"**weigert clients die de legacy (versie 0) TCP-sleuteluitwisseling gebruiken (hetzelfde als`--require-kx-v1`) |
| DOWNGRADE_START_CHECK | hbbr | vertraging (in seconden) voor downgrade check |
| DOWNGRADE_THRESHOLD | hbbr | drempel van downgrade check (bit/ms) |
| SLEUTEL | hbbs/hbbr | als ingesteld is het gebruik van een specifieke sleutel, als ingesteld op**"\_"**Dwing het gebruik van elke sleutel |
| LIMIT_SPEED | hbbr | snelheidslimiet (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | pad van het OAuth2 / OpenID Connect provider-bestand (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | hbbs | als ingesteld op**"1"**maak een gebruiker aan bij de eerste inlog met een provider |
| POORT | HBBS/HBBR | Luisterpoort (21116 voor HBBS - 21117 voor HBBR) |
| RELAY | hbbs | IP-adres/DNS-naam van de machines die hbbr draaien (gescheiden door komma) |
| RUST_LOG | alle | set debug level (fout|waarschuw|info|debug|trace) |
| S3CONFIG_FILE | hbbs | pad van het software-updatebestand (`s3config.toml`) |
| SINGLE_BANDWIDTH | hbbr | maximale bandbreedte voor één enkele verbinding (in Mb/s) |
| TOTAL_BANDWIDTH | HBBB | maximale totale bandbreedte (in Mb/s) |

### Alleen ingelogde gebruikers kunnen de controle nemen

`--logged-in-only`(of`LOGGED_IN_ONLY=Y`) maakt`hbbs`vraag de API-server of het token van de client live is
sessie voordat het twee peers verbindt. Iedereen kan zijn apparaat nog steeds registreren, maar niemand kan een apparaat bedienen
zonder in te loggen. Een client die niet is ingelogd ziet "De verbinding is niet toegestaan. Je bent niet ingelogd."; één
waarvan de sessie eindigde ziet "Uw sessie is verlopen."

### Log in met GitHub, GitLab, Google, Dex...

Begrepen`oauth2.toml`en vul de aanbieders in die je gebruikt (elk vereist een registratie bij de aanbieder, waarvan de aanbieder is geregistreerd
callback is je API-server), en zet dan`OAUTH2_CONFIG_FILE=oauth2.toml`, en`OAUTH2_CREATE_USER=1`om de
gebruiker bij de eerste login. Zie de[Implementatiegids](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Updates van de client

De clients kunnen op je API-server naar nieuwe versies zoeken in plaats van op GitHub: het beantwoordt`POST /api/software/version/latest`(de controle van huidige RustDesk-klanten) en`GET /api/software/releases/latest`. 
De nieuwste versie en de downloadlinks worden voorgelezen van`s3config.toml`(S3-compatibele opslag), zie[de REST API](docs/api.md#software-update-endpoints).

## Klanten

*   **Officiële RustDesk-clients**Werk samen met deze server: geef ze je server en je publieke sleutel.
*   **De SCTGDesk-client**is upstream RustDesk plus onze overlay (naam, pictogrammen, standaardservers en sleutel). Jij bouwt het
    Voor je eigen servers, zie[Aangepaste builds](docs/custom-builds.md#the-client). De**Webclient**is opgebouwd uit
    dezelfde overlay.

## Bouw het zelf

Je hebt een Rust-toolchain nodig en, voor de webconsole, Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Set`SCTGDESK_SKIP_WEB_BUILD=1`om zonder Node.js te bouwen (de console is dan een tijdelijke aanduiding). De[Ontwikkelaarshandleiding](https://sctg-development.github.io/sctgdesk-server/)Behandelt het bouwen, testen, aangepaste server en
Clients bouwt, verpakt, releases en synchroniseert met upstream.

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

### API-documentatie

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### In de client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Beveiliging

*   De embedded API-server is niet beschermd tegen misbruik: plaats een TLS reverse proxy (HAProxy, NGINX, Caddy...) ervoor.
    van deze poorten, en van de WebSocket-poorten. Een voorbeeld van HAProxy is in de[Implementatiegids](docs/deployment.md#tls-reverse-proxy).
*   Verander het standaard adminwachtwoord, behoud`id_ed25519`geheim en back-up maken: het veranderen ervan zorgt ervoor dat elke klant wordt afgewezen
    Je server.
*   Rapporteer kwetsbaarheden zoals beschreven in[SECURITY.md](SECURITY.md).

## Licentie en broncode

SCTGDesk wordt verspreid onder de[GNU Affero Algemene Publieke Licentie v3](LICENSE). De broncode van elke release
in deze repository staat, op de tag van die release. Als je deze software aanpast en anderen het laat gebruiken via een
Network, Sectie 13 van de licentie vereist dat je hen de bijbehorende broncode van je aangepaste versie aanbiedt.

Controleer de SHA-256-sommen die bij elke release zijn gepubliceerd voordat je een gedownloade binaire map uitvoert.

## Bijdragen

[Ontwikkelaarshandleiding](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md). Voor algemene vragen over zelfhosting, de[RustDesk-documentatie](https://rustdesk.com/docs/en/self-host/)Ook van toepassing.

Als je dit werk leuk vindt, overweeg dan om het een ster te geven, of[Sponsoring ervan](https://github.com/sponsors/sctg-development).
