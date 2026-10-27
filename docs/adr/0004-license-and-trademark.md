# ADR 0004: License, AGPL compliance and the "RustDesk" mark

Status: Proposed

## Context

Verified from the repositories:

- All five repositories (our three, `rustdesk`, `rustdesk-server`) are **AGPL-3.0**. Our `LICENCE` / `LICENSE.md` files are not recognised by GitHub, which therefore reports no licence on our repositories.
- Source files we wrote carry an AGPL header ("SCTGDesk"). `debian/copyright` and the README mention RustDesk (README: 21 mentions).
- `hbbs --help` prints "Purslane Ltd. <info@rustdesk.com>" next to ours, and `Cargo.toml` lists a RustDesk address as author. Users of our binaries could end up sending fork bugs to upstream.
- `res/design.svg` and `res/logo-header.svg` in the client still contain the "RUSTDESK" wordmark.
- **Not verified**: RustDesk's trademark policy. No such file exists in `rustdesk`, `rustdesk-server` or `hbb_common`; the rustdesk.com footer links only Terms, Privacy and cookie settings; the Terms page was not read. I make no claim about what the mark owner permits.

## Decisions proposed

**Licence**
- The monorepo stays **AGPL-3.0-only** (or "or later", matching existing headers; to be confirmed against upstream's `LICENCE`). Merging our AGPL API server into an AGPL server needs no relicensing.
- Rename to `LICENSE` so GitHub and tooling detect it. Keep upstream's copyright notices untouched.
- AGPL section 13: a modified version offered over a network must offer its source to remote users. Add a source link in the web console footer, the `/api` version endpoint and `--version`, and publish exact sources per release tag.
- Add `cargo-deny` (licences + advisories) to CI to track third-party licences, plus a `THIRD_PARTY_LICENSES` generation step for the console (`gulp licenses` already exists).
- Contributions via DCO sign-off (`Signed-off-by`), not a CLA, so that nobody has to trust a single copyright holder.

**Trademark** (precautionary, pending the owner's actual policy)
- Treat "RustDesk" as a third-party mark. Use it only descriptively ("a fork of RustDesk", "compatible with the RustDesk client"), never as our product name, and add a non-affiliation notice to the README.
- Remove the RustDesk wordmark from any asset we ship; keep our own name and logo.
- Replace upstream contact addresses in `--help`, `Cargo.toml` and packaging metadata with ours.
- Binary names `hbbs`, `hbbr` and protocol identifiers stay: they are interface compatibility, not branding.
- If you want certainty, ask the owner in writing; this should be your decision before the public announcement.

## Consequences

- A small, mostly mechanical clean-up PR (Phase 2/3) for licence file, headers, contact addresses and assets.
- A legal review by someone qualified remains advisable; this ADR is an engineering position, not legal advice.
