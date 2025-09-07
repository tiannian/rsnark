use rsnark_core::{Circuit, CircuitBuilder, CircuitElement};

use crate::{Backend, Curve};

pub struct Prover<B> {
    backend: B,
    curve: Curve,
}

impl<B> Prover<B>
where
    B: Backend,
{
    pub fn new(curve: Curve) -> Self {
        let backend = B::new();

        Self { backend, curve }
    }

    pub fn compile<C>(&self) -> B::CircuitConstraint
    where
        C: CircuitElement,
        C::Private: Circuit,
    {
        let mut builder = CircuitBuilder::default();
        let circuit = C::create_private(builder.variable_initer_mut());
        circuit.define(&mut builder);

        let define = builder.build();

        let cs = self.backend.compile(&self.curve, &define);

        cs
    }
}
