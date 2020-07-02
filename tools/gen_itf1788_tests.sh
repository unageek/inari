#!/usr/bin/env bash
set -eu

cd $(dirname $0)/..

tmp_dir=$(mktemp -d)
PYTHONPATH=ITF1788 python3 -m itf1788 -c '(rust, std, inari)' -s ITF1788/itl -o $tmp_dir
for f in $tmp_dir/rust/std/inari/*
do
    b=$(basename "$f")
    mv "$f" tests/itf1788_tests/$(echo "$b" | tr '-' '_')
done
