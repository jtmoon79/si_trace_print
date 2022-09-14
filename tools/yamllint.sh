#!/usr/bin/env bash
#
# run `yamllint` on yaml files
#


set -eu

cd "$(dirname -- "${0}")/.."

set -x

"${PYTHON-python}" --version

"${PYTHON-python}" -m yamllint --version

exec "${PYTHON-python}" -m yamllint \
    ./.codecov.yml \
    ./configs/grcov.yml \
    ./.github/workflows/rust.yml \
