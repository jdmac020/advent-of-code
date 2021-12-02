fn main() {
    
}

fn check_increase(first: i32, second: i32) -> i32 {
    if first < second {
        return 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_one_for_greater_second() {
        assert_eq!(1, check_increase(5, 10));
    }

    #[test]
    fn returns_zero_for_lesser_second() {
        assert_eq!(0, check_increase(10, 5));
    }
}