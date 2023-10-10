# NibiruChain/cw-nibiru

Smart contract sandbox for Nibiru Chain.

```bash
⚡ NibiruChain/cw-nibiru
├── 📂 artifacts         # compiled .wasm smart contracts for cw-nibiru
├── 📂 artifacts-cw-plus # compiled .wasm smart contracs from cw-plus
├── 📂 contracts         # Contracts for Nibiru Chain
    └── 📂 bindings-perp # Exposes queries and messages of the x/perp (and oracle) module of Nibiru.
    └── 📂 incentives    # Generalized incentives over time for locked tokens
    └── 📂 lockup        # For locking and unlocking tokens like LP tokens
    └── 📂 pricefeed     # CosmWasm version of the (now deprecated) x/pricefeed module.
    └── 📂 shifter       # Calls peg shift and depth shift in x/perp.
├── 📂 nibiru-std        # Nibiru Chain standard library for CosmsWasm smart contracts
├── 📂 packages          # Other Rust packages
    └── 📦 bindings    # For sending custom messages via the x/wasm module of Nibiru.
    └── 📦 macro       # Implements procedural macros for the "nibiru-macro" package. 
├── Cargo.toml
├── Cargo.lock
├── README.md
```

## Hacking

Install `just` to run project-specific commands.
```bash
cargo install just
```

You can view the list of available development commands with `just -ls`.

Ref: [github.com/casey/just](https://github.com/casey/just)