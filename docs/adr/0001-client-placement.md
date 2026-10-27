# ADR 0001: Where the client lives

Status: Accepted (maintainer decision, 2026-10-27: overlay)

## Context

Measured on 2026-10-27:

- `sctgdesk` (client fork) diverges from `rustdesk/rustdesk` at `94addb1` (2024-07-04). It is **2,021 commits behind**, i.e. upstream moves roughly 2.7 commits a day.
- Our own change set is small: 191 non-merge commits but only **48 files**. Rust changes are about 48 lines in 4 files under `src/` (software-update check against our API server, a TCP switch). The rest is branding assets (`res/`), a Helm chart, a few `flutter/web` tweaks and about 120 CI/signing commits in `.github/workflows`.
- The client is a Flutter + Rust build with signing, notarisation and several platforms. Whether the existing CI still works after two years is **not verified**.
- `sctgdesk` has 12 stars and no push since 2024-07-04; the server and API server have 362 and 147.

## Options

1. **Full client in the monorepo (subdirectory with its full history).**
   - Pros: one place, one issue tracker, atomic cross-component changes.
   - Cons: the repository grows by the whole RustDesk client tree; every upstream sync touches thousands of files in the same repository that carries the server; `git blame` and bisect get noisy; conflicts in a 127k-star project's hot files. Highest maintenance cost of the three options.
2. **Overlay: patches and assets applied onto an upstream tag.**
   - Pros: our delta stays reviewable (about 48 files); updating the client means re-applying patches on a newer tag, and conflicts show up in a handful of files; upstream history is never copied.
   - Cons: patch maintenance discipline is required; a failing patch blocks the client build until fixed; contributors cannot just open the client in an IDE without running the apply step.
3. **Separate repository** (status quo, kept alive).
   - Pros: no monorepo growth; independent release cadence.
   - Cons: the situation that already failed once: the client was left at 2024-07 while the server moved; cross-component changes need two PRs.

## Recommendation

**Option 2, hosted inside the monorepo** under `client/`:

```
client/
  patches/        # git format-patch series, one patch per concern
  overlay/        # files copied over the upstream tree (res/, helm/, flutter/web tweaks)
  ci/             # client build/sign workflows (inputs: upstream tag)
  UPSTREAM_TAG    # the upstream tag the series is known to apply to
  apply.sh        # clones rustdesk/rustdesk at UPSTREAM_TAG, applies patches + overlay
```

This keeps the monorepo benefit you asked for (one repo, one tracker, one release story) while paying only for the 48-file delta instead of a copy of the upstream client. Candidly: a full-copy monorepo is the option I would expect to burn out a small team, because the client moves faster than the server and the sync work would land on the same few people.

The decision to *build* the client at all (signing certificates, store accounts) is separate and depends on whether you want to ship client binaries or only document the patches. I recommend shipping server binaries first and treating client binaries as a later milestone.

## Consequences

- Phase 3 imports `sctgdesk-api-server` with history; the client is **not** imported with history. The `sctgdesk` repository is archived with a pointer to `client/`.
- The 191 client commits stay reachable in the archived repository; the patch series records authorship.
- A CI job checks that `apply.sh` still applies on the latest upstream release tag, so drift is detected early.
