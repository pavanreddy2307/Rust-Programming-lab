
fn main() {
    
    let a: i32 = 2;
    let b: i32 = 3;

    let sum = a * a * a + b * b * b + 3 * a * a * b + 3 * a * b * b;

    println!("The result of (a + b)^3 using expansion is: {}", sum);
}
