#! /bin/env bash

rm -rf ./bin/*

# rustc --edition=2018 --target wasm32-unknown-unknown --out-dir ./bin ./src/lib.rs

# --extern hello=libhello.rlib
rustc -L . --edition=2018 --out-dir ./bin ./src/main.rs 
./bin/main

# tsc --target esnext --lib esnext,dom ./web/index.ts --outDir ./bin
# cp ./web/*{.html,.js} ./bin/


# Run web server
# python -m http.server -b 0.0.0.0 -d ./bin 8000
