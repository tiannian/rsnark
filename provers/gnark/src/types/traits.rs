use rsnark_provers_core::CurveId;

use crate::{
    Error, Result,
    ffi::{self, Object},
};

/// Trait for types that wrap Go-side object references.
///
/// This trait provides a common interface for Rust types that manage
/// references to objects stored in the Go runtime. It enables safe
/// conversion between Rust wrapper types and Go object IDs.
pub trait GoInnerRef {
    /// Returns the Go-side object reference ID.
    fn go_inner_ref(&self) -> i64;

    /// Creates a new instance from a Go-side object reference ID.
    fn from_go_inner_ref(ref_id: i64) -> Self;
}

/// Trait for Go-side objects that support serialization operations.
///
/// This trait provides methods for serializing, deserializing, and
/// performing file I/O operations on Go-side objects through FFI calls.
pub trait InnerSerializableObject: GoInnerRef {
    fn inner_serialize(&self) -> Result<Vec<u8>> {
        let res = ffi::ObjectImpl::serialize(self.go_inner_ref());

        if res.is_empty() {
            return Err(Error::SerializeError);
        }

        Ok(res)
    }

    fn inner_deserialize<C>(ty: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
        C: CurveId,
    {
        let res = ffi::ObjectImpl::deserialize(ty, C::curve_id(), data);

        if res != 0 {
            return Err(Error::DeserializeError);
        }

        Ok(Self::from_go_inner_ref(res))
    }

    fn inner_write_to_file(object_id: i64, path: String) -> Result<()> {
        let res = ffi::ObjectImpl::write_to_file(object_id, path);

        if res == 0 {
            Ok(())
        } else {
            Err(Error::from_go_error(res))
        }
    }

    fn inner_read_from_file<C>(ty: u64, path: String) -> Result<Self>
    where
        Self: Sized,
        C: CurveId,
    {
        let res = ffi::ObjectImpl::read_from_file(ty, C::curve_id(), path);

        if res == 0 {
            Ok(Self::from_go_inner_ref(res))
        } else {
            Err(Error::from_go_error(res))
        }
    }
}

impl<T: GoInnerRef> InnerSerializableObject for T {}
