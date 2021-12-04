use itertools::izip;

pub fn get_depth_increases(numbers: Vec<i32>) -> usize {
    let output = numbers.iter().zip(numbers.iter().skip(1)).filter(|cur| check_increase(cur.0,cur.1));
    return output.count();
}

pub fn get_depth_increase_with_precision(readings: Vec<i32>) -> usize {
    let mut sums = Vec::<i32>::new();

    for (first, second, third) in izip!(readings.iter(), readings.iter().skip(1), readings.iter().skip(2)) {
        sums.push(first + second + third);
    };

    return get_depth_increases(sums);
}

fn check_increase(first: &i32, second: &i32) -> bool {
    return first < second;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_one_for_greater_second() {
        assert_eq!(true, check_increase(&5, &10));
    }

    #[test]
    fn returns_zero_for_lesser_second() {
        assert_eq!(false, check_increase(&10, &5));
    }

    #[test]
    fn returns_three_zip() {
        let numbers = vec![1,2,1,2,3];
        assert_eq!(3, get_depth_increases(numbers));
    }

    #[test]
    fn returns_seven_zip() {
        let numbers = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(7, get_depth_increases(numbers));
    }

    #[test]
    fn zip_three_vecs() {
        let numbers = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(5, get_depth_increase_with_precision(numbers));
    }
}