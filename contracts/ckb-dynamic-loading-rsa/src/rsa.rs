use ckb_std::dynamic_loading::{CKBDLContext, Symbol};
use crate::code_hashes::CODE_HASH_RSA;

type ValidateRsaSighashAll = unsafe extern "C" fn() -> i32;

/// Symbol name
const VALIDATE_RSA_SIGHASH_ALL: &[u8; 24] = b"validate_rsa_sighash_all";

pub struct RsaLib {
    validate_rsa_sighash_all: Symbol<ValidateRsaSighashAll>,
}

impl RsaLib {
    pub fn load<T>(context: &mut CKBDLContext<T>) -> Self {
        // load library
        let lib = context.load(&CODE_HASH_RSA).expect("load RSA");

        // find symbols
        let validate_rsa_sighash_all: Symbol<ValidateRsaSighashAll> = unsafe {
            lib
            .get(VALIDATE_RSA_SIGHASH_ALL)
            .expect("load function")
        };
        RsaLib {
            validate_rsa_sighash_all,
        }
    }

    pub fn validate_rsa_sighash_all(&self) -> Result<(), i32> {
        let f = &self.validate_rsa_sighash_all;
        let error_code = unsafe {f()};
        if error_code == 0 {
            Ok(())
        } else {
            Err(error_code)
        }
    }
}
