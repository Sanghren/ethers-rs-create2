use ethers::prelude::{Address, U256};
use ethers_core::abi;
use ethers_core::abi::ethabi::Bytes;
use ethers_core::abi::Token;
use ethers_core::utils::{get_create2_address, get_create2_address_from_hash, hex, keccak256};


fn main() {
    /// Test for Uniswap pair USDC/wETH
    get_pool_pair_address_with_create2_with_fee(
        "e34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54",
        "0x1F98431c8aD98523631AE4a59f267346ea31F984",
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    );

    /// Test for TraderJoe pair USDC/wAVAX
    get_pool_pair_address_with_create2(
        "0bbca9af0511ad1a1da383135cf3a8d2ac620e549ef9f6ae3a4c33c2fed0af91",
        "0x9Ad6C38BE94206cA50bb0d90783181662f0Cfa10",
        "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7",
        "0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E",
        "0xf4003f4efbe8691b60249e6afbd307abe7758adb",
    );
}

fn get_pool_pair_address_with_create2_with_fee(
    init_code: &str,
    factory_address: &str,
    token0_address: &str,
    token1_address: &str,
    expected_result: &str) {
    let init_code_hash = Bytes::from(
        hex::decode(init_code).unwrap(),
    );
    let factory: Address = factory_address
        .parse()
        .unwrap();
    let token0: Address = token0_address
        .parse()
        .unwrap();
    let token1: Address = token1_address
        .parse()
        .unwrap();
    let fee = 500;

    let input = abi::encode(&vec![
        Token::Address(token0),
        Token::Address(token1),
        Token::Uint(U256::from(fee)),
    ]);

    let salt = keccak256(&input);
    println!("-------\nFactory {:?} \nInput {:?}\nSalt {:?}\ninit_code {:?}", factory, input, salt, hex::decode(init_code).unwrap());

    let pool_address =
        get_create2_address_from_hash(factory, salt.to_vec(), init_code_hash);

    println!("POOL ADDRESS {:?} // {:?}", pool_address, expected_result);
}

fn get_pool_pair_address_with_create2(
    init_code: &str,
    factory_address: &str,
    token0_address: &str,
    token1_address: &str,
    expected_result: &str) {
    let init_code_hash = Bytes::from(
        hex::decode(init_code).unwrap(),
    );
    let factory: Address = factory_address
        .parse()
        .unwrap();
    let token0: Address = token0_address
        .parse()
        .unwrap();
    let token1: Address = token1_address
        .parse()
        .unwrap();

    let input = abi::encode(&vec![
        Token::Address(token0),
        Token::Address(token1),
    ]);

    let salt = keccak256(&input);
    println!("-------\nFactory {:?} \nInput {:?}\nSalt {:?}\ninit_code {:?}", factory, input, salt, init_code_hash);

    let pool_address =
        get_create2_address_from_hash(factory, salt.to_vec(), init_code_hash);

    println!("POOL ADDRESS {:?} // {:?}", pool_address, expected_result);
}