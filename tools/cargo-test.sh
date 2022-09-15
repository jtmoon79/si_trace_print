#!/usr/bin/env bash
#
# call `cargo test` with options I prefer
#

set -eu

cd "$(dirname -- "${0}")/.."

set -x

exec cargo test --verbose --future-incompat-report --all-features "${@}"
