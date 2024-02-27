//borrowing is passing references as function parameters
fn main() {
    let s1 = String::from("hello!");
    let len = s1.len();
    
    println!("The length of {} is {}", s1, len);
}
