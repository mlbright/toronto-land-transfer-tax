# toronto-land-transfer-tax

Calculate the [land transfer taxes in Toronto][ratehub].

**This calculator does not take into account first-time home buyer rebates.**

## Usage

```
use rust_decimal_macros::*;
fn main() {
    let purchase_price = dec!(2000000);
    let tax = toronto_land_transfer_tax::tax(purchase_price).unwrap();
    println!("Land transfer tax on a home bought for {}: {}", purchase_price, tax);
}
```


[ratehub]: https://www.ratehub.ca/land-transfer-tax-toronto
