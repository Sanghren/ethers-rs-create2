use ethers::prelude::{Address, U256};
use ethers_core::abi;
use ethers_core::abi::ethabi::Bytes;
use ethers_core::abi::Token;
use ethers_core::utils::{get_create2_address, get_create2_address_from_hash, hex, keccak256};



fn main() {
    /// Test for Uniswap pair USDC/wETH
    get_pool_pair_address_with_create2(
        "e34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54",
        "0x1F98431c8aD98523631AE4a59f267346ea31F984",
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
    );

    /// Test for TraderJoe pair USDC/wAVAX
    get_pool_pair_address_with_create2(
        "0bbca9af0511ad1a1da383135cf3a8d2ac620e549ef9f6ae3a4c33c2fed0af91",
        "0x9Ad6C38BE94206cA50bb0d90783181662f0Cfa10",
        "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7",
        "0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e",
        "0xf4003f4efbe8691b60249e6afbd307abe7758adb"
    );
}

fn get_pool_pair_address_with_create2(
    init_code: &str,
    factory_address: &str,
    token0_address: &str,
    token1_address: &str,
    expected_result: &str) {
    let UNISWAP_V3_POOL_INIT_CODE_HASH = Bytes::from(
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
    let pool_address =
        get_create2_address_from_hash(factory, salt.to_vec(), UNISWAP_V3_POOL_INIT_CODE_HASH);

    assert_eq!(
        pool_address,
        expected_result
            .parse()
            .unwrap()
    );
    println!("POOL ADDRESS {:?}", pool_address);
}

fn get_pool_pair_address_with_create2_hash(
    init_code: &str,
    factory_address: &str,
    token0_address: &str,
    token1_address: &str,
    expected_result: &str) {
    let UNISWAP_V3_POOL_INIT_CODE_HASH = Bytes::from(
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
        Token::Address(token1)
    ]);

    let salt = keccak256(&input);
    let pool_address =
        get_create2_address(factory, salt.to_vec(), Bytes::from(init_code));

    assert_eq!(
        pool_address,
        expected_result
            .parse()
            .unwrap()
    );
    println!("POOL ADDRESS {:?}", pool_address);
}