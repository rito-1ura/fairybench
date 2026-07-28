use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "genkey" {
        // Generate a fresh keypair
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;
        use rand::RngCore;

        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();

        println!("=== ED25519 KEYPAIR ===");
        println!("Public  (base64): {}", BASE64.encode(verifying_key.as_bytes()));
        println!("Private (base64): {}", BASE64.encode(&signing_key.to_bytes()));
        return;
    }

    if args.len() >= 5 && args[1] == "sign" {
        // Generate a license key: sign <name> <email> <days> <tier>
        use ed25519_dalek::{SigningKey, Signer};

        let name = &args[2];
        let email = &args[3];
        let days: u64 = args[4].parse().expect("days must be a number");
        let tier = &args[5];

        // Read private key from env var or embedded
        let priv_b64 = env::var("FB_PRIV_KEY").expect("Set FB_PRIV_KEY env var (base64 of ed25519 secret)");
        let priv_bytes = BASE64.decode(&priv_b64).expect("Invalid base64 private key");
        let mut key = [0u8; 32];
        key.copy_from_slice(&priv_bytes);
        let signing_key = SigningKey::from_bytes(&key);

        let expiry = SystemTime::now() + std::time::Duration::from_secs(days * 86400);
        let ts = expiry.duration_since(UNIX_EPOCH).unwrap().as_secs();

        let payload = format!("{}:{}:{}:{}", name, email, ts, tier);
        let sig = signing_key.sign(payload.as_bytes());
        let mut combined = payload.into_bytes();
        combined.extend_from_slice(&sig.to_bytes());
        let license_key = BASE64.encode(&combined);

        println!("{}", license_key);
        return;
    }

    // Default: print usage
    println!("FairyBench License Key Generator");
    println!();
    println!("USAGE:");
    println!("  cargo run -- genkey                    Generate a new ed25519 keypair");
    println!("  cargo run -- sign <name> <email> <days> <tier>   Generate a license key");
    println!();
    println!("To sign, set FB_PRIV_KEY env var to the base64-encoded private key.");
    println!("Example (Git Bash):");
    println!("  FB_PRIV_KEY=\"YQj0T93raHPnhriUgnYtD5TFm9cDOLgs/9IeYRda7YE=\" cargo run -- sign \"Acme\" \"a@b.com\" 365 business");
    println!("Example (CMD/PowerShell):");
    println!("  set FB_PRIV_KEY=YQj0T93raHPnhriUgnYtD5TFm9cDOLgs/9IeYRda7YE= && cargo run -- sign \"Acme\" \"a@b.com\" 365 business");
}