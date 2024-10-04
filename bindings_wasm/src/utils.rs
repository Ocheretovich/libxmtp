use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{JsError, JsValue};

pub fn to_value_with_bigint<T: serde::ser::Serialize + ?Sized>(
  value: &T,
) -> Result<JsValue, JsError> {
  let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
  value
    .serialize(&serializer)
    .map_err(|e| JsError::new(&format!("{e}")))
}
