extern crate hex;
extern crate ed25519_dalek;
extern crate base64;
extern crate reqwest;

use ed25519_dalek::*;
use std::collections::HashMap;
use blake2b_simd::Params;

#[tokio::main]
async fn main() {
    let wallet_key = "c501312ca9eb1aaac6344edbe160e41d3d8d79570e6440f2a84f7d9abf462270";

    // Decode Private Key Hex String to Bytes
    let bytes = hex::decode(wallet_key).expect("Decoding failed");
    let mut private_key_bytes: [u8; 32] = [0; 32];
    private_key_bytes.copy_from_slice(&bytes[0..32]);

    // Convert to Signing Key
    let signingkey = SigningKey::from_bytes(&private_key_bytes);

    // Generate the corresponding public key
    let public_key: VerifyingKey = (&signingkey).into();
    // println!("Public key bytes: {:?}", public_key.to_bytes());

    // Generate the b64 of the public key
    let public_key_b64 = base64::encode(&public_key.to_bytes());
    // println!("Public Key Base64: {}", public_key_b64);
    
    // Append 0x00 to public key due to BIP32
    let public_key_array = public_key.to_bytes();
    let mut public_key_array_bip32 = [0; 33];
    public_key_array_bip32[0] = 0;
    public_key_array_bip32[1..].copy_from_slice(&public_key_array);
    // println!("PUBLIC KEY ARRAY BIP32 {:?}", public_key_array_bip32);

    // Generate Wallet Address for BIP32 Public Key
    let hash = Params::new()
        .hash_length(32)
        .to_state()
        .update(&public_key_array_bip32)
        .finalize();
    let wallet_address = "0x".to_string() + &hash.to_hex().to_ascii_lowercase();
    println!("Wallet Address: {}", wallet_address);

    // Blake2B Hash Onboarding URL
    let mut msg_dict = HashMap::new();
    msg_dict.insert("onboardingUrl", "https://testnet.bluefin.io");

    let msg_str = serde_json::to_string(&msg_dict).unwrap();
    let mut intent: Vec<u8> = vec![3, 0, 0, msg_str.len() as u8];
    intent.extend_from_slice(msg_str.as_bytes());

    let hash = Params::new()
        .hash_length(32)
        .to_state()
        .update(&intent)
        .finalize();
    // println!("Onboarding URL Blake2B Hash: {}", hash.to_hex());

    // Sign the Hash and append "1"
    let onboarding_sig_temp  = signingkey.sign(&hash.as_bytes());
    let onboarding_sig = onboarding_sig_temp.to_string().to_ascii_lowercase() + "1";
    // println!("Signature: {}", onboarding_sig);

    // Combine Onboarding Signature and base64 of Public Key
    let onboarding_sig_full = onboarding_sig + &public_key_b64;
    // println!("Full Signature: {}", onboarding_sig_full);

    // POST Request and obtain JWT Token
    let mut body = HashMap::new();
    body.insert("signature", onboarding_sig_full);
    body.insert("userAddress", wallet_address);
    body.insert("isTermAccepted", "True".to_string());

    let client = reqwest::Client::new();
    let res = client.post("https://dapi.api.sui-staging.bluefin.io/authorize")
        .json(&body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("JWT TOKEN: {:?}", res)
}
