use base64::{decode_config, URL_SAFE};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn decode_jwt(jwt: &str) -> String {
    // Split in tree part (header, payload, signature)
    let parts: Vec<&str> = jwt.split('.').collect();

    if parts.len() != 3 {
        return "Invalid JWT format".to_string();
    }

    // Extract payload part
    let payload_base64 = parts[1];

    // 3. Décoder la partie payload en base64url
    let decoded_payload = decode_base64url(payload_base64);

    match decoded_payload {
        Ok(payload) => {
            // 4. Convertir le payload en JSON et le retourner
            match serde_json::from_slice::<Value>(&payload) {
                Ok(json_payload) => serde_json::to_string_pretty(&json_payload).unwrap_or_else(|_| "Error serializing payload".to_string()),
                Err(_) => "Error parsing payload as JSON".to_string(),
            }
        }
        Err(_) => "Error decoding base64 payload".to_string(),
    }
}

///décoder un base64url (base64 avec des caractères différents)
fn decode_base64url(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    // URL_SAFE consider padding and spécific character
    decode_config(encoded, URL_SAFE)
}
