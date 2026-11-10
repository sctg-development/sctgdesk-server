#!/usr/bin/env python3
"""Brand a checkout of upstream RustDesk as SCTGDesk.

Every rule replaces a literal string in a set of files. A rule that matches nothing is an
error, unless its replacement is already there (the script can be run twice): a rule that
silently did nothing would ship a client that still talks to RustDesk's servers.

Settings come from the environment (the CI passes them as secrets):

  APP_NAME              application name, e.g. SCTGDesk
  ORG_NAME              organisation, used for the website and the bundle ids
  RENDEZVOUS_SERVER     host of hbbs, replaces the built-in default
  RS_PUB_KEY            public key of hbbs (the content of id_ed25519.pub)
  API_SERVER            host[:port] of the API server
  API_SERVER_PROTOCOL   http or https (default https)
  COPYRIGHT_HOLDER      optional, added to upstream's in the copyright (default SCTG Development)
  PROJECT_URL           optional, where "set up your own server" leads (default the GitHub repository)
  MACOS_CODESIGN_IDENTITY  optional, "Name (TEAMID)": its TEAMID replaces the upstream team id

  branding.py [--root DIR] [--check] [--disable-api-server-forced] [--local-build]
  branding.py [--root DIR] [--check] --web

--check reports what would change and writes nothing.
--web brands the web client tree (client/apply.sh DIR web): the rules of the desktop and mobile
clients do not apply to it.
"""
import argparse
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path

SKIP_DIRS = {'.git', 'target', 'build', 'node_modules', '.dart_tool', 'Pods', 'vcpkg_installed'}
# The values that upstream ships and that identify it.
UPSTREAM_RENDEZVOUS = 'rs-ny.rustdesk.com'
UPSTREAM_KEY = 'OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw='
UPSTREAM_TEAM_ID = 'HZF9JMC8YN'


@dataclass
class Rule:
    files: list          # glob patterns relative to the root, `**` allowed
    old: str
    new: str
    optional: bool = False   # may legitimately match nothing


def env(name, default=None):
    value = os.environ.get(name, default)
    if value is None or value == '':
        sys.exit(f'branding: environment variable {name} is required')
    return value


def expand(root, patterns):
    seen = []
    for pattern in patterns:
        for path in sorted(root.glob(pattern)):
            if path.is_file() and not SKIP_DIRS.intersection(path.relative_to(root).parts):
                seen.append(path)
    return seen


def read(path):
    return path.read_text(encoding='utf-8', errors='surrogateescape')


def write(path, text):
    path.write_text(text, encoding='utf-8', errors='surrogateescape')


def apply_rule(root, rule, check):
    """Returns (files changed, already applied)."""
    changed, applied = 0, False
    for path in expand(root, rule.files):
        text = read(path)
        if rule.old in text:
            changed += 1
            if not check:
                write(path, text.replace(rule.old, rule.new))
        elif rule.new != rule.old and rule.new in text:
            applied = True
    return changed, applied


