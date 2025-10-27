#[cfg(feature = "sum")]
pub fn sum(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "sub")]
pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(feature = "mul")]
pub fn mul(left: u64, right: u64) -> u64 {
    left * right
}

#[cfg(feature = "div")]
pub fn div(left: u64, right: u64) -> u64 {
    if right == 0 {
        panic!("Division by zero")
    } else {
        left / right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sum")]
    #[test]
    fn should_add_two_numbers() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }

    #[cfg(feature = "sub")]
    #[test]
    fn should_subtract_two_numbers() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }

    #[cfg(feature = "mul")]
    #[test]
    fn should_multiply_two_numbers() {
        let result = mul(2, 2);
        assert_eq!(result, 4);
    }

    #[cfg(feature = "div")]
    #[test]
    fn should_divide_two_numbers() {
        let result = div(2, 2);
        assert_eq!(result, 1);
    }

    #[cfg(feature = "div")]
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn should_panic_if_divided_by_zero() {
        let result = div(2, 0);
        assert_eq!(result, 1);
    }
}
