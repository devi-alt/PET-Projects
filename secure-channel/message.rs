use crate::hybrid_enc::HybridCiphertext;
use crate::keys::KeyPair;
use crate::schnorr::SchnorrSignature;
use crate::serializers::*;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub version: u8, // The version number of the message (1 byte)

    #[serde(
        serialize_with = "serialize_base64",
        deserialize_with = "deserialize_base64"
    )]
    pub payload: Vec<u8>, // The message content (or payload) stored as a Base64-encoded string in JSON.
    #[serde(
        serialize_with = "serialize_fixed_base64",
        deserialize_with = "deserialize_fixed_base64"
    )]
    pub recipient: [u8; 32], // The recipient's identifier (stored as Vec<u8> to serialize easily)
    #[serde(
        serialize_with = "serialize_fixed_base64",
        deserialize_with = "deserialize_fixed_base64"
    )]
    pub sender: [u8; 32], // The recipient's identifier (stored as Vec<u8> to serialize easily)
    #[serde(
        serialize_with = "serialize_schnorr_signature",
        deserialize_with = "deserialize_schnorr_signature"
    )]
    pub signature: SchnorrSignature,
}

impl Message {
    /// Creates a new message with a version, payload, and recipient (CompressedRistretto converted to Vec<u8>)
    pub fn new(
        version: u8,
        payload: Vec<u8>,
        sender: CompressedRistretto,
        recipient: CompressedRistretto,
        signature: SchnorrSignature,
    ) -> Self {
        Message {
        version,
        payload,
        sender: sender.to_bytes(),
        recipient: recipient.to_bytes(),
        signature,
    }
        
    }

    /// Writes the message to a JSON file
    pub fn to_file(&self, filepath: &str) -> std::io::Result<()> {
        let file = File::create(filepath)?;
        serde_json::to_writer_pretty(file, &self)?; // Write JSON in a human-readable format
        Ok(())
    }

    /// Encrypts the whole message using hybrid encryption
    pub fn encrypt(&mut self, recipient_pubkey: &RistrettoPoint) -> Result<(), String> {
        // Serialize the full message (payload, sender, recipient)
    let plaintext = serde_json::to_vec(self)
        .map_err(|e| format!("Failed to serialize message: {}", e))?;

    // Encrypt the serialized bytes with hybrid encryption
    let hybrid_ciphertext = HybridCiphertext::encrypt(&plaintext, recipient_pubkey)?;

    // Replace the payload with the serialized hybrid ciphertext
    self.payload = hybrid_ciphertext.serialize();

    // Set the recipient field
    self.recipient = recipient_pubkey.compress().to_bytes();

    // Set version to 1 (encrypted)
    self.version = 1;

    Ok(())
    }


    

  
  
    pub fn decrypt(&mut self, elgamal_private_key: &Scalar) -> Result<(), String> {
    // 1. Deserialize hybrid ciphertext from payload
    let hybrid_ciphertext: HybridCiphertext = HybridCiphertext::deserialize(&self.payload)
        .map_err(|e| format!("Failed to deserialize hybrid ciphertext: {}", e))?;

    // 2. Decrypt using recipient's private key
    let plaintext = hybrid_ciphertext.decrypt(elgamal_private_key)?;

    // 3. Deserialize plaintext into a full Message
    let decrypted: Message = serde_json::from_slice(&plaintext)
        .map_err(|e| format!("Failed to deserialize decrypted message: {}", e))?;

    // 4. Restore all fields except the signature
    // Do not overwrite the sender or recipient fields after decryption.
    self.version = decrypted.version;
    self.payload = decrypted.payload;
    // Leave sender and recipient unchanged
    // self.sender = decrypted.sender;
    // self.recipient = decrypted.recipient;

    Ok(())
}



    /// signs the payload using Schnorr signatures, sets the signing public key as sender
    pub fn sign(&mut self, signing_key: &Scalar) {
         
        // 1. Create a Schnorr signature over the payload
        let signature = SchnorrSignature::sign(&self.payload, signing_key);

        // 2. Derive the public key from the signing key
        let public_key = RistrettoPoint::mul_base(signing_key).compress();

        // 3. Store the signature and the sender's public key in the message
        self.signature = signature;
        self.sender = public_key.to_bytes();
    }
    

    pub fn verify(&self) -> bool {
       // 1. Recover the Schnorr public key from sender
    let compressed = CompressedRistretto(self.sender);
    let public_key = match compressed.decompress() {
        Some(pk) => pk,
        None => return false, // invalid sender key
    };

    // 2. Call the static verify function
    SchnorrSignature::verify(&self.signature, &self.payload, &public_key)
    }

    pub fn display(&self) {
        println!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use rand::rngs::OsRng;
    use std::fs;

    #[test]
    fn test_message_creation() {
        // Create a sample payload and recipient
        let payload = b"Hello, this is a message!".to_vec();
        let mut csprng = OsRng;
        let recipient = RistrettoPoint::random(&mut csprng).compress();

        // Create a new message
        let version: u8 = 1;
        let message = Message::new(
            version,
            payload.clone(),
            recipient,
            recipient,
            SchnorrSignature::emty_signature(),
        );

        // Check if the fields match
        assert_eq!(message.version, version);
        assert_eq!(message.payload, payload);
        assert_eq!(message.recipient, recipient.to_bytes());

        // Display the message
        message.display();
    }

    #[test]
    fn test_message_encryption_and_decryption() {
        // Sample message to encrypt
        let payload = b"Hello, hybrid encryption!".to_vec();

        // Generate ElGamal keypair
        let keypair = KeyPair::generate();

        // Create a new message with version 0
        let mut message = Message::new(
            0,
            payload.clone(),
            keypair.public_key.compress(),
            keypair.public_key.compress(),
            SchnorrSignature::emty_signature(),
        );

        // Encrypt the message
        message
            .encrypt(&keypair.public_key)
            .expect("Encryption failed");

        // Ensure the message version is 1 after encryption
        assert_eq!(message.version, 1, "Version should be 1 after encryption");

        // Ensure the payload is not the same as the original (it should be encrypted)
        assert_ne!(
            message.payload, payload,
            "Encrypted payload should not match the original payload"
        );

        // Decrypt the message
        message
            .decrypt(&keypair.private_key)
            .expect("Decryption failed");

        // Ensure the message version is back to 0 after decryption
        assert_eq!(message.version, 0, "Version should be 0 after decryption");

        // Ensure the decrypted message matches the original payload
        assert_eq!(
            message.payload, payload,
            "Decrypted payload should match the original payload"
        );
    }
}