# The native client does the OAuth2 login in Rust (src/hbbs_http/account.rs); the Dart bridge of the web client of 1.2.5
# throws on it, and on the id, uuid and device info that every login sends. The same steps in JavaScript, with the
# same results for the Dart code of the login dialog (state_msg, failed_msg, url, auth_body).
WEB_LOGIN_JS = """// ===== login of the browser (client/branding.py) =====
const OIDC_QUERY_TIMEOUT_MS = 3 * 60 * 1000;
let oidcSession = { state_msg: '', failed_msg: '', url: null, auth_body: null };

function getWebUuid() {
  let u = localStorage.getItem('web-uuid');
  if (!u) {
    u = crypto.randomUUID ? crypto.randomUUID() : String(Date.now()) + String(Math.random());
    localStorage.setItem('web-uuid', u);
  }
  return btoa(u);
}

function getWebId() {
  return 'web-' + getWebUuid().replace(/[^A-Za-z0-9]/g, '').slice(0, 8);
}

function getLoginDeviceInfo() {
  return JSON.stringify({ os: 'Web', type: 'browser', name: navigator.userAgent });
}

function oidcAuthResult() {
  return JSON.stringify({
    state_msg: oidcSession.state_msg,
    failed_msg: oidcSession.failed_msg,
    url: oidcSession.url,
    auth_body: oidcSession.auth_body
  });
}

function oidcAuthCancel() {
  if (oidcSession.win) oidcSession.win.close();
  oidcSession = { state_msg: '', failed_msg: '', url: null, auth_body: null };
}

async function oidcAuth(value) {
  const { op, remember_me } = JSON.parse(value);
  const s = { state_msg: 'Requesting account auth', failed_msg: '', url: null, auth_body: null, win: null };
  oidcSession = s;
  // Opened here, while the click is still being handled: a window opened later is blocked
  s.win = window.open('', '_blank');
  const stop = (msg) => {
    s.failed_msg = msg;
    if (s.win) s.win.close();
  };
  const api = getApiServer();
  const id = getWebId();
  const uuid = getWebUuid();
  try {
    const r = await (await fetch(api + '/api/oidc/auth', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ op, id, uuid, deviceInfo: JSON.parse(getLoginDeviceInfo()) })
    })).json();
    if (oidcSession !== s) return;
    if (typeof r.error === 'string') return stop(r.error);
    if (!r.code || !r.url) return stop('Invalid auth response');
    if (s.win) {
      s.win.location.href = r.url;
    } else {
      s.url = r.url; // the window was blocked: the Dart code opens the URL
    }
    s.state_msg = 'Waiting account auth';
    const begin = Date.now();
    while (oidcSession === s && Date.now() - begin < OIDC_QUERY_TIMEOUT_MS) {
      try {
        const q = await (await fetch(api + '/api/oidc/auth-query?' + new URLSearchParams({ code: r.code, id, uuid }))).json();
        if (oidcSession !== s) return;
        if (!q) {
          // null: nothing authorised yet, like the native client the format error is ignored
        } else if (typeof q.error === 'string') {
          if (!q.error.includes('No authed oidc is found')) return stop(q.error);
        } else if (q.access_token) {
          if (remember_me && q.type === 'access_token') {
            localStorage.setItem('option:local:access_token', q.access_token);
            localStorage.setItem('option:local:user_info', JSON.stringify({ name: q.user && q.user.name, status: q.user && q.user.status }));
          }
          if (s.win) s.win.close();
          s.state_msg = 'Login account auth';
          s.auth_body = q;
          return;
        }
      } catch (e) {
        console.log('oidc query failed', e);
      }
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
    if (oidcSession === s) stop('timeout');
  } catch (e) {
    if (oidcSession === s) stop(String(e));
  }
}
// ===== end of the login of the browser =====

"""


