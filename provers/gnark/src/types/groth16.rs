use crate::{
    Result,
    types::{CurveType, GoInnerRef, InnerSerializableObject},
};

macro_rules! impl_groth16_object {
    ($name:ident, $type_id:expr) => {
        impl GoInnerRef for $name {
            fn go_inner_ref(&self) -> i64 {
                self.0
            }

            fn from_go_inner_ref(ref_id: i64) -> Self {
                Self(ref_id)
            }
        }

        impl $name {
            pub fn serialize(&self) -> Result<Vec<u8>> {
                self.inner_serialize()
            }

            pub fn deserialize(curve: CurveType, data: Vec<u8>) -> Result<Self> {
                Self::inner_deserialize($type_id, curve, data)
            }

            pub fn write_to_file(object_id: i64, path: String) -> Result<()> {
                Self::inner_write_to_file(object_id, path)
            }

            pub fn read_from_file(ty: u64, curve_id: u64, path: String) -> Result<Self> {
                Self::inner_read_from_file(ty, curve_id, path)
            }
        }
    };
}

pub struct Groth16ProvingKey(i64);
impl_groth16_object!(Groth16ProvingKey, 1);

pub struct Groth16VerifyingKey(i64);
impl_groth16_object!(Groth16VerifyingKey, 2);

pub struct CompiledCircuit(i64);
impl_groth16_object!(CompiledCircuit, 3);
