<p>
    <a href="https://crates.io/crates/near-gas"><img src="https://img.shields.io/crates/dv/near-gas?style=flat-square&logo=near&label=crates.io" alt="Crates.io (latest)"></a>
    <a href="https://docs.rs/near-gas/latest/near_gas/"><img src="https://img.shields.io/docsrs/near-gas?style=flat-square" alt="Docs.rs"></a>
    <img src="https://img.shields.io/badge/rustc-1.68%2B-lightgray.svg?style=flat-square" alt="Rust Version">
</p>

# near-gas
near-gas is crate for work with gas data in near-protocol.

The crate includes NearGas type and constructors for converting data as NearGas and as u64 type values.

## near-gas examples 
```rust
use near_gas::{NearGas};

fn main() {
    
  let data = "12.657 TGas";
  
  let near_gas = NearGas::from_str(data);
  
  let gas = near_gas.as_gas();
  assert_eq!(gas, 12657000000000);

  let giga_gas = near_gas.as_ggas();
  assert_eq!(giga_gas, 12657);
}
```
## NearGas information
On every transaction you send to the network NEAR charges you a fee (aka gas fee). This fee is used to indirectly pay the people that keep the network infrastructure, and to incentivize developers of smart contracts. [For more information].

[Gas usage in Near Protocol]
 



### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/Mr0melian/near_gas/blob/master/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/Mr0melian/near_gas/blob/master/LICENSE-APACHE
[For more information]: https://docs.near.org/concepts/basics/transactions/gas
[Gas usege in Near Protocol]: https://nomicon.io/RuntimeSpec/Fees/