def web_rules_for(cfg):
    """The web client of upstream (flutter/web/js, see apply.sh) and our landing page (flutter/web/client.html)."""
    app = cfg['APP_NAME']
    api = f"{cfg['API_SERVER_PROTOCOL']}://{cfg['API_SERVER']}"
    ts = 'flutter/web/js/src/connection.ts'
    return [
        # Where the browser connects: hbbs and hbbr behind a TLS WebSocket proxy (ports 21120 and 21121)
        Rule([ts], 'const PORT = 21116', 'const PORT = 21118'),
        Rule([ts], 'uri = tmp[0] + ":" + (port + (isRelay ? roffset || 3 : 2));',
             'uri = tmp[0] + ":" + (PORT + (isRelay ? 3 : 2));'),
        Rule([ts], 'const SCHEMA = "ws://";', 'const SCHEMA = "wss://";'),
        # Upstream's servers and key must not stay the defaults
        Rule([ts], '  "rs-sg.rustdesk.com",\n  "rs-cn.rustdesk.com",\n  "rs-us.rustdesk.com",\n',
             f'  "{cfg["RENDEZVOUS_SERVER"]}",\n'),
        Rule([ts], 'OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw=', cfg['RS_PUB_KEY']),
        # Our rule above leaves `port` and `roffset` of getrUriFromRs unread; tsc refuses it with these
        Rule(['flutter/web/js/tsconfig.json'], '"noUnusedLocals": true', '"noUnusedLocals": false'),
        Rule(['flutter/web/js/tsconfig.json'], '"noUnusedParameters": true', '"noUnusedParameters": false'),
        # Upstream's Flutter code waits for window.onInitFinished (lib/models/web_model.dart, PlatformFFI.init), which
        # the JavaScript of the tag never calls: the app stays on its loading spinner forever
        Rule(['flutter/web/js/src/globals.js'], "console.log('init done');\n}",
             "console.log('init done');\n  window.onInitFinished && window.onInitFinished();\n}"),
        # The web client of 1.2.5 is half way through upstream's rewrite: its Dart bridge throws on three calls that the
        # home page and the toolbar make, which stops the app on its loading spinner or on the first connection
        Rule(['flutter/lib/web/bridge.dart'],
             'String mainGetAppNameSync({dynamic hint}) {\n    throw UnimplementedError();',
             f'String mainGetAppNameSync({{dynamic hint}}) {{\n    return "{app}";'),
        Rule(['flutter/lib/web/bridge.dart'],
             'bool mainIsOptionFixed({required String key, dynamic hint}) {\n    throw UnimplementedError();',
             'bool mainIsOptionFixed({required String key, dynamic hint}) {\n    return false;'),
        Rule(['flutter/lib/web/bridge.dart'],
             'bool mainGetUseTextureRender({dynamic hint}) {\n    throw UnimplementedError();',
             'bool mainGetUseTextureRender({dynamic hint}) {\n    return false;'),
        # The web client has no settings dialog for the key: hbbs started with -k refuses a request without it
        # ("Key mismatch"), so the key of the brand is the default of both requests (punch hole and relay)
        Rule(['flutter/web/js/src/connection.ts'], 'licence_key: localStorage.getItem("key") || undefined,',
             f'licence_key: localStorage.getItem("key") || "{cfg["RS_PUB_KEY"]}",'),
        # The Dart code stores the login token as the local option "access_token", which the browser keeps under
        # "option:local:access_token"; the punch hole request read "access_token" and so never sent it, and a server
        # started with --logged-in-only answered "You have not logged in."
        Rule(['flutter/web/js/src/connection.ts'], 'token: localStorage.getItem("access_token") || undefined,',
             'token: localStorage.getItem("option:local:access_token") || localStorage.getItem("access_token") || undefined,'),
        # Without an "api-server" option (a fresh browser) the web client asks admin.rustdesk.com for the login options,
        # so the OAuth2 buttons never show: the API server of the brand is the default
        Rule(['flutter/web/js/src/globals.js'], "  return 'https://admin.rustdesk.com';\n}",
             f"  return '{api}';\n}}"),
        # The login of the browser (OAuth2, and the id, uuid and device info of the password login): JavaScript
        Rule(['flutter/web/js/src/globals.js'], 'function getApiServer() {\n', WEB_LOGIN_JS + 'function getApiServer(/* brand */) {\n'),
        Rule(['flutter/web/js/src/globals.js'], "    case 'option':\n      value = JSON.parse(value);\n      localStorage.setItem(value.name, value.value);\n",
             "    case 'account_auth':\n      oidcAuth(value);\n      break;\n    case 'account_auth_cancel':\n      oidcAuthCancel();\n      break;\n"
             "    case 'option':\n      value = JSON.parse(value);\n      localStorage.setItem(value.name, value.value); // brand\n"),
        Rule(['flutter/web/js/src/globals.js'], "    case 'api_server':\n      return getApiServer();\n",
             "    case 'account_auth_result':\n      return oidcAuthResult();\n    case 'web_id':\n      return getWebId();\n"
             "    case 'web_uuid':\n      return getWebUuid();\n    case 'login_device_info':\n      return getLoginDeviceInfo();\n"
             "    case 'api_server':\n      return getApiServer(); // brand\n"),
        Rule(['flutter/lib/web/bridge.dart'],
             'Future<void> mainAccountAuth(\n      {required String op, required bool rememberMe, dynamic hint}) {\n    throw UnimplementedError();',
             'Future<void> mainAccountAuth(\n      {required String op, required bool rememberMe, dynamic hint}) {\n'
             "    // synchronous: the browser opens the login window only while the click is being handled\n"
             "    js.context.callMethod('setByName', ['account_auth', jsonEncode({'op': op, 'remember_me': rememberMe})]);\n"
             '    return Future.value();'),
        Rule(['flutter/lib/web/bridge.dart'],
             'Future<void> mainAccountAuthCancel({dynamic hint}) {\n    throw UnimplementedError();',
             "Future<void> mainAccountAuthCancel({dynamic hint}) {\n    js.context.callMethod('setByName', ['account_auth_cancel', '']);\n    return Future.value();"),
        Rule(['flutter/lib/web/bridge.dart'],
             'Future<String> mainAccountAuthResult({dynamic hint}) {\n    throw UnimplementedError();',
             "Future<String> mainAccountAuthResult({dynamic hint}) {\n    return Future.value(js.context.callMethod('getByName', ['account_auth_result']) as String);"),
        Rule(['flutter/lib/web/bridge.dart'],
             'Future<String> mainGetMyId({dynamic hint}) {\n    throw UnimplementedError();',
             "Future<String> mainGetMyId({dynamic hint}) {\n    return Future.value(js.context.callMethod('getByName', ['web_id']) as String);"),
        Rule(['flutter/lib/web/bridge.dart'],
             'Future<String> mainGetUuid({dynamic hint}) {\n    throw UnimplementedError();',
             "Future<String> mainGetUuid({dynamic hint}) {\n    return Future.value(js.context.callMethod('getByName', ['web_uuid']) as String);"),
        Rule(['flutter/lib/web/bridge.dart'],
             'String mainGetLoginDeviceInfo({dynamic hint}) {\n    throw UnimplementedError();',
             "String mainGetLoginDeviceInfo({dynamic hint}) {\n    return js.context.callMethod('getByName', ['login_device_info']) as String;"),
        # The landing page asks our API server for the download links
        Rule(['flutter/web/client.html'], '__API_SERVER__', api),
        Rule(['flutter/lib/**/*.dart'], 'RustDesk', app),
    ]


