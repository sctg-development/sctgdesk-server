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

SCTGDesk est un serveur de bureau à distance auto-hébergé, avec ses propres clients, dérivés de[RustDesk](https://github.com/rustdesk/rustdesk). C’est un logiciel libre sous la**AGPL-3.0**.

> **SCTGDesk est un projet indépendant. Il n’est pas affilié, ni approuvé ni sponsorisé par le projet RustDesk
> ou Purslane Ltd.**« RustDesk » est le nom de ce projet ; il est utilisé ici uniquement pour indiquer ce qu’est ce logiciel
> dérivé de et compatible avec.

Ce dépôt est tout le projet : le serveur, le serveur API, la console web et la superposition client. Il remplace
Les anciens dépôts`sctgdesk-api-server`et`sctgdesk`, qui sont archivées.

## Ce que tu obtiens

*   Le serveur RustDesk :`hbbs`(Serveur d’identification et de rendez-vous) et`hbbr`(relais), en plus`rustdesk-utils`(paires de clés, 
    Vérification de connectivité). Compatible avec les clients officiels de RustDesk.
*   Le côté serveur du**TCP et connexion WebSocket chiffrées**(échanges de clés versions 0 et 1), que le
    le serveur amont open source ne s’implémente pas. C’est ce qui fait que le**Client web**travail.
*   A**REST API**Intégré dans`hbbs`, compatible avec les clients : connexion (mot de passe), OpenID Connect / OAuth2 : 
    GitHub, GitLab, Google, Dex...), des sessions qui survivent à un redémarrage, des carnets d’adresses personnels et partagés (par utilisateur et
    par groupe, lecture seule / lecture-écriture / admin), utilisateurs et groupes, terminaux de mise à jour logicielle.
