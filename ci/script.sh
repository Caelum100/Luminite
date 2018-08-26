#!/usr/bin/env bash
# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET --release --features=metal
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi