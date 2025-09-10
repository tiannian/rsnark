use crate::{
    Error, Result,
    ffi::{self, Object},
    types::CurveType,
};

pub trait GoInnerRef {
    fn go_inner_ref(&self) -> i64;

    fn from_go_inner_ref(ref_id: i64) -> Self;
}

pub trait InnerSerializableObject: GoInnerRef {
    fn inner_serialize(&self) -> Result<Vec<u8>> {
        let res = ffi::ObjectImpl::serialize(self.go_inner_ref());

        if res.is_empty() {
            return Err(Error::SerializeError);
        }

        Ok(res)
    }

    fn inner_deserialize(ty: u64, curve: CurveType, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        let res = ffi::ObjectImpl::deserialize(ty, curve.to_curve_id(), data);

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

    fn inner_read_from_file(ty: u64, curve_id: u64, path: String) -> Result<Self>
    where
        Self: Sized,
    {
        let res = ffi::ObjectImpl::read_from_file(ty, curve_id, path);

        if res == 0 {
            Ok(Self::from_go_inner_ref(res))
        } else {
            Err(Error::from_go_error(res))
        }
    }
}

impl<T: GoInnerRef> InnerSerializableObject for T {}
