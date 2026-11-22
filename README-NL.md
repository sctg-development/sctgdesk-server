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

SCTGDesk is een zelfgehoste remote-desktopserver, met eigen clients, afgeleid van[RustDesk](https://github.com/rustdesk/rustdesk). Het is vrije software onder de**AGPL-3.0**.

> **SCTGDesk is een onafhankelijk project. Het is niet gelieerd aan, niet gesponsord door of gesponsord door het RustDesk-project
> of Purslane Ltd.**"RustDesk" is de naam van dat project; het wordt hier alleen gebruikt om aan te geven wat deze software is
> afgeleid van en compatibel met.

Deze repository is het hele project: de server, de API-server, de webconsole en de client-overlay. Het vervangt
De voormalige repositories`sctgdesk-api-server`en`sctgdesk`, die zijn gearchiveerd.

## Wat je krijgt

*   De RustDesk-server:`hbbs`(ID en rendez-vousserver) en`hbbr`(relay), plus`rustdesk-utils`(sleutelparen, 
    connectiviteitscontrole). Compatibel met de officiële RustDesk-clients.
*   De serverzijde van de**versleutelde TCP- en WebSocket-verbinding**(sleuteluitwisselingsversies 0 en 1), waarvan de
    open-source upstream server wordt niet geïmplementeerd. Dit is wat de**webclient**Werken.
*   A**REST API**ingebed in`hbbs`, compatibel met de clients: login (wachtwoord, OpenID Connect / OAuth2: 
    GitHub, Dex of een standaard OAuth2-provider), sessies die een herstart overleven, persoonlijke en gedeelde adresboeken (per gebruiker en
    per groep, alleen-lezen / lezen-schrijven / beheerder), gebruikers en groepen, software-update-eindpunten.
*   A**webconsole**om dat allemaal te beheren, en interactieve API-documentatie.
*   A**Verbindingsgeschiedenis**met een aantekening bij elke verbinding (wat er is gedaan), doorzoekbaar en exporteerbaar naar CSV, ingevuld door de RustDesk-clients.
*   Een optie om niet-geauthenticeerde controle te weigeren (`--logged-in-only`).
*   A**cliënt**(`client/`): upstream RustDesk plus een kleine overlay, bij bouwtijd voorzien van jouw naam, servers en
    key; en een**webclient**.

## Eén server, elk platform

Dezelfde server draait op een datacenter, een Windows-server, een Mac, een Raspberry Pi of een router, vanaf dezelfde
Release, met de formaten die elk platform verwacht:

| Platform | Architecturen | Hoe je het krijgt |
|---|---|---|
|**Linux**| amd64, arm64, armv7 (armhf) |`deb`en`rpm`pakketten,**ondertekende apt- en dnf-repositories**, statische binairs, Docker-afbeelding (amd64, arm64) |
|**OpenWrt**25,12+ | x86\_64, aarch64, armv7,**MIPS big endian**(ath79) en**MIPS kleine endian**(ramips) |**ondertekend`apk`repository**, met een`procd`service en UCI-configuratie |
|**Windows**| amd64, arm64 | zip,**Winget**, en**native Windows-services**(`sc.exe`) |
|**macOS**| Intel, Apple Silicon | zip |

Elke Linux- en OpenWrt-binary is**statisch**(musl): geen bibliotheek om te installeren, hetzelfde bestand op welke distributie dan ook.

Compatibiliteit komt voorop: de server moet blijven werken met officiële RustDesk-clients en onze wijzigingen voor upstream
bestanden worden klein gehouden zodat nieuwe upstream-releases kunnen worden samengevoegd.

## Installeren

Kies er één. Wat je ook kiest, je hebt een**Sleutelpaar**en de poorten daaronder.

### Linux-pakketten (deb, rpm) en pakketrepositories

Elke release heeft`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`en`sctgdesk-server-utils`voor**amd64**,**arm64**en**armv7**(32-bit ARM,`armhf`), als`.deb`en`.rpm`, opgebouwd uit statische musl-binaries (geen bibliotheekafhankelijkheid). Zij
vervang de eerste`rustdesk-server-*`pakketten.

**Uit de ondertekende repositories**(updates met het systeem):

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

**Of uit de bestanden van een release:**

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

Een eerste installatie op een lege datamap start beide services met een willekeurig toetspaar; opties gaan in`/etc/default/sctgdesk-hbbs`en`-hbbr`(zie[De Implementatiegids](docs/deployment.md), wat ook de
Migratie uit het eerste`rustdesk-server-*`Pakketten:**verwijder ze niet**). Een upgrade start de diensten opnieuw op
die draaien. Data is binnen`/var/lib/rustdesk-server`, logt in`/var/log/rustdesk-server`; De eenheden draaien als root.

De repositories en de ondertekeningssleutels worden beschreven in[Pakketrepositories](docs/repositories.md).

### OpenWrt

Voor OpenWrt 25.12 en later (`apk`), op x86\_64, ARM (aarch64, armv7) en MIPS (`mips_24kc`voor ath79,`mipsel_24kc`voor
ramips) routers:

```sh
wget -O /etc/apk/keys/sctgdesk.rsa.pub https://sctg-development.github.io/sctgdesk-server/apk/sctgdesk.rsa.pub
. /etc/openwrt_release        # DISTRIB_ARCH: the architecture of the packages of your router, for example mips_24kc
echo "https://sctg-development.github.io/sctgdesk-server/apk/$DISTRIB_ARCH/APKINDEX.tar.gz" > /etc/apk/repositories.d/sctgdesk.list
apk update && apk add sctgdesk-server-hbbs sctgdesk-server-hbbr
```

Elk programma is een`procd`service geconfigureerd met UCI (`/etc/config/sctgdesk-hbbs`).`hbbs`is ongeveer 30 MB: zet het neer, en
zijn data, op een USB-schijf of een extroot. De`mips_24kc`de build werd getest op twee echte ath79-routers (QCA9558 en QCA956X); de andere architecturen werden uitsluitend onder QEMU of op andere hardware uitgevoerd. Oudere OpenWrt-releases en andere
architecturen kunnen de statische binaries van een release gebruiken. Details:[Pakketrepositories](docs/repositories.md).

### Binaries (Linux, Windows, macOS)

De[Releases](https://github.com/sctg-development/sctgdesk-server/releases)pagina heeft ook zip-archieven met`hbbs`,`hbbr`en`rustdesk-utils`voor Windows (amd64, arm64) en macOS (Intel, Apple Silicon), met`SHA256SUMS-*`bestanden. 
Pak ze uit en voer ze vervolgens uit vanuit de map waar de data zich zouden moeten bevinden (zie[Eerste run](#first-run)).

Aan**Windows**ze kunnen worden geïnstalleerd met`winget install SCTG.SCTGDeskServer --scope machine`en als diensten worden uitgevoerd
(`hbbs.exe --windows-service`, aangemaakt met`sc.exe`): zie[Op Windows](docs/windows.md).

### Docker

Afbeeldingen worden gepubliceerd op[Docker-hub](https://hub.docker.com/r/sctg/sctgdesk-server/)als`sctg/sctgdesk-server`. 
Opslaan als`docker-compose.yml`, ter vervanging`sctgdesk.example.com`met het adres van uw server:

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

Voor SELinux, voeg toe`:z`naar het volume, of gebruik`--security-opt label=disable`. Zonder`--net=host`Directe peer-to-peer
verbindingen werken niet (alles gaat via het relais). Een helmkaart staat in`deploy/helm`(te herstructureren) en
een voorbeeld manifest in`kubernetes/`.

### Ports

| Port | Protocol | Gebruikt door | Doel |
|---|---|---|---|
| 21114 | TCP |`hbbs`| REST API, webconsole, API-documentatie (`--api-port`) |
| 21115 | TCP |`hbbs`| NAT-typetest |
| 21116 | TCP +**UDP**|`hbbs`| ID / rendezvous |
| 21117 | TCP |`hbbr`| relay |
| 21118 | TCP |`hbbs`| WebSocket (webclient) |
| 21119 | TCP |`hbbr`| WebSocket (webclient) |

Open ze op uw firewall. De API- en WebSocket-poorten moeten zich achter een TLS-reverse proxy op het internet bevinden: zie
de[Implementatiegids](docs/deployment.md).

## Eerste run

`hbbs`en`hbbr`bewaar hun gegevens (sleutelpaar, SQLite-database) in de**Huidige directory**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

Bij de eerste start`hbbs`aanmaakt`id_ed25519`(geheim) en`id_ed25519.pub`(de**publieke sleutel**, die je aan geeft
je klanten). Om zelf een sleutelpaar te maken, of om er één te controleren:

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

Om clients die de sleutel niet hebben te weigeren, start je beide servers met`-k <key>`(of`KEY=<key>`). Controleer of een
server antwoordt op al zijn poorten met`rustdesk-utils doctor <server>`.

**Standaard admin.**De webconsole staat op`http://<server>:21114/`en de API-documentatie op`http://<server>:21114/api/doc/`. Een nieuwe database heeft de gebruiker`admin`met het wachtwoord`Hello,world!`, wat is
Publiek:**wijzig het bij de eerste aanmelding**.

## Configuratie

`hbbs`en`hbbr`Opties kiezen op de opdrachtregel (`hbbs --help`) en vanuit omgevingsvariabelen, of een`.env`Bestand.

| variabele | binair | beschrijving |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | indien ingesteld op**"Y"**Directe peerverbinding wordt niet toegestaan |
| DB_URL | hbbs | pad van de SQLite-database (standaard`./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | HBBS | indien ingesteld op**"Y"**alleen ingelogde gebruikers kunnen een peer bedienen (hetzelfde als`--logged-in-only`) |
| ALLOW_REMOTE_BACKUP | hbbs | indien ingesteld op**"Y"**, een beheerder kan een back-up downloaden (database, sleutelpaar, configuratie) met`GET /api/backup`of vanaf de console (hetzelfde als`--allow-remote-backup`); standaard uit, zie[De Implementatiegids](docs/deployment.md#download-a-backup)|
| REQUIRE_KX_V1 | HBBS | indien ingesteld op**"Y"**weigert clients die de legacy (versie 0) TCP-sleuteluitwisseling gebruiken (hetzelfde als`--require-kx-v1`) |
| DOWNGRADE_START_CHECK | HBR | vertraging (in seconden) vóór downgrade-controle |
| DOWNGRADE_THRESHOLD | HBBR | Drempel voor downgrade check (bit/ms) |
| KEY | hbbs/hbbr | als ingesteld dwing je het gebruik van een specifieke sleutel af, als ingesteld op**"\_"**het gebruik van willekeurige toets afdwingen |
| LIMIT_SPEED | HBBR | snelheidslimiet (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | pad van het OAuth2 / OpenID Connect-providerbestand (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | HBBS | indien ingesteld op**"1"**Een gebruiker aanmaken bij de eerste login met een provider |
| PORT | HBBS/HBBR | Luisterpoort (21116 voor HBBS - 21117 voor HBBR) |
| HEALTH_PORT | hbbr | indien ingesteld, antwoordt HBBR`GET /livez`en`GET /readyz`(HTTP) op deze poort; standaard uit |
| RELAY | HBBS | IP-adres/DNS-naam van de machines waarop HBR draait (gescheiden door komma) |
| LOG_FORMAT | HBBS, HBB |**"JSON"**: één JSON-object per loglijn (`time`,`level`,`target`,`file`,`line`,`msg`); Standaard: tekst |
| RUST_LOG | alles | debugniveau instellen (fout|waarschuwing|info|debug|trace) |
| ROCKET_SECRET_KEY | hbbs | sleutel die de cookies van de API-server ondertekent (base64, 64 bytes). Standaard: gegenereerd bij de eerste start in`secret_key.txt`naast de database |
| S3CONFIG_FILE | hbbs | pad van het software-updatebestand (`s3config.toml`) |
| SINGLE_BANDWIDTH | HBBR | maximale bandbreedte voor één enkele verbinding (in Mb/s) |
| TOTAL_BANDWIDTH | HBBR | maximale totale bandbreedte (in Mb/s) |

### Alleen ingelogde gebruikers kunnen het bedienen

`--logged-in-only`(of`LOGGED_IN_ONLY=Y`) maakt`hbbs`vraag de API-server of het token van de client een live is
sessie voordat het twee peers verbindt. Iedereen kan zijn apparaat nog steeds registreren, maar niemand kan een apparaat bedienen
zonder in te loggen. Een client die niet is ingelogd ziet "De verbinding is niet toegestaan. U bent niet ingelogd."; één
wiens sessie eindigde toont "Uw sessie is verlopen."

### Log in met GitHub, Dex of een andere OAuth2-provider

Kopieer`oauth2.toml`en vul de aanbieders in die u gebruikt (elk heeft een aanvraag nodig die geregistreerd is bij de aanbieder, wiens aanbieder
callback is uw API-server). De aanbiedertypes die kunnen inloggen zijn`Github`,`Dex`en het generieke`Oauth2`(any
standaard OAuth2 / OpenID Connect-provider: u geeft de adressen op). De webconsole controleert een bestand tegen het echte
provider voordat je het opslaat. Stel dan in`OAUTH2_CONFIG_FILE=oauth2.toml`, en`OAUTH2_CREATE_USER=1`om de
gebruiker bij de eerste inlog. Zie de[Implementatiegids](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Updates van de client

De clients kunnen op uw API-server naar nieuwe versies zoeken in plaats van op GitHub: dat antwoordt`POST /api/software/version/latest`(de controle van huidige RustDesk-clients) en`GET /api/software/releases/latest`. 
De nieuwste versie en de downloadlinks worden voorgelezen vanaf`s3config.toml`(S3-compatibele opslag), zie[de REST API](docs/api.md#software-update-endpoints).

## Klanten

*   **Officiële RustDesk-clients**Werk samen met deze server: geef ze je server en je publieke sleutel.
*   **De SCTGDesk-client**is upstream RustDesk plus onze overlay (naam, iconen, standaardservers en sleutel). Jij bouwt het
    Voor uw eigen servers, zie[aangepaste builds](docs/custom-builds.md#the-client). De**webclient**is gebouwd uit
    Dezelfde overlay.

## Bouw het zelf

Je hebt een Rust-toolchain nodig en, voor de webconsole, Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Set`SCTGDESK_SKIP_WEB_BUILD=1`om te bouwen zonder Node.js (de console is dan een tijdelijke aanduiding). De[Ontwikkelaarshandleiding](https://sctg-development.github.io/sctgdesk-server/)Omvat het bouwen, testen, aangepaste servers en
Client builds, verpakkingen, releases en synchronisatie met upstream.

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

*   De ingebedde API-server is niet beschermd tegen misbruik: zet een TLS-reverse proxy (HAProxy, NGINX, Caddy...) vooraan
    ervan, en van de WebSocket-poorten. Een voorbeeld van HAProxy is te vinden in de[Implementatiegids](docs/deployment.md#tls-reverse-proxy).
*   Wijzig het standaard adminwachtwoord, behoud`id_ed25519`Geheim en back-up maken: het wijzigen ervan zorgt ervoor dat elke klant het afwijst
    jouw server.
*   Rapporteer kwetsbaarheden zoals beschreven in[SECURITY.md](SECURITY.md).

## Licentie en broncode

SCTGDesk wordt gedistribueerd onder de[GNU Affero General Public License v3](LICENSE). De broncode van elke release
bevindt zich in deze repository, op de tag van die release. Als je deze software wijzigt en anderen toestaat deze te gebruiken over een
Network, sectie 13 van de licentie vereist dat u hen de bijbehorende broncode van uw gewijzigde versie aanbiedt.

Controleer de SHA-256-bedragen die bij elke release zijn gepubliceerd voordat je een gedownloade binaire bestand uitvoert.

## Bijdragen

[Ontwikkelaarshandleiding](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md). Voor algemene vragen over zelfhosting is de[RustDesk-documentatie](https://rustdesk.com/docs/en/self-host/)Ook van toepassing.

Als je dit werk leuk vindt, overweeg dan om het een ster te geven, of[Sponsoring ervan](https://github.com/sponsors/sctg-development).
