#!/usr/bin/env bash
#
# call `cargo test` with options I prefer
#

set -eu

cd "$(dirname -- "${0}")/.."

if [[ ! "${RUST_BACKTRACE+x}" ]]; then
    export RUST_BACKTRACE=1
fi

set -x

exec cargo test --verbose --future-incompat-report --all-features "${@}"
