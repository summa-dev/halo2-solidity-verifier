//! Solidity verifier generator for [`halo2`] proof with KZG polynomial commitment scheme on BN254.
//!
//! [`halo2`]: http://github.com/privacy-scaling-explorations/halo2

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

mod codegen;
mod evm;
mod transcript;

#[cfg(test)]
mod test;

pub use codegen::{AccumulatorEncoding, BatchOpenScheme, SolidityGenerator};
pub use evm::{
    encode_calldata, get_key_calldata, FN_SIG_GET_VERIFYING_KEY,
    FN_SIG_GET_VERIFYING_KEY_WITH_VK_ADDRESS, FN_SIG_VERIFY_PROOF,
    FN_SIG_VERIFY_PROOF_WITH_VK_ADDRESS,
};
pub use transcript::Keccak256Transcript;

#[cfg(feature = "evm")]
pub use evm::test::{compile_solidity, revm, Evm};
