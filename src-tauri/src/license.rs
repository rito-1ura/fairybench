// FairyBench Business License Verification
// Offline license key validation using ed25519 public-key signatures.
//
// License key format (base64):
//   "name:email:expiry_timestamp" signed with developer's private key.
//
// The public key is embedded in the binary. No network required.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::time::{SystemTime, UNIX_EPOCH};

/// Embedded public key (ed25519) — raw 32-byte public key, base64-encoded.
/// Generate with:
///   openssl genpkey -algorithm ed25519 -out private.pem
///   openssl pkey -in private.pem -pubout -out public.der
/// Or use ed25519-dalek's generate() in a helper binary.
const PUBLIC_KEY_BASE64: &str = "EAB5SKvTeqVhsH04ZHARG/183rr6E8BwZB1ygiCIMHg=";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseInfo {
    pub name: String,
    pub email: String,
    pub verified: bool,
    pub expires_at: String,
    pub tier: String, // "business"
}

/// Verify a license key string.
/// Returns `LicenseInfo` with `verified: true` on success, or a rejected info.
pub fn verify_license(key: &str) -> LicenseInfo {
    // Decode base64
    let raw = match BASE64.decode(key.trim()) {
        Ok(d) => d,
        Err(_) => return rejected("Invalid encoding"),
    };
    // Format: last 64 bytes are the signature, rest is the payload
    if raw.len() < 64 {
        return rejected("Too short");
    }
    let sig_bytes = &raw[raw.len() - 64..];
    let payload = &raw[..raw.len() - 64];
    let payload_str = match std::str::from_utf8(payload) {
        Ok(s) => s,
        Err(_) => return rejected("Invalid UTF-8"),
    };
    // Parse "name:email:expiry_timestamp:tier"
    let parts: Vec<&str> = payload_str.split(':').collect();
    if parts.len() < 4 {
        return rejected("Malformed payload");
    }
    let name = parts[0].to_string();
    let email = parts[1].to_string();
    let expiry_ts: u64 = match parts[2].parse() {
        Ok(t) => t,
        Err(_) => return rejected("Invalid timestamp"),
    };
    let tier = parts[3].to_string();
    // Check expiry
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if now > expiry_ts {
        return LicenseInfo {
            name,
            email,
            verified: false,
            expires_at: "EXPIRED".into(),
            tier,
        };
    }
    // Verify signature
    let sig = match Signature::from_slice(sig_bytes) {
        Ok(s) => s,
        Err(_) => return rejected("Invalid signature bytes"),
    };
    let pk_bytes: [u8; 32] = match BASE64.decode(PUBLIC_KEY_BASE64) {
        Ok(b) if b.len() == 32 => {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&b);
            arr
        }
        _ => return rejected("Invalid public key"),
    };
    let pub_key = match VerifyingKey::from_bytes(&pk_bytes) {
        Ok(k) => k,
        Err(_) => return rejected("Invalid public key bytes"),
    };
    if pub_key.verify(payload, &sig).is_ok() {
        LicenseInfo {
            name,
            email,
            verified: true,
            expires_at: format_timestamp(expiry_ts),
            tier,
        }
    } else {
        rejected("Signature mismatch")
    }
}

fn rejected(reason: &str) -> LicenseInfo {
    LicenseInfo {
        name: String::new(),
        email: String::new(),
        verified: false,
        expires_at: reason.to_string(),
        tier: String::new(),
    }
}

fn format_timestamp(ts: u64) -> String {
    use chrono::{DateTime, NaiveDateTime, Utc};
    let naive = NaiveDateTime::from_timestamp_opt(ts as i64, 0).unwrap_or_default();
    let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
    dt.format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_key() {
        let info = verify_license("invalid_key_here");
        assert!(!info.verified);
    }

    #[test]
    fn test_empty_key() {
        let info = verify_license("");
        assert!(!info.verified);
    }
}
