fn parse_command(command: &str) -> (i32,i32) {
    let plane;
    let adjust;
    let parts: Vec<&str> = command.split(' ').collect();

    let direction = parts[0];
    let amount = parts[1].parse::<i32>().unwrap();

    if direction == "forward" {
        plane = 0;
        adjust = amount
    } else {
        plane = 1;

        if direction == "down" {
            adjust = amount
        } else {
            adjust = amount * -1;
        }
    }

    return (plane, adjust);
}

pub fn calculate_end_position(route: Vec<&str>) -> i32 {
    let mut x_pos : i32 = 0;
    let mut y_pos : i32 = 0;

    for entry in route {
        let command = parse_command(entry);
        if command.0 == 0 {
            x_pos = x_pos + command.1;
        } else {
            y_pos = y_pos + command.1;
        }
    }

    return x_pos * y_pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_returns_0_5_tuple() {
        assert_eq!((0, 5), parse_command("forward 5"));
    }

    #[test]
    fn parse_command_returns_1_10_tuple() {
        assert_eq!((1,10), parse_command("down 10"));
    }

    #[test]
    fn parse_command_returns_1_neg5_tuple() {
        assert_eq!((1,-5), parse_command("up 5"));
    }

    #[test]
    fn calculate_end_position_returns_15() {
        let route = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ];
        assert_eq!(150, calculate_end_position(route));
    }
}