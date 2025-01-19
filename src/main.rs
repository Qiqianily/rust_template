fn main() {
    println!("Hello, world!");
    println!("Hello Rust!")
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }
}
