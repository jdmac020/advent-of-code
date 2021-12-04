mod data;

fn main() {
    println!("Checking sonar pings... \n...\n...\n...\n");
    let readings = data::get_sonar_readings();
    let readings_count = readings.iter().count();
    let increases = check_vector_via_zip(readings);
    println!("\tNumber of readings: {}", readings_count);
    println!("\tNumber of increases: {}", increases);
}

fn check_increase(first: &i32, second: &i32) -> bool {
    return first < second;
}

fn check_vector_via_zip(numbers: Vec<i32>) -> usize {
    let output = numbers.iter().zip(numbers.iter().skip(1)).filter(|cur| check_increase(cur.0,cur.1));
    return output.count();
}

fn check_vector_via_window(numbers: Vec<i32>) -> usize {
    let items = numbers.windows(2).filter(|w| check_increase(&w[0], &w[1])).count();
    return items;
}

fn bump_vec(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = numbers.clone();
    result.insert(0,0);
    return result;
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
        assert_eq!(3, check_vector_via_zip(numbers));
    }

    #[test]
    fn returns_three_window() {
        let numbers = vec![1,2,1,2,3];
        assert_eq!(3, check_vector_via_window(numbers));
    }

    #[test]
    fn bump_adds_zero() {
        let numbers = vec![1,2,1,2,3];
        assert_eq!(vec![0,1,2,1,2,3], bump_vec(numbers));
    }
    
    #[test]
    fn returns_seven_zip() {
        let numbers = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(7, check_vector_via_zip(numbers));
    }

    
    #[test]
    fn returns_seven_window() {
        let numbers = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(7, check_vector_via_window(numbers));
    }
}