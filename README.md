# near-gas

<p>
    <a href="https://crates.io/crates/near-gas"><img src="https://img.shields.io/crates/d/near-gas?style=flat-square&logo=near&label=crates.io" alt="Crates.io (downloads)"></a>
    <a href="https://docs.rs/near-gas/latest/near_gas/"><img src="https://img.shields.io/docsrs/near-gas?style=flat-square" alt="Docs.rs"></a>
    <img src="https://img.shields.io/badge/rustc-1.68%2B-lightgray.svg?style=flat-square" alt="Rust Version">
</p>

near-gas is crate to ergonomically operate with NEAR Protocol gas unit in Rust projects.

The crate includes NearGas type and constructors for converting data as NearGas and as u64 type values.

## near-gas examples 

```rust
use near_gas::NearGas;

fn main() {
    let data = "12.657 tgas";

    let near_gas: NearGas = data.parse().unwrap();

    // Convert the value to the most precise "gas" unit
    assert_eq!(near_gas.as_gas(), 12657000000000);
    // Convert the value to "gigagas" unit
    assert_eq!(near_gas.as_ggas(), 12657);
    
    // Display Gas. It will print: "Here is 12.7 Tgas"
    println!("Here is {}", near_gas);

    // When `serde` feature is enabled, NearGas can be used in serde-serializable structs.
    // NearGas will be serialized to a gas-precision u64 value encoded as string.
    #[derive(serde::Serialize)]
    struct FunctionCallDetails {
        used_gas: NearGas,
    }

    let details = FunctionCallDetails { used_gas: near_gas };

    assert_eq!(serde_json::to_string(&details).unwrap(), r#"{"used_gas":"12657000000000"}"#);
}
```

## NearGas information

On every transaction you send to the network NEAR charges you a fee (aka gas fee). This fee is used to indirectly pay the people that keep the network infrastructure, and to incentivize developers of smart contracts. [For more information].

[Gas usage in Near Protocol]

## Crate Features

* `serde` - [serde](https://serde.rs/) support
* `borsh` - [borsh](https://github.com/near/borsh-rs) support
* `abi` - [near-abi](https://github.com/near/abi) support
* `schemars` - [schemars](https://github.com/GREsau/schemars) support
* `interactive-clap` - [interactive-clap](https://github.com/near-cli-rs/interactive-clap) support

### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/Mr0melian/near_gas/blob/master/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/Mr0melian/near_gas/blob/master/LICENSE-APACHE
[For more information]: https://docs.near.org/concepts/basics/transactions/gas
[Gas usage in Near Protocol]: https://nomicon.io/RuntimeSpec/Fees/