def rules_for(cfg, forced_api, local_build):
    app, low = cfg['APP_NAME'], cfg['APP_NAME'].lower()
    org = cfg['ORG_NAME']
    holder = os.environ.get('COPYRIGHT_HOLDER') or 'SCTG Development'
    api = f"{cfg['API_SERVER_PROTOCOL']}://{cfg['API_SERVER']}"
    site = f'https://{org}.eu.org'.lower()
    # Where a user who wants a server of their own is sent (the link at the bottom of the main window)
    project = os.environ.get('PROJECT_URL') or 'https://github.com/sctg-development/sctgdesk-server'
    rules = [
        # Where the client connects by default
        Rule(['libs/hbb_common/src/config.rs'], UPSTREAM_RENDEZVOUS, cfg['RENDEZVOUS_SERVER']),
        Rule(['libs/hbb_common/src/config.rs'], UPSTREAM_KEY, cfg['RS_PUB_KEY']),
        Rule(['src/common.rs'], '"https://admin.rustdesk.com".to_owned()', f'"{api}".to_owned()'),
        # In WebSocket mode a client reaches a domain on ws(s)://<domain>/ws/id, wss only when the "api-server" option
        # starts with https. That option is empty for us (our API server is forced above), so the client used ws:
        # on port 80, in clear, with its token. Use the scheme of our API server.
        Rule(['libs/hbb_common/src/websocket.rs'], 'let api_server = Config::get_option("api-server");',
             f'let api_server = "{api}".to_owned();'),
        # About pages and download links
        Rule(['flutter/lib/desktop/pages/desktop_setting_page.dart',
              'flutter/lib/mobile/pages/settings_page.dart'], 'https://rustdesk.com', site),
        # "Set up your own server": upstream's pricing page (Flutter) and blog article (Sciter) sell its own server
        Rule(['flutter/lib/desktop/pages/connection_page.dart'], 'const url = "https://rustdesk.com/pricing";',
             f'const url = "{project}";'),
        Rule(['src/ui/index.tis'], 'handler.open_url("https://rustdesk.com/blog/id-relay-set/");',
             f'handler.open_url("{project}");'),
        # The copyright keeps upstream's holder and adds ours: each rule anchors on what follows the holder, so that
        # a second run does not add it twice.
        Rule(['flutter/lib/desktop/pages/desktop_setting_page.dart'], 'Pte. Ltd.\\n$license',
             f'Pte. Ltd. and {holder}\\n$license'),
        Rule(['src/ui/index.tis'], '2026 Purslane Tech Pte. Ltd.\\', f'2026 Purslane Tech Pte. Ltd. and {holder}\\'),
        Rule(['flutter/windows/runner/Runner.rc', 'Cargo.toml', 'libs/portable/Cargo.toml',
              'flutter/macos/Runner/Configs/AppInfo.xcconfig'],
             'Pte. Ltd. All rights reserved.', f'Pte. Ltd. and {holder} All rights reserved.'),
        Rule(['flutter/lib/**/*.dart'], 'https://rustdesk.com/download', f'{api}/api/software/download'),
        Rule(['flutter/lib/desktop/pages/desktop_home_page.dart'], "contains('rustdesk')", f"contains('{low}')"),
        # Names
        Rule(['**/*.dart', '**/*.rs', '**/*.toml'], 'RustDesk', app),
        # ... except the names that are shared with code we do not build: a symbol of a C library, the files of
        # the drivers, the window that the TopMost DLL creates. Renaming them breaks the link or the feature.
        Rule(['src/platform/windows.rs'], f'Delete{app}TestCertsW', 'DeleteRustDeskTestCertsW'),
        Rule(['libs/virtual_display/dylib/src/win10/mod.rs'], f'{app}IddDriver', 'RustDeskIddDriver'),
        Rule(['libs/remote_printer/src/lib.rs'], f'{app}PrinterDriver', 'RustDeskPrinterDriver'),
        Rule(['src/privacy_mode/win_topmost_window.rs', 'libs/scrap/src/dxgi/mag.rs'],
             f'{app}PrivacyWindow', 'RustDeskPrivacyWindow'),
        Rule(['**/*.toml'], '"rustdesk"', f'"{low}"'),
        # the package of Cargo.toml is renamed above: the lockfile has to follow, builds use --locked
        Rule(['Cargo.lock'], 'name = "rustdesk"\nversion', f'name = "{low}"\nversion'),
        Rule(['**/*.toml', '**/*.rc'], '"rustdesk.exe"', f'"{low}.exe"'),
        Rule(['**/*.rc'], 'RustDesk', app, optional=True),
        Rule(['**/*.rc'], 'rustdesk', low, optional=True),
        Rule(['libs/portable/generate.py'], 'rustdesk.exe', f'.{low}.exe'),
        # Flatpak packaging is not verified on upstream 1.5.0 (the manifest no longer has this name)
        Rule(['flatpak/rustdesk.json'], 'rustdesk-', f'{low}-', optional=True),
        Rule(['**/*.plist', '**/*.xcscheme', '**/*.xcconfig', '**/*.desktop', '**/*.service'],
             'RustDesk', app),
        # upstream's build.py hardcodes the macOS bundle name
        Rule(['build.py'], 'RustDesk.app', f'{app}.app'),
        # Bundle identifiers
        Rule(['**/*.plist'], 'com.carriez.rustdesk', f'com.{org}.{low}'.lower()),
        Rule(['**/*.plist', '**/*.pbxproj', '**/*.xcconfig'], 'com.carriez.flutterHbb',
             f'com.{org}.{low}-ios'.lower()),
        Rule(['**/*.xcconfig'], 'com.carriez', f'com.{org}'.lower(), optional=True),
    ]
    team = re.search(r'\(([A-Z0-9]+)\)$', os.environ.get('MACOS_CODESIGN_IDENTITY', ''))
    if team:
        rules.append(Rule(['flutter/ios/**/*', 'flutter/macos/**/*'], UPSTREAM_TEAM_ID, team.group(1)))
    if forced_api:
        # Ignore the API server configured by the user: always use ours.
        rules.append(Rule(['src/common.rs'],
                          'fn get_api_server_(api: String, custom: String) -> String {\n',
                          'fn get_api_server_(api: String, custom: String) -> String {\n'
                          f'    return "{api}".to_owned();\n'))
    if local_build:
        cap = low.capitalize()
        rules.append(Rule(['flutter/lib/models/**/*.dart'], 'RustdeskImpl', f'{cap}Impl'))
    return rules


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    parser.add_argument('--root', default='.', help='upstream checkout to brand (default: .)')
    parser.add_argument('--check', action='store_true', help='report only, write nothing')
    parser.add_argument('--disable-api-server-forced', action='store_true',
                        help='let users choose their API server instead of forcing ours')
    parser.add_argument('--local-build', action='store_true', help='also rename the Dart bridge class')
    parser.add_argument('--web', action='store_true', help='brand the web client tree instead')
    args = parser.parse_args()

    cfg = {k: env(k) for k in ('APP_NAME', 'ORG_NAME', 'RENDEZVOUS_SERVER', 'RS_PUB_KEY', 'API_SERVER')}
    cfg['API_SERVER_PROTOCOL'] = env('API_SERVER_PROTOCOL', 'https')
    root = Path(args.root).resolve()
    if not (root / 'Cargo.toml').is_file():
        sys.exit(f'branding: {root} does not look like a RustDesk checkout')

    misses = []
    rules = (web_rules_for(cfg) if args.web
             else rules_for(cfg, not args.disable_api_server_forced, args.local_build))
    for rule in rules:
        changed, applied = apply_rule(root, rule, args.check)
        status = f'{changed} file(s)' if changed else ('already applied' if applied else 'NO MATCH')
        print(f'{status:>16}  {rule.old[:50]!r} in {rule.files[0]}{" ..." if len(rule.files) > 1 else ""}')
        if not changed and not applied and not rule.optional:
            misses.append(rule)

    # Nothing of upstream's identity may be left in the places that decide where we connect.
    config = root / ('flutter/web/js/src/connection.ts' if args.web else 'libs/hbb_common/src/config.rs')
    leaked = [v for v in (UPSTREAM_RENDEZVOUS, UPSTREAM_KEY, 'rs-sg.rustdesk.com', 'rs-cn.rustdesk.com',
                          'rs-us.rustdesk.com') if config.is_file() and v in read(config)]
    if leaked and not args.check:
        misses.append(Rule([str(config.relative_to(root))], 'upstream server or key still present', ''))

    if misses:
        print(f'\nbranding: {len(misses)} rule(s) matched nothing. The upstream layout changed; '
              'update client/branding.py before building.', file=sys.stderr)
        sys.exit(1)
    print('\nbranding: every rule applied')


if __name__ == '__main__':
    main()
