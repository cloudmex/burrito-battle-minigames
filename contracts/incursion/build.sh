#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --all --target wasm32-unknown-unknown --release

if [ ! -d res/ ];
then
mkdir res
fi

ls
cp target/wasm32-unknown-unknown/release/incursion.wasm ./res/

echo "¿Quieres desplegar el contrato de incursion?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near dev-deploy --wasmFile res/incursion.wasm; break;;
        No ) exit;;
    esac
done