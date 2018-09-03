#!/usr/bin/env bash
# This script takes care of testing your crate

set -ex

main() {
    cargo build --release --features=$FEATURES
    cargo test --release --features=$FEATURES
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi