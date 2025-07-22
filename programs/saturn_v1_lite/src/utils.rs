pub fn calculate_bond_quote(current_trading_price: u64, current_backing_price: u64) -> u64 {
    let difference = current_trading_price - current_backing_price;

    if difference > current_backing_price {
        return current_backing_price + difference / 100 * 30
    } else if difference < current_backing_price && difference > current_backing_price / 2 {
        return current_backing_price + difference / 100 * 50
    } else if difference < current_backing_price / 2 && difference > current_backing_price / 4 {
        return current_backing_price + difference / 100 * 65
    } else if difference < current_backing_price / 4 {
        return current_backing_price + difference / 100 * 80
    } else {
        return 0
    }
}

pub fn safe_divide(dividend: u64, divisor: u64) -> u64 {

    assert_ne!(
        divisor,
        0,
        "Attempt To Divide with Zero - Saturn Safe Divide"
    );
   

    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    // If remainder is more than half of divisor, round up
    if remainder >= divisor / 2 + divisor % 2 {
        quotient + 1
    } else {
        quotient
    }
}