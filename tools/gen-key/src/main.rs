fn main() {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
    use ed25519_dalek::{SigningKey, Signer};
    use rand::rngs::OsRng;
    use rand::RngCore;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Generate keypair: create random 32 bytes for secret key
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();

    let pub_b64 = BASE64.encode(verifying_key.as_bytes());
    let priv_b64 = BASE64.encode(&signing_key.to_bytes());

    println!("=== ED25519 KEYPAIR ===");
    println!("Public  (base64): {}", pub_b64);
    println!("Private (base64): {}", priv_b64);
    println!();

    // Generate a test license key
    let name = "Test User";
    let email = "test@example.com";
    let expiry = SystemTime::now() + std::time::Duration::from_secs(365 * 86400);
    let ts = expiry.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let tier = "business";

    let payload = format!("{}:{}:{}:{}", name, email, ts, tier);
    let sig = signing_key.sign(payload.as_bytes());
    let mut combined = payload.into_bytes();
    combined.extend_from_slice(&sig.to_bytes());
    let license_key = BASE64.encode(&combined);

    println!("=== TEST LICENSE KEY ===");
    println!("{}", license_key);
    println!();

    println!("=== INSTRUCTIONS ===");
    println!("1. Replace PUBLIC_KEY_BASE64 in src-tauri/src/license.rs with:");
    println!("   \"{}\"", pub_b64);
    println!("2. Save private key securely for generating real license keys");
    println!("3. Test the license key above in FairyBench Business activation");
}