#[doc(inline)]
pub use rsnark_core as core;

pub mod provers {
    #[doc(inline)]
    pub use rsnark_provers_core as core;
    #[doc(inline)]
    pub use rsnark_provers_gnark as gnark;
    #[doc(inline)]
    pub use rsnark_provers_mock as mock;
}
