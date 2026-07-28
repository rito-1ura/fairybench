#!/usr/bin/env bash
# FairyBench License Key Generator
# Usage: ./tools/gen-key.sh <name> <email> <expiry_days> <tier>
# Example: ./tools/gen-key.sh "Acme Corp" "licenses@acme.com" 365 business

set -euo pipefail

if [ $# -ne 4 ]; then
  echo "Usage: $0 <name> <email> <expiry_days> <tier>"
  echo "Example: $0 'Acme Corp' 'licenses@acme.com' 365 business"
  exit 1
fi

NAME="$1"
EMAIL="$2"
DAYS="$3"
TIER="$4"

# Private key (base64) — keep this SECRET!
PRIV_B64="YQj0T93raHPnhriUgnYtD5TFm9cDOLgs/9IeYRda7YE="

# Build and run the signing script
cd "$(dirname "$0")/gen-key"
cat > src/main.rs << RUSTEOF
fn main() {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
    use ed25519_dalek::{SigningKey, Signer};
    use std::time::{SystemTime, UNIX_EPOCH};

    let priv_bytes = BASE64.decode("${PRIV_B64}").unwrap();
    let mut key = [0u8; 32];
    key.copy_from_slice(&priv_bytes);
    let signing_key = SigningKey::from_bytes(&key);

    let name = "${NAME}";
    let email = "${EMAIL}";
    let tier = "${TIER}";
    let expiry = SystemTime::now() + std::time::Duration::from_secs(${DAYS} * 86400);
    let ts = expiry.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let payload = format!("{}:{}:{}:{}", name, email, ts, tier);
    let sig = signing_key.sign(payload.as_bytes());
    let mut combined = payload.into_bytes();
    combined.extend_from_slice(&sig.to_bytes());
    let key_b64 = BASE64.encode(&combined);

    println!("{}", key_b64);
}
RUSTEOF

cargo run --quiet 2>/dev/null