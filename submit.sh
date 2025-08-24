#!/usr/bin/env bash
set -euo pipefail

BIN="$1"
TARGET="$2"

CMD="cargo equip --bin $BIN --remove docs --minify libs"

if [[ "$TARGET" == "file" ]]; then
  $CMD > submission.rs
elif [[ "$TARGET" == "clipboard" ]]; then
  $CMD | pbcopy
else
  echo "usage: $0 BIN {file|clipboard}" >&2
  exit 1
fi