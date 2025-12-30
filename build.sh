#!/usr/bin/env bash
set -euo pipefail

export SOURCE_DATE_EPOCH="1767078440"
export CARGO_BUILD_JOBS="1"
export CARGO_HOME=".home"

PWD_PATH="$(pwd)"

RUSTFLAGS=(
  "--remap-path-prefix=${PWD_PATH}=."
  "--remap-path-prefix=${HOME}=.home"
  "--remap-path-prefix=target=target"
)

export CARGO_ENCODED_RUSTFLAGS="$(IFS=$'\x1f'; echo "${RUSTFLAGS[*]}")"

cargo build --release
