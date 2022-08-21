# ethers-rs-create2

Tests around create2 and ethers.

## Issue with Trader Joe

Right now I am testing how to get the pair address for Trader Joe.

According
to [their sdk](https://github.com/traderjoe-xyz/joe-sdk/blob/9ea22ece0253e5769cd32c89f9ce9ab8540c4264/src/entities/pair.ts#L39)
, this are the parameters used in the `getCreate2` call:

```typescript
getCreate2Address(
    FACTORY_ADDRESS[chainId],
    keccak256(['bytes'], [pack(['address', 'address'], [tokens[0].address, tokens[1].address])]),
    init_code_hash[chainId]
)
```

where:

- [FACTORY_ADDRESS[chainId]](https://github.com/traderjoe-xyz/joe-sdk/blob/9ea22ece0253e5769cd32c89f9ce9ab8540c4264/src/constants.ts#L27)
  => `0x9Ad6C38BE94206cA50bb0d90783181662f0Cfa10`
- [init_code_hash[chainId]](https://github.com/traderjoe-xyz/joe-sdk/blob/9ea22ece0253e5769cd32c89f9ce9ab8540c4264/src/constants.ts#L157)
  => `0bbca9af0511ad1a1da383135cf3a8d2ac620e549ef9f6ae3a4c33c2fed0af91`

I quickly tested this function in typescript to see what would be the outcome, with the following token addresses:

- `0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7` (wavax)
- `0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E` (usdc)

Is returning the correct pair address `0xf4003F4efBE8691B60249E6afbD307aBE7758adb` .

Now I adapted the [example above](https://github.com/gakonst/ethers-rs/blob/49b4ac7acbbd71b4d10f7780c8d2efe9af27799b/ethers-core/src/utils/mod.rs#L251) `get_create2_address_from_hash` function in ethers.

It returns correct value for the example, generating the expected address on ethereum for the weth/usdc pool address.

But when I try to do the same for a pool on trader joe, it won't give me the expected result. I tried several things but none worked.

Am quite sure I did a woopsie somewhere, but I can't find it.