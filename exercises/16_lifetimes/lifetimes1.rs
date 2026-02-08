// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// TODO: Fix the compiler error by updating the function signature.
// The right return
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Expiriment with the wrong return type to see the compiler respone.
// fn longest(x: String, y: String) -> String {
//     if x.len() > y.len() { x } else { y }
// }

fn main() {
    // You can optionally experiment here.
    let s1 = String::from("abcd");
    let s2 = String::from("123");
    let s3 = String::from("jlfkjsdlkjflks");
    let s4 = String::from("jlfkjsdlk");
    let s5 = "hello";

    println!("s1: {s1}, s2: {s2}, s3: {s3}, s4: {s4}");
    print!("{s5:}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("hel", "abcd"), "abcd");
        assert_eq!(longest("min", "jlfkjsdlkjflks"), "jlfkjsdlkjflks");
    }
}
