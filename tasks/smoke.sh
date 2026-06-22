#!/usr/bin/env bash

set -uo pipefail

if [ $# -eq 0 ]; then
  echo "Usage: mise run smoke <manifests_dir>"
  exit 1
fi

cargo build --release

binary="target/release/wgv"
pass=0
fail=0
total=0

dirlist=$(mktemp)
trap 'rm -f "$dirlist"' EXIT
find "$1" -name "*.yaml" | sed 's|/[^/]*$||' | sort -u | shuf > "$dirlist"

while IFS= read -r dir; do
  total=$((total + 1))

  if output=$($binary "$dir" --ignore-warnings 2>&1); then
    pass=$((pass + 1))
  else
    fail=$((fail + 1))
    echo "FAIL: $dir"
    echo "$output" | head -5
    echo ""
  fi
done < "$dirlist"

echo "Done. $pass passed, $fail failed out of $total total."
