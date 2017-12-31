#!/bin/sh

set -ex

# Setup some variables for executing cargo commands.
# Things are a little different if we're testing with cross.
if [ ! z "$CROSS_TARGET" ]; then
  rustup target add "$CROSS_TARGET"
  cargo install cross --force
  export CARGO_CMD="cross"
  export TARGET_PARAM="--target $CROSS_TARGET"
else
  export CARGO_CMD="cargo"
  export TARGET_PARAM=""
fi

# Test the build and docs.
"$CARGO_CMD" build --verbose $TARGET_PARAM
"$CARGO_CMD" doc --verbose $TARGET_PARAM

# If we're testing on an older version of Rust, then only check that we
# can build the crate. This is because the dev dependencies might be updated
# more frequently, and therefore might require a newer version of Rust.
#
# This isn't ideal. It's a compromise.
if [ "$TRAVIS_RUST_VERSION" = "1.12.0" ]; then
  exit
fi

"$CARGO_CMD" test --verbose $TARGET_PARAM
"$CARGO_CMD" test --verbose --no-default-features --lib $TARGET_PARAM
if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
  "$CARGO_CMD" test \
    --verbose --features i128 $TARGET_PARAM
  "$CARGO_CMD" test \
    --verbose --no-default-features --features i128 --lib $TARGET_PARAM
  "$CARGO_CMD" bench \
    --verbose --no-run $TARGET_PARAM
  "$CARGO_CMD" bench \
    --verbose --no-run --no-default-features $TARGET_PARAM
  "$CARGO_CMD" bench \
    --verbose --no-run --features i128 $TARGET_PARAM
  "$CARGO_CMD" bench \
    --verbose --no-run --no-default-features --features i128 $TARGET_PARAM
fi
