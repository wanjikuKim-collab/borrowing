//borrowing is passing references as function parameters
fn main() {
    let s1 = String::from("hello!");
    let len = calculat_length(&s1);
    
    println!("The length of {} is {}", s1, len);
}

fn calculat_length(s: &String) -> usize {
    let length = s.len();
    length
}
