use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

// For MVP, we use a fixed derivation or a hardcoded machine-specific key.
// In a real app, this should be derived securely using machine-uid or similar.
const FIXED_KEY: &[u8; 32] = b"safebridge_mvp_secret_key_32byte";

pub fn encrypt_password(password: &str) -> Result<String, String> {
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(FIXED_KEY);
    let cipher = Aes256Gcm::new(key);
    
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, password.as_bytes().as_ref())
        .map_err(|e| format!("Encryption error: {:?}", e))?;
        
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);
    
    Ok(general_purpose::STANDARD.encode(combined))
}

pub fn decrypt_password(encrypted: &str) -> Result<String, String> {
    let combined = general_purpose::STANDARD.decode(encrypted)
        .map_err(|e| format!("Base64 decode error: {:?}", e))?;
        
    if combined.len() < 12 {
        return Err("Invalid ciphertext length".into());
    }
    
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(FIXED_KEY);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let decrypted = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {:?}", e))?;
        
    String::from_utf8(decrypted).map_err(|e| format!("UTF8 error: {:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_password_success() {
        // Dado un password en texto plano
        let original_password = "MySuperSecretPassword123!";
        
        // Cuando encriptamos el password
        let encrypted = encrypt_password(original_password).unwrap();
        
        // Entonces el resultado encriptado no debe ser igual al original y debe ser desencriptable
        assert_ne!(encrypted, original_password);
        
        let decrypted = decrypt_password(&encrypted).unwrap();
        assert_eq!(decrypted, original_password);
    }

    #[test]
    fn test_decrypt_invalid_ciphertext_length() {
        // Dado un texto cifrado inválido (muy corto)
        let invalid_encrypted = "short";
        
        // Cuando intentamos desencriptarlo
        let result = decrypt_password(invalid_encrypted);
        
        // Entonces debe retornar un error
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_invalid_base64() {
        // Dado un texto que no es base64 válido
        let invalid_base64 = "!!invalid_base64!!";
        
        // Cuando intentamos desencriptarlo
        let result = decrypt_password(invalid_base64);
        
        // Entonces debe retornar un error de base64 decode
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Base64 decode error"));
    }
}
