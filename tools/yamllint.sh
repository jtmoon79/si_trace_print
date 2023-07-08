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
    -c "configs/yamllint.yml" \
    ./.codecov.yml \
    ./configs/grcov.yml \
    ./configs/yamllint.yml \
    ./.github/dependabot.yml \
    ./.github/workflows/rust.yml \
