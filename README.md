# Disarray

[![Clippy](https://github.com/scattered-systems/disarray/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/disarray/actions/workflows/clippy.yml)
[![Docker](https://github.com/Scattered-Systems/disarray/actions/workflows/docker.yml/badge.svg)](https://github.com/Scattered-Systems/disarray/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/disarray/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/disarray/actions/workflows/rust.yml)

***

Disarray leverages a malleable core building off the component architecture being introduced in the WebAssembly Stack to deliver a high-performant,
post-quantum multi-chain supporting both PoS and PoW workloads as acceptable means of generating new blocks. This chain serves as the technical foundation 
for more rigorous services, Aether and Chaos, implementing optimized surfaces for executing a variety of different commands contextualized and proxied 
according to the active Flow module.

## Features

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
docker run -p 9090:9090 scsys/disarray:latest
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
