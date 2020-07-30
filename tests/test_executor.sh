#!/usr/bin/env bash
cd "$(dirname "$0")"

cd ../ && cargo build

cd "$(dirname "$0")"

json-server -c server/config.json server/db.json &

sleep 1
RUST_LOG="info" ../target/debug/osprey executor executor/config.json

wait