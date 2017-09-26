pub fn j() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    #[test]
    fn j() {
        assert!(super::j() == 42)
    }
}
