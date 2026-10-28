# Copyright 2024 SCTG Development - Ronan LE MEILLAT
# SPDX-License-Identifier: AGPL-3.0-or-later

# Single multi-stage image, built natively for linux/amd64 and linux/arm64 (static musl binaries).
# The libs/hbb_common git submodule must be checked out in the build context.
FROM node:lts-bookworm AS builder
RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev zip git sqlite3 musl-dev musl-tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN mkdir /build
RUN . /root/.cargo/env \
    && case "$(dpkg --print-architecture)" in \
         arm64) echo aarch64-unknown-linux-musl > /build/_target; ln -svf /usr/bin/ar /usr/bin/aarch64-linux-musl-ar ;; \
         amd64) echo x86_64-unknown-linux-musl > /build/_target ;; \
         *) echo "unsupported architecture $(dpkg --print-architecture)" >&2; exit 1 ;; \
       esac \
    && rustup target add "$(cat /build/_target)"
COPY src /build/src
COPY libs /build/libs
# the API server and its libraries are workspace members, the web console is embedded by their build script
COPY crates /build/crates
COPY web /build/web
COPY Cargo.toml Cargo.lock build.rs db_v2.sqlite3 /build/
RUN . /root/.cargo/env && cd /build \
    && DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo build --locked --release --target "$(cat /build/_target)" \
    && mkdir -p /build/bin \
    && for b in hbbs hbbr rustdesk-utils; do cp "/build/target/$(cat /build/_target)/release/$b" /build/bin/; done

FROM ubuntu:jammy
RUN mkdir -p /usr/local/bin
COPY --from=builder /build/bin/hbbs /usr/local/bin/hbbs
COPY --from=builder /build/bin/hbbr /usr/local/bin/hbbr
COPY --from=builder /build/bin/rustdesk-utils /usr/local/bin/rustdesk-utils
RUN mkdir -p /usr/local/share/sctgdesk
WORKDIR /usr/local/share/sctgdesk