*   A**Console web**pour gérer tout cela, et une documentation API interactive.
*   A**Historique des connexions**avec une note sur chaque connexion (ce qui a été fait), consultable et exportable en CSV, remplie par les clients RustDesk.
*   Une option pour refuser un contrôle non authentifié (`--logged-in-only`).
*   A**Client**(`client/`: en amont RustDesk plus une petite superposition, marquée au moment de la compilation avec votre nom, serveurs et
    clé ; et un**Client web**.

La compatibilité passe avant tout : le serveur doit continuer à fonctionner avec les clients officiels de RustDesk, et nos modifications en amont
Les fichiers sont maintenus petits afin que les nouvelles versions en amont puissent être fusionnées.

## Installation

Choisis-en un. Quoi que tu choisisses, il te faut un**Paire de touches**et les ports en dessous.

### Paquets Linux (deb, rpm)

Chaque sortie a`sctgdesk-server-hbbs`,`sctgdesk-server-hbbr`et`sctgdesk-server-utils`pour**AMD64**et**arm64**, comme`.deb`et`.rpm`, construit à partir de binaires musl statiques (pas de dépendance à la bibliothèque). Ils remplacent les premiers`rustdesk-server-*`Des colis.

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

Une première installation ne déclenche rien : modifier les unités (`systemctl edit --full rustdesk-hbbs`, voir[Le Guide de déploiement](docs/deployment.md)), alors`sudo systemctl enable --now rustdesk-hbbs rustdesk-hbbr`. Un
Mise à jour relance les services qui fonctionnaient. Les données sont entrées`/var/lib/rustdesk-server`, se connecte`/var/log/rustdesk-server`; Les unités fonctionnent en tant que racine.

### Binaires (Linux, Windows, macOS)

Le[Sorties](https://github.com/sctg-development/sctgdesk-server/releases)La page possède également des archives postales avec`hbbs`,`hbbr`et`rustdesk-utils`pour Windows (amd64, arm64) et macOS (Intel, Apple Silicon), avec`SHA256SUMS-*`dossiers. 
Décompressez-les, puis exécutez-les depuis le répertoire où les données devraient se trouver (voir[Première série](#first-run)).

### Docker

Les images sont publiées sur[Hub Docker](https://hub.docker.com/r/sctg/sctgdesk-server/)comme`sctg/sctgdesk-server`. 
Enregistrer comme`docker-compose.yml`, remplaçant`sctgdesk.example.com`Avec l’adresse de votre serveur :

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

Pour le SELinux, ajoutez`:z`au volume, ou à l’utilisation`--security-opt label=disable`. Sans`--net=host`Pair à pair direct
Les connexions ne peuvent pas fonctionner (tout passe par le relais). Un tableau de helm est en`deploy/helm`(à refactoriser) et
Un exemple de manifestation dans`kubernetes/`.

### Ports

| Port | Protocole | Utilisé par | Objectif |
|---|---|---|---|
| 21114 | TCP |`hbbs`| API REST, console web, documentation API (`--api-port`) |
| 21115 | TCP |`hbbs`| Test de type NAT |
| 21116 | TCP +**UDP**|`hbbs`| ID / rendez-vous |
| 21117 | TCP |`hbbr`| relais |
| 21118 | TCP |`hbbs`| WebSocket (client web) |
| 21119 | TCP |`hbbr`| WebSocket (client web) |

Ouvrez-les sur votre pare-feu. Les ports API et WebSocket devraient être derrière un proxy inverse TLS sur Internet : voir
le[Guide de déploiement](docs/deployment.md).

## Première série

`hbbs`et`hbbr`conserver leurs données (paire de clés, base de données SQLite) dans le**Annuaire actuel**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

Dès le premier départ`hbbs`crée`id_ed25519`(secret) et`id_ed25519.pub`(le**Clé publique**, que tu donnes à
vos clients). Pour former vous-même une paire de clés, ou en vérifier une :

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

Pour refuser les clients qui n’ont pas la clé, démarrez les deux serveurs avec`-k <key>`(ou`KEY=<key>`). Vérifie que
Server répond sur tous ses ports avec`rustdesk-utils doctor <server>`.

**Admin par défaut.**La console web est à`http://<server>:21114/`et la documentation API à`http://<server>:21114/api/doc/`. Une nouvelle base de données a l’utilisateur`admin`avec le mot de passe`Hello,world!`, c’est-à-dire
Public :**Change-le dès la première connexion**.

## Configuration

`hbbs`et`hbbr`Prendre les options en ligne de commande (`hbbs --help`) et à partir de variables d’environnement, ou un`.env`classier.

| variable | binaire | description |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | si on est réglé sur**« Y »**interdit la connexion directe entre les pairs |
| DB_URL | hbbs | chemin de la base de données SQLite (par défaut`./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | hbbs | si on est réglé à**« Y »**seuls les utilisateurs connectés peuvent contrôler un pair (comme`--logged-in-only`) |
| REQUIRE_KX_V1 | hbbs | si réglé sur**« Y »**refuse les clients qui utilisent l’ancien échange de clés TCP (version 0) (identique à`--require-kx-v1`) |
| DOWNGRADE_START_CHECK | hbbr | délai (en secondes) avant la rétrogradation |
| DOWNGRADE_THRESHOLD | hbbr | seuil de contrôle de rétrogradation (bit/ms) |
| CLÉ | hbbs/hbbr | si défini force l’utilisation d’une clé spécifique, si on est défini à**"\_"**forcer l’utilisation de toute clé |
| LIMIT_SPEED | hbbr | limitation de vitesse (en Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | chemin du fichier des fournisseurs OAuth2 / OpenID Connect (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | hbbs | si réglé sur**"1"**créer un utilisateur dès la première connexion avec un fournisseur |
| PORT | HBBS/HBBR | port d’écoute (21116 pour HBBS - 21117 pour HBBR) |
| RELAIS | HBBS | Adresse IP/nom DNS des machines exécutant hbbr (séparé par virgule) |
| RUST_LOG | tous | définir le niveau de débogage (erreur|avertissement|info|déboguer|trace) |
| S3CONFIG_FILE | hbbs | chemin du fichier de mise à jour logicielle (`s3config.toml`) |
| SINGLE_BANDWIDTH | hbbr | bande passante maximale pour une seule connexion (en Mb/s) |
| TOTAL_BANDWIDTH | hbbr | bande passante totale maximale (en Mb/s) |

### Seuls les utilisateurs connectés peuvent contrôler

`--logged-in-only`(ou`LOGGED_IN_ONLY=Y`) rend`hbbs`demandez au serveur API si le jeton du client est en ligne
Avant de connecter deux pairs. N’importe qui peut toujours enregistrer son appareil, mais personne ne peut contrôler un appareil
sans se connecter. Un client qui n’est pas connecté voit « La connexion n’est pas autorisée. Vous ne vous êtes pas connecté. » ; un
dont la session s’est terminée voit « Votre session a expiré. »

### Connectez-vous avec GitHub, GitLab, Google, Dex...

Bien reçu`oauth2.toml`et remplissez les fournisseurs que vous utilisez (chacun nécessite une demande enregistrée auprès du prestataire, qui
le callback est votre serveur API), puis régler`OAUTH2_CONFIG_FILE=oauth2.toml`, et`OAUTH2_CREATE_USER=1`pour créer le
utilisateur lors de la première connexion. Voir le[Guide de déploiement](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Mises à jour du client

Les clients peuvent rechercher de nouvelles versions sur votre serveur API au lieu de GitHub : il répond`POST /api/software/version/latest`(la vérification des clients actuels de RustDesk) et`GET /api/software/releases/latest`. 
La dernière version et les liens de téléchargement sont consultés depuis`s3config.toml`(stockage compatible S3), voir[l’API REST](docs/api.md#software-update-endpoints).

## Clients

*   **Clients officiels de RustDesk**Travaillez avec ce serveur : donnez-leur votre serveur et votre clé publique.
*   **Le client SCTGDesk**est en amont de RustDesk plus notre superposition (nom, icônes, serveurs par défaut et clé). Vous le construisez
    Pour vos propres serveurs, voir[Constructions personnalisées](docs/custom-builds.md#the-client). Le**Client web**est construit à partir de
    La même superposition.

## Construisez-le vous-même

Il faut une chaîne d’outils Rust et, pour la console web, Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Décor`SCTGDESK_SKIP_WEB_BUILD=1`pour construire sans Node.js (la console sert alors de placeholder). Le[Manuel du développeur](https://sctg-development.github.io/sctgdesk-server/)couvre la construction, les tests, les serveurs personnalisés et
Builds client, empaquetage, publie et synchronise avec l’amont.

## Captures d’écran

### Webconsole

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### Documentation APIe

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### Dans le client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Sécurité

*   Le serveur API embarqué n’est pas protégé contre les abus : placez un proxy inverse TLS (HAProxy, NGINX, Caddy...) en avant
    de celui-ci, et des ports WebSocket. Un exemple de HAProxy se trouve dans le[Guide de déploiement](docs/deployment.md#tls-reverse-proxy).
*   Changer le mot de passe administrateur par défaut, conserver`id_ed25519`Secret et sauvegarde : Changer fait rejeter chaque client
    Votre serveur.
*   Signaler les vulnérabilités telles que décrites dans[SECURITY.md](SECURITY.md).

## Licence et code source

SCTGDesk est distribué sous la[Licence publique générale GNU Affero v3](LICENSE). Le code source de chaque version
est dans ce dépôt, au tag de cette version. Si vous modifiez ce logiciel et laissez d’autres personnes l’utiliser plutôt qu’un
Network, la section 13 de la licence exige que vous leur fourniez le code source correspondant de votre version modifiée.

Vérifiez les sommes SHA-256 publiées à chaque version avant d’exécuter un binaire téléchargé.

## Contributions

[Manuel du développeur](https://sctg-development.github.io/sctgdesk-server/)·[CONTRIBUTING.md](CONTRIBUTING.md)·[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)·[SECURITY.md](SECURITY.md). Pour les questions générales sur l’auto-hébergement, le[Documentation RustDesk](https://rustdesk.com/docs/en/self-host/)S’applique aussi.

Si vous aimez ce travail, pensez à lui donner une étoile, ou[Le parrainage](https://github.com/sponsors/sctg-development).
