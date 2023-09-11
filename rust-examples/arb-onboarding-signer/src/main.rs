use ethers_signers::{LocalWallet, Signer};
use ethers::utils::hex;
use web3::signing::keccak256;

#[tokio::main]
async fn main() {

    // for testnet this is the message to be signed, for mainnet use https://trade-arb.firefly.exchange
    let msg = "https://testnet.firefly.exchange";    

    // take hash of the message
    let hash = hex::encode(keccak256(msg.as_bytes()));
    println!("message hash: {}", hash);

    const WALLET_KEY: &str = "2ee813034aab842141cb85d477f7d0e359838f46fcab34a935c69410a4d39efb";

    let wallet = WALLET_KEY.parse::<LocalWallet>().unwrap();

    let signature = format!("0x{}", wallet.sign_message(hex::decode(&hash).unwrap().as_slice()).await.unwrap().to_string());

    // send this signature to /authorize route
    println!("onboarding signature: {}", signature);

}
