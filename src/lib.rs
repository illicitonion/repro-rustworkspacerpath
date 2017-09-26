extern crate dep;

pub fn i() -> i32 {
    dep::j()
}

#[cfg(test)]
mod tests {
    #[test]
    fn i() {
        assert!(super::i() == 42)
    }
}
