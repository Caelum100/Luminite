#!/usr/bin/env bash
# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cargo build --target $TARGET --release --features=$FEATURES

    cp target/$TARGET/release/luminite $stage/
    cp -r assets $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET_NAME.tar.gz *
    cd $src

    rm -rf $stage
}

main