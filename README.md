![Image text](https://github.com/cloudmex/burrito-battle-minigames/blob/master/assets/Logotipo.png)

## Descripción 📄
Contratos inteligentes para los minijuegos de Burrito Battle.

## Instalación 🔧 

Para ejecutar este proyecto localmente, debe seguir los siguientes pasos:

Paso 1: requisitos previos

1. Asegúrese de haber instalado [Node.js] ≥ 12 (recomendamos usar [nvm])
2. Asegúrese de haber instalado yarn: `npm install -g yarn`
3. Instalar dependencias: `yarn install`
4. Cree una cuenta de prueba de NEAR
5. Instale NEAR CLI globalmente: [near-cli] es una interfaz de línea de comandos (CLI) para interactuar con NEAR blockchain.

Paso 2: Configure su NEAR CLI

Configure su near-cli para autorizar su cuenta de prueba creada recientemente:

    near login
    
## Despliegue 📦

Ejecute el siguiente comando dentro de cada carpeta (contracts) el cual generará nuestro archivo WASM en el directorio correspondiente (contracts/incursion/ , contracts/hospital/). Además de que la consola preguntará si deseamos desplegar el contrato correspondiente.
    
    ./build.sh

## Métodos de los contratos 🚀

Asignamos el identificador de nuestro contrato desplegado a una constante (Sustituir el ID por el del contrato desplegado):

    Incursion
    ID=dev-1657319025362-20400432440915
    echo $ID

    Hospital
    ID=dev-1658170507800-83790945510897
    echo $ID

Los contratos deben inicializarse antes de su uso, por lo que lo haremos con los siguientes comandos dependiendo del contrato:

    Incursion
    near call $ID new '{"owner_account_id": "yairnava.testnet", "treasury_id": "darkyair.testnet"}' --accountId yairnava.testnet

    Hospital
    near call $ID new '{"owner_account_id": "yairnava.testnet", "treasury_id": "darkyair.testnet", "cost_strw": 1000, "epoch_to_restore": 2}' --accountId yairnava.testnet

### Incursion

Crear una nueva incursión

    near call $ID create_incursion '{}' --accountId yairnava.testnet

Eliminar todas las incursiones

    near call $ID delete_all_incursions '{}' --accountId yairnava.testnet

Consultar incursion activa

    near view $ID get_active_incursion '{}'

Consultar incursiones en espera

    near view $ID get_awaiting_incursions '{}'

Registrarse en incursion y Transferir nft

    near call dev-1652924595303-59024384289373 nft_transfer_call '{"receiver_id": "dev-1657319025362-20400432440915","token_id":"152", "msg":"{\"incursion_id\":1}"}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

Mostrar la incursion activa de jugador

    near call $ID get_player_incursion --accountId yairnava.testnet

Crear sala de batalla

    near call $ID create_battle_room --accountId yairnava.testnet --gas 300000000000000

Verificar si el jugador tiene una sala de batalla creada

    near view $ID is_in_battle_incursion '{"account_id": "timoribus.testnet.testnet"}'

Obtener sala de batalla activa

    near call $ID get_active_battle_room --accountId noobmaster777.testnet

Verificar la información de los jugadores en batalla

    near view $ID burritos_incursion_info '{"incursion_id": 1}'
    
Combatir Ronda Player vs Mega Burrito [type_move => (1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse, 4 = Defenderse)]
    
    near call $ID battle_player_incursion '{"type_move":"'1'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_incursion '{"type_move":"'2'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_incursion '{"type_move":"'3'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_incursion '{"type_move":"'4'"}' --accountId yairnava.testnet --gas=300000000000000

Verificar si tiene un burrito para retirar

    near view $ID can_withdraw_burrito '{"account_id": "noobmaster777.testnet"}'

Recuperar burrito

    near call $ID withdraw_burrito_owner '{}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

### Hospital

Consultar información del contrato

    near view $ID get_contract_info

Ingresar burrito en capsula y Transferir nft

    near call dev-1652924595303-59024384289373 nft_transfer_call '{"receiver_id": "dev-1658170507800-83790945510897","token_id":"151", "msg":""}' --accountId yairnava.testnet --depositYocto 1 --gas 300000000000000

Consultar capsulas del jugador

    near view $ID get_player_capsules '{"player": "yairnava.testnet"}'

## Construido con 🛠️

* [RUST](https://www.rust-lang.org/) - Lenguaje de programación usado para contrato inteligente.
* [Rust Toolchain](https://docs.near.org/docs/develop/contracts/rust/intro#installing-the-rust-toolchain)
* [NEAR CLI](https://docs.near.org/docs/tools/near-cli) - Herramienta de interfaz de línea de comandos para interactuar con cuentas y contratos inteligentes en NEAR.
* [yarn](https://classic.yarnpkg.com/en/docs/install#mac-stable)