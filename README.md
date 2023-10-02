## Create Contract API

Rust API for create and deploy a smart contract.

```shell
# start server at localhost:7777
cargo run
```

## API

- `POST /api/contracts` - Create a new contract
- `GET /api/{contract_address}/total_supply` - Get total supply of token

## References

- https://docs.moonbeam.network/builders/build/eth-api/libraries/ethersrs/
- https://github.com/gakonst/ethers-rs

## Usage

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```

## Documentation

https://book.getfoundry.sh/
