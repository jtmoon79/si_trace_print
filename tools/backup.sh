#!/usr/bin/env bash
#
# backup.sh
#
# a quick manual backup script using 7zip
#

set -euo pipefail

cd "$(dirname "${0}")/.."

HERE="$(basename -- "$(realpath .)")"
ZIPFILE="../${HERE}-$(date '+%Y%m%dT%H%M%S')-$(hostname).zip"

Zz=$(which 7z)

(
set -x

"${Zz}" a -spf -bb1 -bt -stl -snl -tzip "${ZIPFILE}" \
    ./Cargo.toml \
    ./Cargo.lock \
    ./configs \
    ./.codecov.yml \
    ./.github \
    ./.gitignore \
    ./LICENSE \
    ./README.md \
    ./rustfmt.toml \
    ./src \
    ./tools \

"${Zz}" l "${ZIPFILE}"
)

echo -e "\n\n\n"

ls -lh "${ZIPFILE}"
