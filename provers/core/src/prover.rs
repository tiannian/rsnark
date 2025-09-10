use std::marker::PhantomData;

use rsnark_core::{Circuit, CircuitBuilder, CircuitWitness};

use crate::{Backend, CircuitProver, Curve};

pub struct Prover<B> {
    pub(crate) backend: B,
}

impl<B> Prover<B>
where
    B: Backend,
{
    pub fn new(curve: Curve) -> Self {
        let backend = B::new(curve);

        Self { backend }
    }

    pub fn compile_circuit<C>(&self) -> Result<CircuitProver<B, C>, B::Error>
    where
        C: CircuitWitness,
        C::PrivateElement: Circuit,
    {
        let mut builder = CircuitBuilder::default();
        let circuit = C::create_private(builder.variable_initer_mut());
        circuit.define(&mut builder);

        let define = builder.build();

        let cs = self.backend.compile(&define)?;

        Ok(CircuitProver {
            prover: Self {
                backend: self.backend.clone(),
            },
            constraint: cs,
            marker: PhantomData,
        })
    }

    // pub fn compile_constraints(&self, constraints: &[u8]) -> B::CircuitConstraint {
}
