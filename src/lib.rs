#![no_std]

/// Trait to implement the `.toogle()` method, to switch between two values
///
/// ## Exemple
/// ```rust
/// use toogle::Toggle;
///
/// #[derive(PartialEq, Eq)]
/// pub enum TwoEnum {
///     Value1, Value2
/// };
///
/// impl Toggle for TwoEnum {
///     fn toggle(&mut self) {
///         *self = match *self {
///             TwoEnum::Value1 => TwoEnum::Value2,
///             TwoEnum::Value2 => TwoEnum::Value1,
///         }
///     }
/// }
///
/// ```
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
