#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[should_panic]
    fn test_should_panic() {
        panic!("Make this test fail");
    }
}
