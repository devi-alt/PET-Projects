
use aead::Key;
use curve25519_dalek::ristretto::CompressedRistretto;
use crate::schnorr::SchnorrSignature;

use crate::{keys::KeyPair, message::Message};

mod aes;
mod elgamal;
mod hybrid_enc;
mod keys;
mod message;
mod schnorr;
mod serializers;
mod test;

fn main() {
    // Load signing key (Scalar)
    let signing_key = KeyPair::from_file("signing_key.txt")
        .expect("Failed to load signing key")
        .private_key();

    // Load encryption public key (RistrettoPoint)
    let encryption_key =
        KeyPair::pk_from_file("encryption_key.txt")
            .expect("Failed to load encryption key");

    // Create plaintext message (just version + payload)
    let mut message = Message::new(
        1, // version
        b"Group ID: 123456".to_vec(), 
        CompressedRistretto::default(), // placeholder; encrypt() will overwrite
        CompressedRistretto::default(), // placeholder; sign() will overwrite
        SchnorrSignature::emty_signature(), // empty signature
    );

    // Encrypt message
    message.encrypt(&encryption_key).unwrap();

    // Sign message
    message.sign(&signing_key);

    // Write to file
    message.to_file("signed_encrypted_message.json").unwrap();

    println!("Message saved successfully!");
}
