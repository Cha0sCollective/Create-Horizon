#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
workspace_root="$(cd -- "$repo_root/.." && pwd)"
local_tools="$workspace_root/.local/toolchains"

if [[ -x "$local_tools/cargo/bin/cargo" ]]; then
    export CARGO_HOME="$local_tools/cargo"
    export RUSTUP_HOME="$local_tools/rustup"
    export PATH="$CARGO_HOME/bin:$PATH"
fi
export RUSTUP_TOOLCHAIN=1.99.1

if [[ $# -eq 0 ]]; then
    printf 'Usage: tools/dev.sh <command> [arguments...]\n' >&2
    exit 2
fi
exec "$@"
