# rSnark

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
rsnark = "0.1.0"
```

### Writing Your First Circuit

Defining a circuit requires two simple steps:

1. Define the circuit's inputs and outputs using the `#[derive(Circuit)]` macro
2. Implement the `Circuit` trait to define the circuit's constraint rules

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

### Circuit Visibility Rules

The `#[derive(Circuit)]` macro treats Rust's visibility modifiers as indicators:

- Fields **without** `pub` are treated as **private inputs**
- Fields **with** `pub` are treated as **public inputs**

## Supported Prover Triples

Similar to compiler target triples, rSnark uses backend triples to define which backend, curve, and proving system to use. The format is: `{proving_system}-{curve}-{backend}`.

Currently supported backend triples:

| Triple | Description |
|--------|-------------|
| `groth16-bn254-gnark` | Groth16 with BN254 curve using Gnark backend |
| `groth16-bls12-381-gnark` | Groth16 with BLS12-381 curve using Gnark backend |
| `groth16-bls24-317-gnark` | Groth16 with BLS24-317 curve using Gnark backend |
| `groth16-bls12-377-gnark` | Groth16 with BLS12-377 curve using Gnark backend |
| `groth16-bw6-761-gnark` | Groth16 with BW6-761 curve using Gnark backend |
| `groth16-bls24-315-gnark` | Groth16 with BLS24-315 curve using Gnark backend |
| `groth16-bw6-633-gnark` | Groth16 with BW6-633 curve using Gnark backend |

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
