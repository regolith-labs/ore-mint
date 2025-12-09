# ORE Mint

ORE Mint manages the mint authority of the ORE token and secures emissions rules.


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.

## Instructions

- [`MintORE`](program/src/mint_ore.rs) - Mints new ORE tokens to the treasury.

## State
- [`Authority`](api/src/state/authority.rs) - Holds the mint authority.


## Tests

To run the test suite, use the Solana toolchain: 

```
cargo test-sbf
```

For line coverage, use llvm-cov:

```
cargo llvm-cov
```
