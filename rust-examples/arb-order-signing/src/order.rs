use ethers_signers::{LocalWallet, Signer};
use ethers::types::{U256, H160};
use ethabi::{Token};
use ethers::utils::hex;
use web3::signing::keccak256;


const EIP712_ORDER_STRUCT_STRING: &str = "Order(bytes8 flags,uint128 quantity,uint128 price,uint128 triggerPrice,uint128 leverage,address maker,uint128 expiration)";

const EIP712_CANCEL_ORDER_STRUCT_STRING: &str = "CancelLimitOrder(string action,bytes32[] orderHashes)";

const EIP712_DOMAIN_STRING: &str = "EIP712Domain(string name,string version,uint128 chainId,address verifyingContract)";

const EIP712_DOMAIN_NAME: &str = "IsolatedTrader";

const EIP712_DOMAIN_VERSION: &str = "1.0";

const EIP712_PREFIX: &str = "1901";

#[derive(Debug, Clone)]
pub struct Order {
    pub is_buy: bool,
    pub reduce_only: bool,
    pub quantity: String, 
    pub price: String,
    pub trigger_price: String, 
    pub leverage: String, 
    pub expiration: String, 
    pub salt: String,   
    pub maker: H160 
}

/**
 * Helper method to encode tokens and hash it
 */
pub fn encode_and_hash(tokens: &[Token]) -> String{

    // serialized encoded data
    let mut encoded_data = ethabi::encode(&tokens);

    // take keccak hash
    let hash = keccak256(&mut encoded_data[..]);
 
    return hex::encode(hash);
}

/**
 * Encodes order flags and returns a 16 bit hex
 */
fn get_order_flags(order:Order) -> String{

    let mut boolean_flag = 0;

    if order.is_buy {
        boolean_flag += 1;
    };

    if order.reduce_only {
        boolean_flag += 2;
    };

    let salt:u128 = order.salt.parse().unwrap();
    let flags = format!("{:0>15}{}", format!("{:x}", salt), boolean_flag);
    return flags;

}

pub fn get_order_data_hash(order:Order) -> String{

    // compute order flags
    let order_flags = get_order_flags(order.clone());

    let tokens = [
        Token::FixedBytes(Vec::from(keccak256(EIP712_ORDER_STRUCT_STRING.as_bytes()))),
        Token::FixedBytes(hex::decode(order_flags).unwrap()),
        Token::Uint(U256::from_dec_str(&order.quantity).unwrap()),
        Token::Uint(U256::from_dec_str(&order.price).unwrap()),
        Token::Uint(U256::from_dec_str(&order.trigger_price).unwrap()),
        Token::Uint(U256::from_dec_str(&order.leverage).unwrap()),
        Token::Address(order.maker),
        Token::Uint(U256::from_dec_str(&order.expiration).unwrap()),
        ];

    return encode_and_hash(&tokens);
}

/**
 * Given an order hash, encodes its data and computes its keckak hash just like solidity
 */
pub fn get_order_cancel_hash(order_hash: &str) -> String{
    
    let order_hash_sha3 = encode_and_hash(&[
        Token::FixedBytes(Vec::from(hex::decode(order_hash).unwrap()))
    ]);

    let tokens = [
        Token::FixedBytes(Vec::from(keccak256(EIP712_CANCEL_ORDER_STRUCT_STRING.as_bytes()))),
        Token::FixedBytes(Vec::from(keccak256(b"Cancel Orders"))),
        Token::FixedBytes(Vec::from(hex::decode(order_hash_sha3).unwrap())),
    ];

    return encode_and_hash(&tokens);
}


/**
 * Returns the EIP-712 style domain hash
 */
fn get_domain_separator_hash(trader_contract: &str, network_id: &str) -> String{
    
    let trader: H160 = trader_contract.parse().unwrap();

    let tokens = [
        Token::FixedBytes(Vec::from(keccak256(EIP712_DOMAIN_STRING.as_bytes()))),
        Token::FixedBytes(Vec::from(keccak256(EIP712_DOMAIN_NAME.as_bytes()))),
        Token::FixedBytes(Vec::from(keccak256(EIP712_DOMAIN_VERSION.as_bytes()))),
        Token::Uint(U256::from_dec_str(&network_id).unwrap()),
        Token::Address(trader),
     ];

     return encode_and_hash(&tokens);

}

fn get_eip_712_hash(domain_separator_hash: &str, data_hash: &str) -> String{

    let data = format!(
        "{}{}{}",
        EIP712_PREFIX,
        domain_separator_hash,
        data_hash
    );

    let msg_hash = keccak256(hex::decode(&data).unwrap().as_slice());

    return format!("{}", hex::encode(msg_hash));

}

/**
 * Given an order, trader contract address and network id, 
 * returns EIP 712 hash of the order
 */
pub fn get_hash(order:Order, trader_contract: &str, network_id: &str) -> String {

    let  order_data_hash = get_order_data_hash(order);
    let domain_hash = get_domain_separator_hash(trader_contract, network_id);
    let eip712_order_hash = get_eip_712_hash(&domain_hash, &order_data_hash);
    return eip712_order_hash;
}

/**
 * Given an order hash, trader contract address and network id, 
 * returns EIP 712 cancel hash of the order
 */
pub fn get_cancel_hash (order_hash: &str,trader_contract: &str, network_id: &str) -> String {
    let order_cancellation_hash = get_order_cancel_hash(order_hash);
    let domain_hash = get_domain_separator_hash(trader_contract, network_id);
    let eip712_cancel_order_hash = get_eip_712_hash(&domain_hash, &order_cancellation_hash);

    return eip712_cancel_order_hash;
}

pub async fn sign_order(wallet: LocalWallet, eip712_order_hash: &str) -> String{    
    let signature = wallet.sign_message(hex::decode(&eip712_order_hash).unwrap().as_slice()).await.unwrap().to_string();
    return format!("0x{}01", signature);
}