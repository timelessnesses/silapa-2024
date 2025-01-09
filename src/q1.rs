// basic shop
// price
// discount
// discounted
// net price
// paid
// change

use crate::input::input;

pub fn q1() {
    println!("Input:");
    let price: f64 = input(Some("Price: "));
    let discount: f64 = input(Some("Discount: "));
    let paid: f64 = input(Some("Paid: "));

    let discounted = price * (discount / 100.0);
    let net_price = price - discounted;
    let change = paid - net_price;
    println!();
    println!("Output:");
    println!("Price: {:.2}", price);
    println!("Discount: {:.2}", discount);
    println!("Discounted: {:.2}", discounted);
    println!("Net Price: {:.2}", net_price);
    println!("Paid: {:.2}", paid);
    println!("Change: {:.2}", change);
}
