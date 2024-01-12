<div align="center">
    <h1>⚡DeFi Decimal Demo: AMM Constant Product Example⚡</h1>
</div>

#### Overview

This project serves as a demonstration of utilizing [decimal](https://github.com/invariant-labs/decimal) arithmetic in Decentralized Finance (DeFi), focusing on the example of an Automated Market Maker (AMM) with a constant product model.

## 🔨 Getting Started

### Prerequisites

- Rust & Cargo ([rustup](https://www.rust-lang.org/tools/install))
- cargo-contract ([cargo-contract](https://github.com/paritytech/cargo-contract))

#### Rust & Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### cargo-contract

```bash
rustup component add rust-src && cargo install --force --locked cargo-contract
```

## Installation

#### Clone the repository:

```bash
git clone git@github.com:invariant-labs/amm-decimal.git
```

#### Build project

```bash
cargo contract build
```

#### Run tests

```bash
cargo test
```
