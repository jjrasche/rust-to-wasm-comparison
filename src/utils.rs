pub mod my_nested_mod {
    /// ```rust
    /// assert_eq!(my_nested_mod::sum(10, 20), 30);
    /// ```
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
}