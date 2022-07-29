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
select yn in "Si" "No"; 
do
    case $yn in
        Si ) 
                echo "Tipo de despliegue"
                select option in Dev Account
                do
                        case $option in
                                Dev)
                                        near dev-deploy --wasmFile res/incursion.wasm; break;;
                                Account)
                                        echo Ingrese la cuenta:
                                        read account
                                        near deploy $account --wasmFile res/incursion.wasm; break;;
                        esac
                done
                break;;
        No ) exit;;
    esac
done