use ethers::prelude::{Address, U256};
use ethers_core::abi;
use ethers_core::abi::ethabi::Bytes;
use ethers_core::abi::Token;
use ethers_core::utils::{get_create2_address, get_create2_address_from_hash, hex, keccak256};


fn main() {
    /// Test for TraderJoe pair USDC/wAVAX
    get_pool_pair_address_with_create2_and_manual_concat(
        "0bbca9af0511ad1a1da383135cf3a8d2ac620e549ef9f6ae3a4c33c2fed0af91",
        "0x9Ad6C38BE94206cA50bb0d90783181662f0Cfa10",
        "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7",
        "0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E",
        "0xf4003f4efbe8691b60249e6afbd307abe7758adb",
    );
}

fn get_pool_pair_address_with_create2_and_manual_concat(
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

    let salt = keccak256(&input);
    println!("-------\nFactory {:?} \nInput {:?}\nSalt {:?}\ninit_code {:?}", factory, input, salt, init_code_hash);

    let pool_address =
        get_create2_address_from_hash(factory, keccak256([token0.0, token1.0].concat()), init_code_hash);

    println!("POOL ADDRESS {:?} // {}", pool_address, expected_result);
}