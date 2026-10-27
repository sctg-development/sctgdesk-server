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
  MACOS_CODESIGN_IDENTITY  optional, "Name (TEAMID)": its TEAMID replaces the upstream team id

  branding.py [--root DIR] [--check] [--disable-api-server-forced] [--local-build]

--check reports what would change and writes nothing.
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


def rules_for(cfg, forced_api, local_build):
    app, low = cfg['APP_NAME'], cfg['APP_NAME'].lower()
    org = cfg['ORG_NAME']
    api = f"{cfg['API_SERVER_PROTOCOL']}://{cfg['API_SERVER']}"
    site = f'https://{org}.eu.org'.lower()
    rules = [
        # Where the client connects by default
        Rule(['libs/hbb_common/src/config.rs'], UPSTREAM_RENDEZVOUS, cfg['RENDEZVOUS_SERVER']),
        Rule(['libs/hbb_common/src/config.rs'], UPSTREAM_KEY, cfg['RS_PUB_KEY']),
        Rule(['src/common.rs'], '"https://admin.rustdesk.com".to_owned()', f'"{api}".to_owned()'),
        # About pages and download links
        Rule(['flutter/lib/desktop/pages/desktop_setting_page.dart',
              'flutter/lib/mobile/pages/settings_page.dart'], 'https://rustdesk.com', site),
        Rule(['flutter/lib/desktop/pages/desktop_setting_page.dart'], 'Purslane Tech Pte. Ltd.',
             f'{org.capitalize()} and Purslane Tech Pte. Ltd.'),
        Rule(['flutter/lib/**/*.dart'], 'https://rustdesk.com/download', f'{api}/api/software/download'),
        Rule(['flutter/lib/desktop/pages/desktop_home_page.dart'], "contains('rustdesk')", f"contains('{low}')"),
        # Names
        Rule(['**/*.dart', '**/*.rs', '**/*.toml'], 'RustDesk', app),
        Rule(['**/*.toml'], '"rustdesk"', f'"{low}"'),
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
    args = parser.parse_args()

    cfg = {k: env(k) for k in ('APP_NAME', 'ORG_NAME', 'RENDEZVOUS_SERVER', 'RS_PUB_KEY', 'API_SERVER')}
    cfg['API_SERVER_PROTOCOL'] = env('API_SERVER_PROTOCOL', 'https')
    root = Path(args.root).resolve()
    if not (root / 'Cargo.toml').is_file():
        sys.exit(f'branding: {root} does not look like a RustDesk checkout')

    misses = []
    for rule in rules_for(cfg, not args.disable_api_server_forced, args.local_build):
        changed, applied = apply_rule(root, rule, args.check)
        status = f'{changed} file(s)' if changed else ('already applied' if applied else 'NO MATCH')
        print(f'{status:>16}  {rule.old[:50]!r} in {rule.files[0]}{" ..." if len(rule.files) > 1 else ""}')
        if not changed and not applied and not rule.optional:
            misses.append(rule)

    # Nothing of upstream's identity may be left in the places that decide where we connect.
    config = (root / 'libs/hbb_common/src/config.rs')
    leaks = [v for v in (UPSTREAM_RENDEZVOUS, UPSTREAM_KEY) if config.is_file() and v in read(config)]
    if leaks and not args.check:
        misses.append(Rule(['libs/hbb_common/src/config.rs'], 'upstream server or key still present', ''))

    if misses:
        print(f'\nbranding: {len(misses)} rule(s) matched nothing. The upstream layout changed; '
              'update client/branding.py before building.', file=sys.stderr)
        sys.exit(1)
    print('\nbranding: every rule applied')


if __name__ == '__main__':
    main()
