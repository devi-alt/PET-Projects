extern crate aes_gcm;
extern crate curve25519_dalek;
extern crate rand;

use aead::Key; 
use aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, KeyInit}; // Use KeyInit for the `new` method
use aes_gcm::{Aes256Gcm, Nonce, aes}; // AES-GCM with 256-bit key
use curve25519_dalek::scalar::Scalar;
use rand::{RngCore, rngs};
use rand::{rngs::OsRng, Rng};

const AES_KEY_SIZE: usize = 32; // AES-256 requires a 256-bit key (32 bytes)
pub const AES_NONCE_SIZE: usize = 12; // Recommended nonce size for AES-GCM is 12 bytes

/// Struct to hold the AES ciphertext and nonce
pub struct AESCiphertext {
    pub nonce: [u8; AES_NONCE_SIZE], // The nonce used for encryption
    pub ciphertext: Vec<u8>,         // The encrypted message
}

impl AESCiphertext {
    /// Display nonce and ciphertext as hex for readability
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("Nonce: {:?}", self.nonce);
        println!("Ciphertext: {:?}", self.ciphertext);
    }

    /// Generates a random scalar to be used as an AES key
    pub fn keygen() -> Scalar {
       let k =  Scalar::random(&mut OsRng); 
       k
        
    }

    /// Converts a Scalar into a 32-byte array to be used as an AES key
    fn scalar_to_aes_key(scalar: &Scalar) -> [u8; AES_KEY_SIZE] {
        scalar.to_bytes() // Scalar provides a 32-byte output
    }

    /// Encrypts a plaintext message using AES-256-GCM with a Scalar as the AES key
    pub fn encrypt_message(scalar_key: &Scalar, message: &[u8]) -> Result<AESCiphertext, String> {

        // 1. Generate nonce
    let mut nonce_bytes = [0u8; AES_NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 2. Convert scalar to AES key bytes
    let key_bytes = Self::scalar_to_aes_key(scalar_key);

    // 3. Create AES key object
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 4. Create cipher
    let cipher = Aes256Gcm::new(key);

    // 5. Encrypt
    let ciphertext = cipher
        .encrypt(nonce, message)
        .map_err(|_| "encryption failed".to_string())?;

    // 6. Return result
    Ok(AESCiphertext {
        nonce: nonce_bytes,
        ciphertext,
    })
    }

    /// Decrypts a ciphertext using AES-256-GCM with a Scalar as the AES key
    pub fn decrypt(scalar_key: &Scalar, aes_ciphertext: &AESCiphertext) -> Result<Vec<u8>, String> {

       // 1. Convert scalar to AES key bytes
    let key_bytes = Self::scalar_to_aes_key(scalar_key);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 2. Create AES cipher
    let cipher = Aes256Gcm::new(key);

    // 3. Wrap nonce from the struct
    let nonce = Nonce::from_slice(&aes_ciphertext.nonce);

    // 4. Decrypt
    let plaintext = cipher
        .decrypt(nonce, aes_ciphertext.ciphertext.as_ref())
        .map_err(|_| "decryption failed".to_string())?;

    Ok(plaintext)


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_correctness() {
        let key = AESCiphertext::keygen();
        // Message to encrypt
        let message = b"Hello, AES-GCM encryption using Scalar as the key!";

        // Encrypt the message
        let aes_ciphertext = AESCiphertext::encrypt_message(&key, message).expect("Encryption failed");

        // Decrypt the message
        let decrypted_message =
            AESCiphertext::decrypt(&key, &aes_ciphertext).expect("Decryption failed");

        // Ensure the decrypted message matches the original message
        assert_eq!(
            decrypted_message, message,
            "Decrypted message should match the original plaintext"
        );
    }
}
