#!/usr/bin/env bash
set -euo pipefail

rust_versions=( 1.{42..34}.0 )
bitvec_versions=( 0.17.3 0.{16..10}.0 )

explain ()
{
    echo "$@" "(Rust $rust_version and bitvec $bitvec_version)"
}

try_all ()
{
    for rust_version in "${rust_versions[@]}";
    do
        for bitvec_version in "${bitvec_versions[@]}";
        do
            (
                set -e
                rm -f Cargo.lock
                rustup install $rust_version
                cargo +$rust_version update --package bitvec --precise $bitvec_version
            ) >&2 || {
                explain "Update failed"
                continue
            };
            cargo +$rust_version build --tests >&2 || {
                explain "Build failed"
                continue
            };
            cargo +$rust_version test >&2 && {
                explain "Tests passed"
            } || {
                explain "Tests failed"
            };
        done;
    done
}

try_all

