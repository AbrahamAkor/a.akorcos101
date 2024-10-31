fn main() {
    let rate:f64 = 10.00;// Interest rate per annum
    let principal:f64 = 520_000_000.00;// Principal amount in naira
    let time:f64 = 5.00; // Time in years

    let compound_amount:f64 =principal * (1.00+rate/100.00).powf(time);
    let compound_interest:f64 = compound_amount - principal;
    println!("Compound Interest adter {} years; ₦{:.2}", time, compound_interest);
    println!("Total Amount (A) after {} year: ₦{:.2}", time, compound_amount);



}