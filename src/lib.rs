use rust_decimal::prelude::*;
use rust_decimal_macros::*;

pub fn ontario_land_transfer_tax(purchase_price: Decimal) -> anyhow::Result<Decimal> {
    Ok(
        bracket(purchase_price, dec!(0.005), dec!(0.0), Some(dec!(55000)))?
            + bracket(
                purchase_price,
                dec!(0.010),
                dec!(55000.01),
                Some(dec!(250000)),
            )?
            + bracket(
                purchase_price,
                dec!(0.015),
                dec!(250000.01),
                Some(dec!(400000)),
            )?
            + bracket(
                purchase_price,
                dec!(0.020),
                dec!(400000.01),
                Some(dec!(2000000)),
            )?
            + bracket(purchase_price, dec!(0.025), dec!(2000000.01), None)?,
    )
}

pub fn toronto_land_transfer_tax(purchase_price: Decimal) -> anyhow::Result<Decimal> {
    Ok(
        bracket(purchase_price, dec!(0.005), dec!(0.0), Some(dec!(55000)))?
            + bracket(
                purchase_price,
                dec!(0.01),
                dec!(55000.01),
                Some(dec!(250000)),
            )?
            + bracket(
                purchase_price,
                dec!(0.015),
                dec!(250000.01),
                Some(dec!(400000)),
            )?
            + bracket(
                purchase_price,
                dec!(0.02),
                dec!(400000.01),
                Some(dec!(2000000)),
            )?
            + bracket(purchase_price, dec!(0.025), dec!(2000000.01), None)?,
    )
}

pub fn tax(purchase_price: Decimal) -> anyhow::Result<Decimal> {
    Ok(toronto_land_transfer_tax(purchase_price)? + ontario_land_transfer_tax(purchase_price)?)
}

fn bracket(
    purchase_price: Decimal,
    rate: Decimal,
    floor: Decimal,
    ceiling: Option<Decimal>,
) -> anyhow::Result<Decimal> {
    if purchase_price < floor {
        Ok(dec!(0))
    } else {
        match ceiling {
            Some(ceiling) => Ok((min(purchase_price, ceiling) - floor) * rate),
            None => Ok((purchase_price - floor) * rate),
        }
    }
}

fn min(a: Decimal, b: Decimal) -> Decimal {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ontario_tax() {
        assert_eq!(
            ontario_land_transfer_tax(dec!(500000)).unwrap(),
            dec!(6474.99955)
        );
    }
    #[test]
    fn toronto_tax() {
        assert_eq!(
            toronto_land_transfer_tax(dec!(500000)).unwrap(),
            dec!(6474.99955)
        );
    }
    #[test]
    fn total_tax() {
        assert_eq!(tax(dec!(500000)).unwrap(), dec!(12949.99910));
    }
}
