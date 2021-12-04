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
}