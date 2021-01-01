pub trait Toggle {
    fn toggle(&mut self);
}

impl Toggle for bool {
        fn toggle(&mut self) {
                    *self = !*self;
        }
}

#[cfg(test)]
mod tests {
    use crate::Toggle;

    #[test]
    fn test_toogle() {
        let mut test = false;

        assert_eq!(test, false);
        test.toggle();

        assert_eq!(test, true);
        test.toggle();

        assert_eq!(test, false);
    }
}
