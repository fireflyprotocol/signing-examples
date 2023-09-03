use ethers_signers::{LocalWallet, Signer};
use web3_unit_converter::Unit;

// module containing order struct and signing functionality
mod order;

#[tokio::main]
async fn main() {

    const WALLET_KEY: &str = "2ee813034aab842141cb85d477f7d0e359838f46fcab34a935c69410a4d39efb";

    // for each market there is a different trader contract
    const TRADER_CONTRACT: &str = "0x934Dd6503795ef6EE6a36e3b3f1d7Be6c7096955"; 

    // chain id for testnet its 421613, for mainnet its 42161
    const NETWORK_ID: &str = "421613";


    let wallet = WALLET_KEY.parse::<LocalWallet>().unwrap();
    let address = wallet.address();
    println!("Wallet address: {:?}\n", address);

    // create an order for signing
    let order = order::Order {
        is_buy: true,
        reduce_only: false,
        price: Unit::Ether(&"1601").to_wei_str().unwrap(), // in 1e18 format
        quantity: Unit::Ether(&"0.01").to_wei_str().unwrap(), // in 1e18 format
        leverage: Unit::Ether(&"20").to_wei_str().unwrap(), // in 1e18 format
        trigger_price:  Unit::Ether(&"0").to_wei_str().unwrap(), // in 1e18 format, always zero
        salt:"169332763775317".to_string(),
        expiration:"1696006037".to_string(),
        maker: address
    };

    println!("{:?}\n", order);

    let order_hash = order::get_hash(order, TRADER_CONTRACT, NETWORK_ID);
    println!("Order hash: 0x{}\n", order_hash);
    // 0xf72743780d4d05dab491a370eb280a511b96e4c68234f4e6f5f9d2fe48645df5
    // 0xf72743780d4d05dab491a370eb280a511b96e4c68234f4e6f5f9d2fe48645df5

    let signature =  order::sign_order(wallet, &order_hash).await;
    println!("signature: {}\n", signature);
    // 0xb2b516102d7c0061990354861859cc33cd8380f5e72d15890ad86639c43ef30026cb2de176a3d51f290b4fe99e0a1c66cdf34a479ca0015494d62378d08a56d91b01
    // 0xb2b516102d7c0061990354861859cc33cd8380f5e72d15890ad86639c43ef30026cb2de176a3d51f290b4fe99e0a1c66cdf34a479ca0015494d62378d08a56d91b01



    // adding 0x to the hash - I believe "0x" is hashed as well
    let order_hash_0x = "0x".to_string() + &order_hash;

    let cancel_order_hash = order::get_cancel_hash(&order_hash_0x, TRADER_CONTRACT, NETWORK_ID);
    println!("Cancel Order hash: 0x{}\n", cancel_order_hash);

    // sign cancellation hash
    let wallet = WALLET_KEY.parse::<LocalWallet>().unwrap();
    let cancel_signature =  order::sign_order(wallet, &cancel_order_hash).await;
    println!("Cancel signature: {}\n", cancel_signature);

}
