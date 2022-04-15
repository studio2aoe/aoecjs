#!/usr/bin/env bash
DIR_APP=$(realpath $(dirname $0))
DIR_CRATE=$(realpath ${DIR_APP}/crate)
DIR_RELEASE=${DIR_CRATE}/target/wasm32-unknown-unknown/release
DIR_DIST=${DIR_APP}/dist

printf "Build the crate to wasm\n\n"
printf "source crate: $(realpath ${DIR_CRATE})\n"
printf "wasm destination: $(realpath ${DIR_DIST})\n\n"

mkdir -p ${DIR_DIST}

cargo build\
  --release\
  --target wasm32-unknown-unknown\
  --manifest-path ${DIR_CRATE}/Cargo.toml

cp\
  ${DIR_RELEASE}/aoecjs.wasm\
  ${DIR_DIST}/aoecjs.0.0.1.wasm
printf "\n"

printf "Bulid the src with webpack"
npx webpack
