mod set1;

mod chap0 {
    pub fn foo() -> u32 {
        666
    }
}

#[cfg(test)]
mod test {
    use chap0;

    #[test]
    fn foo_test() {
        assert!(chap0::foo() == 666)
    }
}
