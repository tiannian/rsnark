use std::marker::PhantomData;

use rsnark_core::{Circuit, CircuitBuilder, CircuitWitness};

use crate::{Backend, CircuitProver, Curve};

pub struct Prover<B> {
    pub(crate) backend: B,
    pub(crate) curve: Curve,
}

impl<B> Prover<B>
where
    B: Backend,
{
    pub fn new(curve: Curve) -> Self {
        let backend = B::new();

        Self { backend, curve }
    }

    pub fn compile_circuit<C>(&self) -> CircuitProver<B, C>
    where
        C: CircuitWitness,
        C::PrivateElement: Circuit,
    {
        let mut builder = CircuitBuilder::default();
        let circuit = C::create_private(builder.variable_initer_mut());
        circuit.define(&mut builder);

        let define = builder.build();

        let cs = self.backend.compile(&self.curve, &define);

        CircuitProver {
            prover: Self {
                backend: self.backend.clone(),
                curve: self.curve.clone(),
            },
            constraint: cs,
            marker: PhantomData,
        }
    }

    // pub fn compile_constraints(&self, constraints: &[u8]) -> B::CircuitConstraint {
}
