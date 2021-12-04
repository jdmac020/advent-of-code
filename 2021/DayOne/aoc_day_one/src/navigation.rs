const FORWARD_DIRECTION_STR: &str = "forward";
const FORWARD_DIRECTION: i32 = 0;
const DOWN_DIRECTION_STR: &str = "down";
const DOWN_DIRECTION: i32 = 1;
const UP_DIRECTION_STR: &str = "up";
const UP_DIRECTION: i32 = 2;

struct PositionChange {
    pub horizontal: i32,
    pub vertical: i32
}

struct Command {
    pub direction: i32,
    pub amount: i32
}

fn parse_command(command: &str) -> Command {
    let mut parsed_command = Command { direction: 0, amount: 0 };
    let parts: Vec<&str> = command.split(' ').collect();

    let direction = parts[0];
    let amount = parts[1].parse::<i32>().unwrap();
    match direction {
        FORWARD_DIRECTION_STR => {
            parsed_command.direction = FORWARD_DIRECTION;
            parsed_command.amount = amount;
        },
        DOWN_DIRECTION_STR => {
            parsed_command.direction = DOWN_DIRECTION;
            parsed_command.amount = amount;
        }, 
        UP_DIRECTION_STR => {
            parsed_command.direction = UP_DIRECTION;
            parsed_command.amount = amount * -1;
        }
        _ => println!("Might as well surface and ask for directions now!")
    }

    return parsed_command;
}

fn calculate_position_adjust(amount:i32, aim:i32) -> PositionChange {
    return PositionChange { horizontal: amount, vertical: amount * aim }
}

pub fn calculate_end_position(route: Vec<&str>) -> i32 {
    let mut x_pos : i32 = 0;
    let mut y_pos : i32 = 0;

    for entry in route {
        let command = parse_command(entry);
        if command.direction == 0 {
            x_pos = x_pos + command.amount;
        } else {
            y_pos = y_pos + command.amount;
        }
    }

    return x_pos * y_pos;
}

pub fn calculate_actual_end_position(route: Vec<&str>) -> i32 {
    let mut aim : i32 = 0;
    let mut x_pos : i32 = 0;
    let mut y_pos : i32 = 0;

    for entry in route {
        let command = parse_command(entry);

        if command.direction != FORWARD_DIRECTION {
            aim = aim + command.amount;
        }

        if command.direction == FORWARD_DIRECTION {
            let new_pos = calculate_position_adjust(command.amount, aim);
            x_pos = x_pos + new_pos.horizontal;
            y_pos = y_pos + new_pos.vertical;
        }
    }

    return x_pos * y_pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_returns_0_5_tuple() {
        let expected = Command {
            direction: FORWARD_DIRECTION,
            amount: 5
        };
        let result = parse_command("forward 5");
        assert_eq!(expected.direction, result.direction);
        assert_eq!(expected.amount, result.amount);
    }

    #[test]
    fn parse_command_returns_1_10_tuple() {
        let expected = Command {
            direction: DOWN_DIRECTION,
            amount: 10
        };
        let result = parse_command("down 10");
        assert_eq!(expected.direction, result.direction);
        assert_eq!(expected.amount, result.amount);
    }

    #[test]
    fn parse_command_returns_1_neg5_tuple() {
        let expected = Command {
            direction: UP_DIRECTION,
            amount: -5
        };
        let result = parse_command("up 5");
        assert_eq!(expected.direction, result.direction);
        assert_eq!(expected.amount, result.amount);
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

    #[test]
    fn calculate_actual_end_position_returns_900() {
        let route = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ];
        assert_eq!(900, calculate_actual_end_position(route));
    }

    #[test]
    fn calculate_position_adjust_returns_horizontal_5_for_forward_five_no_aim() {
        let expected = PositionChange { horizontal: 5, vertical: 0 };
        let result = calculate_position_adjust(5, 0);
        assert_eq!(expected.horizontal, result.horizontal);
        assert_eq!(expected.vertical, result.vertical);
    }
    

    #[test]
    fn calculate_position_adjust_returns_horizontal_5_vertical_25_for_aim_5() {
        let expected = PositionChange { horizontal: 5, vertical: 25 };
        let result = calculate_position_adjust(5, 5);
        assert_eq!(expected.horizontal, result.horizontal);
        assert_eq!(expected.vertical, result.vertical);
    }
}