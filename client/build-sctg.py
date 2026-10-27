#!/usr/bin/env python3
"""SCTGDesk build entry point.

Brands the upstream checkout it sits in (see branding.py), then runs upstream's own build.py
with the remaining arguments, so that a new upstream build option works without a change here.

  build-sctg.py [--no_customization] [--local_build] [--disable_api_server_forced] [--ios] [build.py options]

Environment: see branding.py; ARCH (default amd64) is passed to build.py as DEB_ARCH;
MACOS_CODESIGN_IDENTITY signs the macOS app and dmg.
"""
import argparse
import os
import platform
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
OSX = platform.system() == 'Darwin'


def run(cmd, **kw):
    print('+', cmd if isinstance(cmd, str) else ' '.join(cmd), flush=True)
    if subprocess.call(cmd, shell=isinstance(cmd, str), **kw) != 0:
        sys.exit(f'build-sctg: command failed: {cmd}')


def version():
    for line in (HERE / 'Cargo.toml').read_text(encoding='utf-8').splitlines():
        if line.startswith('version'):
            return line.split('=')[1].strip().strip('"')
    sys.exit('build-sctg: no version in Cargo.toml')


def build_ios():
    run('cargo build --features flutter --release --target aarch64-apple-ios --lib')
    run('flutter build ipa --release --no-codesign', cwd=HERE / 'flutter')


def package_macos(app_name):
    """Sign the app and the dmg. Moved from the former build-sctg.py; not verified on upstream 1.5.0."""
    identity = os.environ.get('MACOS_CODESIGN_IDENTITY')
    if not identity:
        print('build-sctg: MACOS_CODESIGN_IDENTITY not set, the macOS app is not signed')
        return
    low = app_name.lower()
    flutter = HERE / 'flutter'
    app = f'./build/macos/Build/Products/Release/{app_name}.app'
    dmg = f'{low}-{version()}.dmg'
    run(f'codesign --force --options runtime -s "{identity}" --deep --strict {app} -vvv', cwd=flutter)
    run(f'create-dmg --volname "{app_name} Installer" --window-pos 200 120 --window-size 800 400 '
        f'--icon-size 100 --app-drop-link 600 185 --icon {app_name}.app 200 190 '
        f'--hide-extension {app_name}.app {low}.dmg {app}', cwd=flutter)
    os.rename(flutter / f'{low}.dmg', HERE / dmg)
    run(f'codesign --force --options runtime -s "{identity}" --deep --strict {dmg} -vvv', cwd=HERE)


def main():
    parser = argparse.ArgumentParser(description=__doc__, add_help=False,
                                     formatter_class=argparse.RawTextHelpFormatter)
    parser.add_argument('-h', '--help', action='store_true')
    parser.add_argument('--no_customization', action='store_true')
    parser.add_argument('--local_build', action='store_true')
    parser.add_argument('--disable_api_server_forced', action='store_true')
    parser.add_argument('--ios', action='store_true')
    args, rest = parser.parse_known_args()
    if args.help:
        print(__doc__)
        sys.exit(subprocess.call([sys.executable, str(HERE / 'build.py'), '--help']))

    if not args.no_customization:
        cmd = [sys.executable, str(HERE / 'branding.py'), '--root', str(HERE)]
        if args.local_build:
            cmd.append('--local-build')
        if args.disable_api_server_forced:
            cmd.append('--disable-api-server-forced')
        run(cmd)

    if args.ios:
        build_ios()
        return

    os.environ.setdefault('DEB_ARCH', os.environ.get('ARCH', 'amd64'))
    run([sys.executable, str(HERE / 'build.py')] + rest)
    if OSX and '--flutter' in rest and not args.no_customization:
        package_macos(os.environ['APP_NAME'])


if __name__ == '__main__':
    main()
