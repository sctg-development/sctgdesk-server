# winget package

Package identifier: `SCTG.SCTGDeskServer`. winget stores no file: each manifest points to the zip of a GitHub release and
gives its SHA-256. A new version is a pull request to [`microsoft/winget-pkgs`](https://github.com/microsoft/winget-pkgs),
opened by `.github/workflows/winget.yml` when a release is published.

The zip is `sctgdesk-server-<version>-winget-<arch>.zip`: the programs at its root (not in a versioned folder like the other
archives), so that the path of the nested installers never changes and the automatic update can reuse the previous
manifests.

## First submission (once, by hand)

The automation only updates a package that already exists. Create it from the files of this folder:

1. Copy them to `manifests/s/SCTG/SCTGDeskServer/<version>/` of a fork of `microsoft/winget-pkgs`, replace
   `<version>`, the URLs and the SHA-256 (`Get-FileHash` or `sha256sum`, they are in `SHA256SUMS-windows-macos` of the
   release).
2. Check: `winget validate --manifest <folder>` and `winget install --manifest <folder>` on a Windows machine.
3. Open the pull request. Microsoft's bots install the package in a sandbox and a moderator reviews the first one.

Then create the secret `WINGET_TOKEN` of this repository: a classic personal access token with the `public_repo` scope,
of the account that owns the fork of `winget-pkgs`.
