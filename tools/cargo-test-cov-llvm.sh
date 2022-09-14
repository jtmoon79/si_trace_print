#!/usr/bin/env bash
#
# run `cargo test` with code coverage report
#
# instructions from https://doc.rust-lang.org/rustc/instrument-coverage.html

set -eu

cd "$(dirname -- "${0}")/.."

if ! which llvm-profdata &>/dev/null; then
    echo "Cannot find llvm-profdata in PATH" >&2
    echo "Is 'llvm' installed? Try:" >&2
    echo "    apt install llvm" >&2
    exit 1
fi
if ! which jq &>/dev/null; then
    echo "Cannot find jq in PATH" >&2
    exit 1
fi


cov_file="instrument-coverage.profraw.json"
pdata_file="instrument-coverage.profdata"

rm -vf -- "${cov_file}" "${pdata_file}"

function list_objects () {
    declare objectf=
    for objectf in $(
        RUSTFLAGS="-C instrument-coverage" \
            cargo test --tests --no-run --message-format=json \
            | jq -r 'select(.profile.test == true) | .filenames[]' \
            | grep -v dSYM -
    ); do
        echo -n '--object' "${objectf}" ' '
    done
}

demangler=''
if cargo install --list | grep -q '^rustfilt '; then
    demangler='--Xdemangler=rustfilt'
fi

set -x

cargo clean

RUSTFLAGS="-C instrument-coverage" \
    LLVM_PROFILE_FILE="${cov_file}" \
    cargo test --tests

llvm-profdata merge -sparse "${cov_file}" -o "${pdata_file}"

llvm-cov report \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile="${pdata_file}" \
    $(list_objects)

llvm-cov show \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile="${pdata_file}" \
    --show-instantiations --show-line-counts-or-regions \
    "${demangler}" \
    $(list_objects)
