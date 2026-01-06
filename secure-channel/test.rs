#[cfg(test)]
mod tests {
    use crate::keys::KeyPair;
    use crate::message::Message;
    use crate::schnorr::SchnorrSignature;
    use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};

    #[test]
    fn test_full_secure_channel_pipeline() {
        // Generate keys
        let signing_keys = SchnorrSignature::keygen();
        let encryption_keys = KeyPair::generate();

        let original_payload = b"Group ID: 123456".to_vec();

        // Use default points and wrap in CompressedRistretto
        let sender = CompressedRistretto(RistrettoPoint::default().compress().to_bytes());
        let recipient = CompressedRistretto(RistrettoPoint::default().compress().to_bytes());

        // Create message with empty signature
        let mut message = Message::new(
            0,
            original_payload.clone(),
            recipient,
            sender,
            SchnorrSignature::emty_signature(),
        );

        // Encrypt
        message
            .encrypt(&encryption_keys.public_key)
            .expect("Encryption failed");
        println!("Step 1 - Encrypted recipient: {:?}", message.recipient);

        // Sign
        message.sign(&signing_keys.private_key);
        println!("Step 2 - Signature after signing: {:?}", message.signature);

        // Verify signature on encrypted payload
        assert!(message.verify(), "Signature verification on encrypted payload failed");

        // Decrypt
        message
            .decrypt(&encryption_keys.private_key)
            .expect("Decryption failed");
        println!("Step 3 - Signature after decryption: {:?}", message.signature);

        // Check payload
        assert_eq!(
            message.payload, original_payload,
            "Recovered plaintext does not match original"
        );
        println!("Decrypted message payload: {:?}", message.payload);
        println!(
            "Signature: R = {:?}, s = {:?}",
            message.signature.R.compress(),
            message.signature.s
        );

        // Verify signature on dec
    }
}