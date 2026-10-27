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

# SctgDesk Server Programma

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**Binaire download**](https://github.com/sctg-development/sctgdesk-server/releases)

[**API-documentatie**](https://sctg-development.github.io/sctgdesk-api-server/)

Dit is een aangepaste versie van RustDesk Server, die gratis en open source is.

*   Het eerste verschil is dat deze versie de nieuwe versie bevat*TCP*modus inbegrepen in de RustDesk Server Pro-versie.
*   Het tweede verschil is dat deze versie een voorlopige implementatie van de Rustdesk Server Pro API-server bevat.
    *   Ondersteuning voor persoonlijk adresboek
    *   Ondersteuning voor gedeeld adresboek op groepsniveau
        *   Alleen-lezen, lezen-schrijven, admin
    *   Ondersteuning voor gedeeld adresboek op gebruikersniveau
        *   Alleen-lezen, lezen-schrijven, admin
*   Het derde verschil is dat deze versie een voorlopige implementatie van een eenvoudige webconsole bevat.

De webconsole is toegankelijk op het adres`http://<server-ip>:21114/`met login "admin" en wachtwoord "Hallo, wereld!" .\
Je kunt de API-documentatie bekijken in de ingebouwde API-server op het adres`http://<server-ip>:21114/api/doc/`.

Een niet-interactieve API-documentatie is beschikbaar op[sctgdesk-api-server repo](https://sctg-development.github.io/sctgdesk-api-server/).

## Start het project

**Als je mijn werk waardeert, overweeg het dan alsjeblieft een ster te geven! 🤩 of een** [![](https://img.shields.io/static/v1?label=Sponsor\&message=%E2%9D%A4\&logo=GitHub\&color=%23fe8e86)](https://github.com/sponsors/sctg-development)

## TL; DR

Je kunt het volgende gebruiken`docker-compose.yml`bestand om de server te starten:

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

en start de server met:

```bash
mkdir -p data
docker-compose up 
```

## Dubbelsterren

Binaries zijn beschikbaar voor de volgende platforms:

*   Linux x86\_64 statisch gekoppeld
*   Linux arm64 statisch gekoppeld
*   Linux armv7 statisch gekoppeld
*   MacOS Intel
*   MacOS Apple Silicon
*   Windows x86\_64

### Standaard beheerder

De standaard admin-gebruiker wordt aangemaakt met de gebruikersnaam`admin`en het wachtwoord`Hello,world!`. Je kunt het wachtwoord wijzigen na de eerste inlogmethode op de webconsole.

## API Standalone versie

De API standalone versie is een versie van de server die de API-server en de webconsole bevat, maar niet de rendez-vous server.\
De standalone versie is beschikbaar in een eigen repository[sctgdesk-api-server](https://github.com/sctg-development/sctgdesk-api-server).\
Voor alle API- of webconsole-gerelateerde problemen, raadpleeg de[sctgdesk-api-server](https://github.com/sctg-development/sctgdesk-api-server)Repository.

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

### Gebruik in de Rustdesk-client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Autoupdate-links genereren

We hebben onze client aangepast om de autoupdate-links van de API-server te halen in plaats van Github-releases.\
Om de autoupdate-links te laten werken, moet je je client aanpassen om de autoupdate-links van de API-server op te halen. Dit[Hoe je het kunt doen](https://github.com/sctg-development/sctgdesk/blob/481d3516fef1daa145d8044594187cb11959f8be/src/common.rs#L953L972):

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

# Beveiliging

De embedded API-server is niet beveiligd en ook niet beschermd tegen DDOS-aanvallen. Een goede praktijk is om een reverse proxy voor de API-server te gebruiken. NGINX is hiervoor een goede keuze. HAProxy is ook een goede keuze.\
We gebruiken HAProxy voor de API-server in onze productieomgeving. 
Dit is ons configuratiebestand voor HAProxy, het wordt alleen als voorbeeld verstrekt. Je zou het moeten aanpassen aan je eigen behoeften:

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

De hbbs-server wordt gestart met

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

# Beperk ongewenste toegang

Om de toegang tot je server te beperken, kun je de`--logged-in-only`optie of stel de`LOGGED_IN_ONLY=Y`Omgevingsvariabele voor de`hbbs`Server. Dit beperkt de controle tot alleen ingelogde gebruikers.

Zelfs met deze optie ingeschakeld kunnen gebruikers zich nog steeds registreren op de Rendez-vous-server, maar ze kunnen de peer van een andere gebruiker niet besturen. Als iemand probeert een peer te controleren zonder ingelogd te zijn, krijgt hij een foutmelding:

<img width="524" alt="Error message for unauthenticated control attempt" src="https://github.com/user-attachments/assets/cfa46504-39d8-46a7-9072-3ece6818b4a3">

Door deze functie in te schakelen, kun je een extra beveiligingslaag aan je server toevoegen en ongeautoriseerde toegang voorkomen.

**Configureren`LOGGED_IN_ONLY`**

Om deze functie in te schakelen:

1.  Stel de`LOGGED_IN_ONLY=Y`Omgevingsvariabele voor de`hbbs`Server.
2.  Alternatief kun je de`--logged-in-only`optie bij het uitvoeren van de`hbbs`Server.

**Voorbeeld**

Om de`LOGGED_IN_ONLY`omgevingsvariabele kunt u de volgende regel toevoegen aan uw`~/.bashrc`Bestand of gelijkwaardig:

```bash
export LOGGED_IN_ONLY=Y
```

# RustDesk Server Program

[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/macos-intel-build.yml)
[![build](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/windows.yml)

[**Download**](https://github.com/sctgdesk/sctgdesk-server/releases)

[**Handboek**](https://rustdesk.com/docs/en/self-host/)

[**FAQ**](https://github.com/rustdesk/rustdesk/wiki/FAQ)

[**Hoe OSS naar Pro te migreren**](https://rustdesk.com/docs/en/self-host/rustdesk-server-pro/installscript/#convert-from-open-source)

Host zelf je eigen RustDesk-server, die is gratis en open source.

## Hoe handmatig te bouwen

Eerst moet je een werkende Rust-ontwikkeltoolchain hebben en een werkende Node ≥ 20 werkende installatie.

*   Unices (Linux, MacOS, enz.):

```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --release
```

*   Ramen met cmd.exe shell:

```cmd
set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release
```

Drie uitvoerbare bestanden worden gegenereerd in target/release.

*   hbbs - RustDesk ID/Rendezvous-server met API-server
*   hbbr - RustDesk relaisserver
*   rustdesk-utils - RustDesk CLI-hulpprogramma's

Je kunt bijgewerkte binairen vinden op de[Uitgaven](https://github.com/sctg-development/sctgdesk-server/releases)pagina.

Alle vrijgegeven binaries na release v1.1.99-40 worden geattesteerd met Github Actions. Je kunt de attestatie controleren door de sha256sum van de binary te controleren met`https://search.sigstore.dev/?hash=<sha256>`bijvoorbeeld.

Als je extra functies wilt,[RustDesk Server Pro](https://rustdesk.com/pricing.html)Misschien past het beter bij je.

Als je je eigen server wilt ontwikkelen,[rustdesk-server-demo](https://github.com/rustdesk/rustdesk-server-demo)Misschien is het een betere en eenvoudigere start voor jou dan deze repo.

## Installatie

Docker-images worden automatisch gegenereerd en gepubliceerd op elke github-release.

Deze beelden zijn opgebouwd tegen`ubuntu-22.04`met de enige toevoeging van de hoofdbinaire (`hbbr`en`hbbs`). Ze zijn beschikbaar op[Docker-hub](https://hub.docker.com/r/sctg/sctgdesk-server/)met deze tags:

| Architectuur | afbeelding:tag |
| --- | --- |
| amd64 |`sctg/sctgdesk-server:latest`|
| arm64v8 |`sctg/sctgdesk-server:latest`|
| arm32v7 |`sctg/sctgdesk-server:latest`|

Je kunt deze afbeeldingen direct starten met`docker run`met deze commando's:

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr 
```

of zonder`--net=host`, maar P2P-directe verbinding kan niet werken.

Voor systemen die SELinux gebruiken, vervanging`/root`door`/root:z`vereist is om de containers correct te laten draaien. Alternatief kan de separatie van SELinux-containers volledig worden uitgeschakeld door de optie toe te voegen`--security-opt label=disable`.

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]> 
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-serverlatest hbbr 
```

De`relay-server-ip`parameter is het IP-adres (of DNS-naam) van de server die deze containers draait. De**optioneel** `port`De parameter moet worden gebruikt als je een andere poort gebruikt dan**21117**voor`hbbr`.

Je kunt docker-compose ook gebruiken, waarbij je deze configuratie als sjabloon gebruikt:

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

Bewerk regel 16 zodat deze naar je relaisserver wijst (degene die luistert op poort 21117). Je kunt ook de volumelijnen (lijn 18 en lijn 33) bewerken als je dat nodig hebt.

(docker-compose krediet gaat naar @lukebarone en @QuiGonLeong)

> \[! OPMERKING]\
> De rustdesk/rustdesk-server:latest in China kan worden vervangen door het nieuwste versienummer op Docker Hub, zoals`rustdesk-server:1.1.10-3`. Anders kan de oude versie worden verwijderd door beeldversnelling.

> Let op dat hier de sctg/sctgdesk-server-server:latest in China kan worden vervangen door het nieuwste versienummer op dockerhub, zoals sctg/sctgdesk-server-server:1.1.99-37. Anders kan de oude versie worden verwijderd vanwege beeldversnelling.

## Hoe maak je een sleutelbord

Een sleutelpaar is nodig voor encryptie; je kunt het aanbieden, zoals eerder uitgelegd, maar je hebt een manier nodig om er een te maken.

Je kunt dit commando gebruiken om een sleutelhanger te genereren:

```bash
/usr/bin/rustdesk-utils genkeypair
```

Als je de`rustdesk-utils`pakket op je systeem geïnstalleerd, kun je hetzelfde commando aanroepen met docker:

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server-server:latest genkeypair
```

De output zal ongeveer als volgt zijn:

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## Pakketten

Er zijn aparte .deb pakketten beschikbaar voor elke binaire map, die je kunt vinden in de[Uitgaven](https://github.com/sctg-development/sctgdesk-server/releases). 
Deze pakketten zijn bedoeld voor de volgende distributies:

*   Ubuntu 22.04 LTS
*   MacOS Intel of Apple Silicon
*   Windows x86\_64 of i686

## ENV-variabelen

`hbbs`en`hbbr`kan worden geconfigureerd met deze ENV-variabelen. 
Je kunt de variabelen zoals gewoonlijk specificeren of een`.env`Bestand.

| variabele | binair | beschrijving |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | als ingesteld op**"J"**Staat directe peerverbinding niet toe |
| DOWNGRADE_START_CHECK | hbbr | vertraging (in seconden) voor downgrade check |
| DOWNGRADE_THRESHOLD | hbbr | drempel van downgrade check (bit/ms) |
| SLEUTEL | hbbs/hbbr | als ingesteld is het gebruik van een specifieke sleutel, als ingesteld op**"\_"**Dwing het gebruik van elke sleutel |
| LIMIT_SPEED | hbbr | snelheidslimiet (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | pad voor oauth2 configuratiebestand |
| OAUTH2\_CREATE_USER | hbbs | als ingesteld op**"1"**Maak een gebruiker aan als die niet bestaat |
| POORT | HBBS/HBBR | Luisterpoort (21116 voor HBBS - 21117 voor HBBR) |
| RELAY | hbbs | IP-adres/DNS-naam van de machines die hbbr draaien (gescheiden door komma) |
| RUST_LOG | alle | set debug level (fout|waarschuw|info|debug|trace) |
| S3CONFIG_FILE | hbbs | pad voor s3 configuratiebestand |
| SINGLE_BANDWIDTH | hbbr | maximale bandbreedte voor één enkele verbinding (in Mb/s) |
| TOTAL_BANDWIDTH | HBBB | maximale totale bandbreedte (in Mb/s) |
