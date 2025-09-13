# rSnark

[![Crates.io](https://img.shields.io/crates/v/rsnark.svg)](https://crates.io/crates/rsnark)
[![Documentation](https://docs.rs/rsnark/badge.svg)](https://docs.rs/rsnark)

> Write zero-knowledge circuits in Rust for multiple prover systems.

rSnark is a Rust library for writing zero-knowledge circuits and generating proofs. It provides a core library to write circuits and a provers library to generate proofs using various backend implementations like Gnark.

## Features

- **Simple Circuit Definition**: Use derive macros to easily define circuits
- **Multiple Backend Support**: Write circuit once, prover on multiple prover system
- **Nested Circuits**: Compose complex circuits from simpler ones
- **Type Safety**: Leverage Rust's type system for circuit safety

## Getting Started

Add rSnark to your `Cargo.toml`:

```toml
[dependencies]
rsnark = "0.1.3"
```

or use `cargo add`:

```bash
cargo add rsnark
```

### Writing Your First Circuit

Defining a circuit requires two simple steps:

1. Define the circuit's inputs and outputs using the `#[derive(Circuit)]` macro
2. Implement the `Circuit` trait to define the circuit's constraint rules

Use the following way to define circuit:

```rust
use rsnark::{
    Groth16BN254GnarkProver,
    core::{API, Circuit, CircuitDefine, CircuitWitness},
};

#[derive(Circuit)]
pub struct TestCircuit {
    a: u32,        // private input
    b: u32,        // private input  
    pub c: u32,    // public input
}

impl Circuit for CircuitDefine<TestCircuit> {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b);
        api.assert_is_equal(&c, &self.c);
    }
}
```

Use these code to generate proof:

```rust
let prover = Groth16BN254GnarkProver::new();
let circuit_prover = prover.compile_circuit::<TestCircuit>().unwrap();
let (pk, vk) = circuit_prover.setup().unwrap();

let circuit_witness = TestCircuit {
    a: 3,
    b: 4,
    c: 7, // 3 + 4 = 7
};

let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();
let public_witness = circuit_witness.into_public_witness();
circuit_prover.verify(&vk, &proof, public_witness).unwrap();
```

## Circuit Private / Public Inputs

The `#[derive(Circuit)]` macro treats Rust's visibility modifiers as indicators:

- Fields **without** `pub` are treated as **private inputs**
- Fields **with** `pub` are treated as **public inputs**

> Note: Private inputs has higher priority, This will effect with [subcircuit](https://docs.rs/rsnark/latest/rsnark/#nested-circuits) struction.

## Export Verifier and Proof

```rust
// Export solidity contract from verifying key.
let solidity_contract = vk.export_solidity().unwrap();

// Export solidity proof.
let solidity_proof = proof.to_solidity().unwrap();
```

We will support these following platform:

- Solidity (with BN256 curve)
- [ ] Solana
- [ ] Ton
- [ ] Move
- [ ] ArkWorks

## Supported Prover Triples

Similar to compiler target triples, rSnark uses backend triples to define which backend, curve, and proving system to use. The format is: `{proving_system}-{curve}-{backend}`.

Currently supported backend triples:

### Groth16 Proof System

| Triple | Description |
|--------|-------------|
| `groth16-bn254-gnark` | Groth16 with BN254 curve using Gnark backend |
| `groth16-bls12_381-gnark` | Groth16 with BLS12-381 curve using Gnark backend |
| `groth16-bls12_377-gnark` | Groth16 with BLS12-377 curve using Gnark backend |
| `groth16-bls24_317-gnark` | Groth16 with BLS24-317 curve using Gnark backend |
| `groth16-bls24_315-gnark` | Groth16 with BLS24-315 curve using Gnark backend |
| `groth16-bw6_761-gnark` | Groth16 with BW6-761 curve using Gnark backend |
| `groth16-bw6_633-gnark` | Groth16 with BW6-633 curve using Gnark backend |
| `plonk-bn254-gnark` | PLONK with BN254 curve using Gnark backend |
| `plonk-bls12_381-gnark` | PLONK with BLS12-381 curve using Gnark backend |
| `plonk-bls12_377-gnark` | PLONK with BLS12-377 curve using Gnark backend |
| `plonk-bls24_317-gnark` | PLONK with BLS24-317 curve using Gnark backend |
| `plonk-bls24_315-gnark` | PLONK with BLS24-315 curve using Gnark backend |
| `plonk-bw6_761-gnark` | PLONK with BW6-761 curve using Gnark backend |
| `plonk-bw6_633-gnark` | PLONK with BW6-633 curve using Gnark backend |

## Project Structure

This workspace contains several crates:

- **`rsnark`** - Main library and unified API
- **`rsnark-core`** - Core circuit definition and API traits
- **`rsnark-macros`** - Derive macros for circuit definition
- **`rsnark-provers-core`** - Common prover traits and interfaces
- **`rsnark-provers-gnark`** - Gnark backend implementation
- **`rsnark-provers-mock`** - Mock prover for testing

## Examples

Check out the `examples/` directory for more detailed examples of how to use rSnark.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
