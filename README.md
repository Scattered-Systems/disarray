# Disarray

[![Clippy](https://github.com/scattered-systems/disarray/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/disarray/actions/workflows/clippy.yml)
[![Docker](https://github.com/Scattered-Systems/disarray/actions/workflows/docker.yml/badge.svg)](https://github.com/Scattered-Systems/disarray/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/disarray/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/disarray/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/disarray.svg)](https://crates.io/crates/disarray)
[![docs.rs](https://docs.rs/disarray/badge.svg)](https://docs.rs/disarray)

***

Disarray is a hybrid multichain derived from the recently formalized *Minotaur Protocol* which allows both stakers and miners to contribute blocks to the chain without comprimising the fidelity of the information communicated on the mainnet. The protocol employs the *modular, tweakable sleeve technology* enabling traditionally non-quantum elliptic-curve digital signature schemas to extend their security into a post-quantum world. Networking is handled primarily by *[libp2p](https://libp2p.io/)*, a complete peer-to-peer networking stack at the heart of several major projects from *[IPFS](https://ipfs.io)* to *Polkadot* / *[Substrate](https://substrate.io)*

## Features

Disarray features several critical capabilities enabling the multichain to service the proposed platform as a basis for orchestrating remote workflows offloaded from active devices via their Flow modules. The blockchain empolys several noveltites from the modules tweakable sleeve's for bolstering non-quantum elliptic-curve signatures to a hybrid composition which additionally utalizies the latest in consensus technologies to present a complete experience. Lastly, the system pushes towards WebAssembly (WASM) nativity, leveraging the WebAssembly System Interface (WASI) API to overcome several barriers previously prohibiting the practice.

## Roadmap

## Getting Started

Make sure you have docker installed on the target system

### Docker  

#### *Pull the image*

```bash
docker pull scsys/disarray:latest
```

#### *Build the image locally (optional)*

```bash
docker buildx build --tag disarray:latest .
```

#### *Run the image*

```bash
docker run \
    -p 9999:9999 \
    scsys/disarray:latest
```

### Usage

```bash
cargo run -- system --on
```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
