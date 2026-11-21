#!/usr/bin/env bash
# Builds hbbs, hbbr and rustdesk-utils for the MIPS routers of OpenWrt: 32-bit, MIPS32r2, soft float, musl.
#
#   packaging/openwrt/build-mips.sh mips     big endian    (OpenWrt ath79: mips_24kc)
#   packaging/openwrt/build-mips.sh mipsel   little endian (OpenWrt ramips: mipsel_24kc)
#
# The binaries end up in target/<arch>-unknown-linux-musl/release/. Needs: a nightly Rust with rust-src, and zig
# (`pip install ziglang`, or ZIG="/path/to/zig" for another one).
#
# Why this is not a plain `cargo build`:
# - `mips-unknown-linux-musl` is a tier 3 target: no prebuilt standard library, hence nightly and -Zbuild-std.
# - The project has C dependencies (libsodium, openssl, sqlite, ring): zig acts as the C compiler, the linker and
#   `ar`, with its own musl. Rust and the C code must agree on soft float, which zig's "musleabi" ABI is.
# - 32-bit MIPS has no 64-bit atomics, so the standard library has no AtomicU64 and several dependencies (the AWS
#   SDK, our own crates) need it. The target files in packaging/openwrt/targets/ are the builtin ones with
#   "max-atomic-width": 64: LLVM then calls __atomic_*_8, which zig's compiler-rt implements with a lock. Their file
#   name is the builtin triple on purpose: openssl-src and cc-rs recognise the target by its name.
# - `--cfg has_std`: the build script of indexmap 1.x probes for std with the stable sysroot and wrongly builds
#   without it when the standard library is built by cargo.
# - No PIE: a static-pie executable crashes at start-up on MIPS (the self-relocation of musl), a fixed-address
#   executable does not. Only the three programs are compiled and linked that way: the dependencies stay position
#   independent, because some of them (crc-fast) also build a shared library, which cannot be linked otherwise.
set -euo pipefail

arch="${1:?usage: $0 mips|mipsel}"
case "$arch" in mips|mipsel) ;; *) echo "unknown architecture $arch" >&2; exit 1 ;; esac

root="$(cd "$(dirname "$0")/../.." && pwd)"
triple="$arch-unknown-linux-musl"
spec="$root/packaging/openwrt/targets/$triple.json"
zig="${ZIG:-python3 -m ziglang}"
bin="$(mktemp -d)"
trap 'rm -rf "$bin"' EXIT

# cc-rs and rustc add --target=<rust triple>, which zig does not know: it already has -target
cat > "$bin/zigcc" <<WRAP
#!/bin/bash
args=()
for a in "\$@"; do
  case "\$a" in --target=*|-target=*) ;; *) args+=("\$a") ;; esac
done
exec $zig cc -target $arch-linux-musleabi -mcpu=mips32r2 "\${args[@]}"
WRAP
printf '#!/bin/sh\nexec %s ar "$@"\n' "$zig" > "$bin/zigar"
printf '#!/bin/sh\nexec %s ranlib "$@"\n' "$zig" > "$bin/zigranlib"
chmod +x "$bin"/zig*

var="${triple//-/_}"
export "CC_$var=$bin/zigcc" "AR_$var=$bin/zigar" "RANLIB_$var=$bin/zigranlib"
upper="$(printf %s "$var" | tr a-z A-Z)"
export "CARGO_TARGET_${upper}_LINKER=$bin/zigcc"
export RUSTFLAGS="--cfg has_std"
# the web console and the API documentation are embedded: they are built beforehand (see packages.yml)
cd "$root"
common=(-Zjson-target-spec -Zbuild-std=std,panic_abort --target "$spec" --release --locked)
# 1. everything, position independent (the programs it links are replaced by the next step)
cargo +nightly build "${common[@]}" --bins
# 2. each program again, at a fixed address
for b in hbbs hbbr rustdesk-utils; do
  cargo +nightly rustc "${common[@]}" --bin "$b" -- -C relocation-model=static -C link-arg=-no-pie
done
ls -l "target/$triple/release/hbbs" "target/$triple/release/hbbr" "target/$triple/release/rustdesk-utils"
