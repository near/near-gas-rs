# near_gas
near_gas is crate for work with gas data in near-protocol 
## near_gas examples 
```rust
use near-gas::{NearGas};

fn main() {
    
  let data = "12.657 TGas";
  
  let near_gas = NearGas::from_str(data);
  
  let gas = near_gas.from_gas();
  println!("{:?}", gas)
}
```



### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/Mr0melian/near_gas/blob/master/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/Mr0melian/near_gas/blob/master/LICENSE-APACHE
