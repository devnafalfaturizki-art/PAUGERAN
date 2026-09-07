//! Unit tests for AES-256-GCM encryption.
//! 
//! [CB §63] — Encryption & Security

#[cfg(test)]
mod tests {
    use paugeran::crypto::aes_gcm::AesGcmCipher;

    #[test]
    fn test_encrypt_decrypt() {
        let cipher = AesGcmCipher::new(b"01234567890123456789012345678901");
        let plaintext = b"Super secret API key";
        
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_different_keys_produce_different_ciphertexts() {
        let cipher1 = AesGcmCipher::new(b"01234567890123456789012345678901");
        let cipher2 = AesGcmCipher::new(b"12345678901234567890123456789012");
        
        let plaintext = b"test";
        let ct1 = cipher1.encrypt(plaintext).unwrap();
        let ct2 = cipher2.encrypt(plaintext).unwrap();
        
        assert_ne!(ct1, ct2);
    }
}
