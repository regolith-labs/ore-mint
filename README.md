# ORE Mint

ORE Mint holds the mint authority for the ORE token and implements emissions constraints.


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/error.rs) – Custom program events.
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
