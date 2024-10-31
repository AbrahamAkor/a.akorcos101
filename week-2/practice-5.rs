struct Product {
    name: &'static str,
    quantity: u32,
    amount: f64,
}

fn main() {
    // List of products
    let products = [
        Product { name: "Toshiba", quantity: 2, amount: 450_000.0 },
        Product { name: "Mac", quantity: 1, amount: 1_500_000.0 },
        Product { name: "HP", quantity: 3, amount: 750_000.0 },
        Product { name: "Dell", quantity: 3, amount: 2_850_000.0 },
        Product { name: "Acer", quantity: 1, amount: 250_000.0 },
    ];

    // Calculate total sum of amounts
    let total_sum: f64 = products.iter().map(|product| product.amount).sum();

    // Calculate average amount
    let average: f64 = total_sum / products.len() as f64;

    // Display the results
    println!("\n==================== Sales Record ====================\n");
    println!("{:<5} {:<10} {:<10} {:<15}", "S/N", "Item", "Qty", "Amount (₦)");
    println!("-----------------------------------------------------");

    for (i, product) in products.iter().enumerate() {
        println!("{:<5} {:<10} {:<10} ₦{:<15.2}", i + 1, product.name, product.quantity, product.amount);
    }

    println!("-----------------------------------------------------");
    println!("Total Sum of Amounts: ₦{:.2}", total_sum);
    println!("Average Amount:       ₦{:.2}", average);
    println!("\n=====================================================");
}
