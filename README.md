# cw-assessment
## Developing

If you have recently created a contract with this template, you probably could use some
help on how to build and test the contract, as well as prepare it for production. This
file attempts to provide a brief overview, assuming you have installed a recent
version of Rust already (eg. 1.54.0+).

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed. Currently, we are testing on 1.44.1+.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

## Compiling and running tests

Now that you created your custom contract, make sure you can compile and run it before
making any changes. Go into the

```sh
# this will produce a wasm build in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm

# this runs unit tests with helpful backtraces
RUST_BACKTRACE=1 cargo unit-test

# auto-generate json schema
cargo schema
```

The wasmer engine, embedded in `cosmwasm-vm` supports multiple backends:
singlepass and cranelift. Singlepass has fast compile times and slower run times,
and supportes gas metering. It also requires rust `nightly`. This is used as default
when embedding `cosmwasm-vm` in `go-cosmwasm` and is needed to use if you want to
check the gas usage.

However, when just building contacts, if you don't want to worry about installing
two rust toolchains, you can run all tests with cranelift. The integration tests
may take a small bit longer, but the results will be the same. The only difference
is that you can not check gas usage here, so if you wish to optimize gas, you must
switch to nightly and run with cranelift.

### Understanding the tests

The main code is in `src/contract.rs` and the unit tests there run in pure rust,
which makes them very quick to execute and give nice output on failures, especially
if you do `RUST_BACKTRACE=1 cargo unit-test`.

However, we don't just want to test the logic rust, but also the compiled Wasm artifact
inside a VM. You can look in `tests/integration.rs` to see some examples there. They
load the Wasm binary into the vm and call the contract externally. Effort has been
made that the syntax is very similar to the calls in the native rust contract and
quite easy to code. In fact, usually you can just copy a few unit tests and modify
a few lines to make an integration test (this should get even easier in a future release).

To run the latest integration tests, you need to explicitely rebuild the Wasm file with
`cargo wasm` and then run `cargo integration-test`.

We consider testing critical for anything on a blockchain, and recommend to always keep
the tests up to date. While doing active development, it is often simplest to disable
the integration tests completely and iterate rapidly on the code in `contract.rs`,
both the logic and the tests. Once the code is finalized, you can copy over some unit
tests into the integration.rs and make the needed changes. This ensures the compiled
Wasm also behaves as desired in the real system.

## Generating JSON Schema

While the Wasm calls (`init`, `handle`, `query`) accept JSON, this is not enough
information to use it. We need to expose the schema for the expected messages to the
clients. You can generate this schema by calling `cargo schema`, which will output
4 files in `./schema`, corresponding to the 3 message types the contract accepts,
as well as the internal `State`.

These files are in standard json-schema format, which should be usable by various
client side tools, either to auto-generate codecs, or just to validate incoming
json wrt. the defined schema.

## Generating `contract.wasm` and `hash.txt`
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.1
```

## Run local node

In this example, we will run the simple CosmWasm Smart Contract in the LocalTerra Blockchain.

### Prerequisites

- LocalTerra

LocalTerra is a complete Terra testnet and ecosystem containerized with Docker and orchestrated 
with a simple docker-compose file, designed to make it easy for smart contract developers to test 
out their contracts on a sandboxed environment before moving to a live testnet or mainnet.
Please use this guide to install the [LocalTerra Blockchain](https://github.com/terra-money/localterra)

- wasmd

wasmd is the backbone of CosmWasm platform. It is the implementation of a Cosmoszone with wasm 
smart contracts enabled.

```
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd
# replace the v0.16.0 with the most stable version on https://github.com/CosmWasm/wasmd/releases
git checkout v0.16.0
make install

# verify the installation
wasmd version
```

### Run Local Node

```
# default home is ~/.wasmd
# if you want to setup multiple apps on your local make sure to change this value
APP_HOME="~/.wasmd"
RPC="http://localhost:26657"
CHAIN_ID="localnet"
# initialize wasmd configuration files
wasmd init localnet --chain-id ${CHAIN_ID} --home ${APP_HOME}

# add minimum gas prices config to app configuration file
sed -i -r 's/minimum-gas-prices = ""/minimum-gas-prices = "0.01ucosm"/' ${APP_HOME}/config/app.toml

# Create main address
# --keyring-backend test is for testing purposes
# Change it to --keyring-backend file for secure usage.
export KEYRING="--keyring-backend test --keyring-dir $HOME/.wasmd_keys"
wasmd keys add main $KEYRING

# create validator address
wasmd keys add validator $KEYRING

# add your wallet addresses to genesis
wasmd add-genesis-account $(wasmd keys show -a main $KEYRING) 10000000000ucosm,10000000000stake --home ${APP_HOME}
wasmd add-genesis-account $(wasmd keys show -a validator $KEYRING) 10000000000ucosm,10000000000stake --home ${APP_HOME}

# add fred's address as validator's address
wasmd gentx validator 1000000000stake --home ${APP_HOME} --chain-id ${CHAIN_ID} $KEYRING

# collect gentxs to genesis
wasmd collect-gentxs --home ${APP_HOME}

# validate the genesis file
wasmd validate-genesis --home ${APP_HOME}

# run the node
wasmd start --home ${APP_HOME}
```