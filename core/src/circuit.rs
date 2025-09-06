use ruint::aliases::U256;

use crate::{API, PrivateVariable, PublicVariable};

pub trait Circuit {
    fn define(&self, api: &mut impl API);
}

pub trait CircuitElement {
    type Private;
    type Public;

    fn append_public(&self, witness: &mut Vec<U256>);
    fn append_private(&self, witness: &mut Vec<U256>);
}

pub type PrivateCircuitElement<T> = <T as CircuitElement>::Private;
pub type PublicCircuitElement<T> = <T as CircuitElement>::Public;

impl CircuitElement for U256 {
    type Private = PrivateVariable;
    type Public = PublicVariable;

    fn append_public(&self, witness: &mut Vec<U256>) {
        witness.push(*self);
    }

    fn append_private(&self, witness: &mut Vec<U256>) {
        witness.push(*self);
    }
}

impl CircuitElement for u128 {
    type Private = PrivateVariable;
    type Public = PublicVariable;

    fn append_public(&self, witness: &mut Vec<U256>) {
        let x = U256::from(*self);
        witness.push(x);
    }

    fn append_private(&self, witness: &mut Vec<U256>) {
        let x = U256::from(*self);
        witness.push(x);
    }
}

impl CircuitElement for u64 {
    type Private = PrivateVariable;
    type Public = PublicVariable;

    fn append_public(&self, witness: &mut Vec<U256>) {
        let x = U256::from(*self);
        witness.push(x);
    }

    fn append_private(&self, witness: &mut Vec<U256>) {
        let x = U256::from(*self);
        witness.push(x);
    }
}

impl CircuitElement for bool {
    type Private = PrivateVariable;
    type Public = PublicVariable;

    fn append_public(&self, witness: &mut Vec<U256>) {
        let x = U256::from(*self);
        witness.push(x);
    }

    fn append_private(&self, witness: &mut Vec<U256>) {
        let x = U256::from(*self);
        witness.push(x);
    }
}
