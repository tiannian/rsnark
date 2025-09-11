use std::marker::PhantomData;

use crate::{
    Error, Result, ffi,
    types::{GoInnerRef, InnerSerializableObject},
};
use rsnark_provers_core::{CurveId, curve::BN254};

macro_rules! impl_groth16_object {
    ($name:ident, $type_id:expr) => {
        impl<C> GoInnerRef for $name<C> {
            fn go_inner_ref(&self) -> i64 {
                self.go_ref_id
            }

            fn from_go_inner_ref(ref_id: i64) -> Self {
                Self {
                    go_ref_id: ref_id,
                    marker: PhantomData,
                }
            }
        }

        impl<C> $name<C> {
            pub fn serialize(&self) -> Result<Vec<u8>> {
                self.inner_serialize()
            }

            pub fn write_to_file(object_id: i64, path: String) -> Result<()> {
                Self::inner_write_to_file(object_id, path)
            }
        }

        impl<C> $name<C>
        where
            C: CurveId,
        {
            pub fn deserialize(data: Vec<u8>) -> Result<Self> {
                Self::inner_deserialize::<C>($type_id, data)
            }

            pub fn read_from_file(path: String) -> Result<Self> {
                Self::inner_read_from_file::<C>($type_id, path)
            }
        }
    };
}

/// Groth16 proving key wrapper for Go-side objects.
///
/// This type represents a proving key generated during the trusted setup phase
/// of the Groth16 protocol. The actual key data is stored in the Go runtime
/// and managed through FFI calls.
pub struct Groth16ProvingKey<C> {
    go_ref_id: i64,
    marker: PhantomData<C>,
}

impl_groth16_object!(Groth16ProvingKey, 1);

/// Groth16 verifying key wrapper for Go-side objects.
///
/// This type represents a verifying key generated during the trusted setup phase
/// of the Groth16 protocol. The verifying key is used to verify proofs without
/// access to the secret proving key.
pub struct Groth16VerifyingKey<C> {
    go_ref_id: i64,
    marker: PhantomData<C>,
}
impl_groth16_object!(Groth16VerifyingKey, 2);

/// Compiled circuit wrapper for Go-side constraint systems.
///
/// This type represents a circuit that has been compiled into Gnark's internal
/// constraint system format. It contains the optimized representation of the
/// circuit ready for setup and proving operations.
pub struct CompiledCircuit<C> {
    go_ref_id: i64,
    marker: PhantomData<C>,
}
impl_groth16_object!(CompiledCircuit, 3);

impl Groth16VerifyingKey<BN254> {
    /// Exports the verifying key as Solidity contract code.
    ///
    /// This method generates Solidity code that can be used to verify
    /// Groth16 proofs on-chain. The generated contract includes the
    /// verifying key data and verification logic.
    ///
    /// # Returns
    ///
    /// Returns the Solidity contract code as a string.
    ///
    /// # Errors
    ///
    /// This function may return an error if the Solidity export operation fails.
    pub fn export_solidity(&self) -> Result<String> {
        let res = ffi::object::export_solidity(self.go_ref_id);

        let code = i64::from_be_bytes(res[0..8].try_into().unwrap());

        if code != 0 {
            Err(Error::from_go_error(code))
        } else {
            let string = String::from_utf8(res[8..].to_vec())?;
            Ok(string)
        }
    }
}
