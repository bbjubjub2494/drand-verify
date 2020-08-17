use wasm_bindgen::prelude::*;

use super::points::g1_from_variable;
use super::verify::verify;

struct VerifyWebError(pub String);

impl From<hex::FromHexError> for VerifyWebError {
    fn from(source: hex::FromHexError) -> Self {
        Self(source.to_string())
    }
}

impl From<VerifyWebError> for JsValue {
    fn from(source: VerifyWebError) -> JsValue {
        JsValue::from_str(&source.0)
    }
}

/// This is the entry point from JavaScript.
///
/// The argument types are chosen such that the JS binding is simple
/// (u32 can be expressed as number, u64 cannot; strings are easier than binary data).
///
/// The result type is translated to an exception in case of an error
/// and too a boolean value in case of success.
#[wasm_bindgen]
pub fn drand_verify(
    pk_hex: &str,
    round: u32,
    previous_signature_hex: &str,
    signature_hex: &str,
) -> Result<bool, JsValue> {
    Ok(drand_verify_impl(
        pk_hex,
        round,
        previous_signature_hex,
        signature_hex,
    )?)
}

/// Like drand_verify but with the structured error type needed to translate between lower level errors and JsValue.
/// If you cn show me how to translate from hex::FromHexError to JsValue without this intermediate function,
/// I'd be happy to learn how.
fn drand_verify_impl(
    pk_hex: &str,
    round: u32,
    previous_signature_hex: &str,
    signature_hex: &str,
) -> Result<bool, VerifyWebError> {
    let pk = g1_from_variable(&hex::decode(pk_hex)?);
    let previous_signature = hex::decode(previous_signature_hex)?;
    let signature = hex::decode(signature_hex)?;
    let result = verify(&pk, round.into(), &previous_signature, &signature);
    Ok(result)
}
