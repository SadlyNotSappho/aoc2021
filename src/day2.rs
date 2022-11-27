pub fn part1(input: String) {
    let input_vec: Vec<String> = input.trim().split('\n').map(|i| i.to_string()).collect();
    let parsed_vec: Vec<SubmarineCommands> = input_vec.iter().map(|i| {
        let instructions: Vec<&str> = i.split(" ").into_iter().collect();
    
        let dir = match instructions[0] {
            "forward" => SubmarineDirection::Forward,
            "down" => SubmarineDirection::Down,
            "up" => SubmarineDirection::Up,
            _ => panic!("Couldn't parse direction")
        };
        let amount: i32 = instructions[1].parse().unwrap();

        SubmarineCommands {
            direction: dir,
            amount
        }
    }).collect();

    let mut depth = 0;
    let mut horizontal = 0;

    for i in parsed_vec {
        if i.direction == SubmarineDirection::Forward {
            horizontal += i.amount;
        } else if i.direction == SubmarineDirection::Down {
            depth += i.amount
        } else {
            depth -= i.amount
        }
    }

    println!("{}", horizontal * depth);
}

pub struct SubmarineCommands {
    pub direction: SubmarineDirection,
    pub amount: i32
}

#[derive(PartialEq)]
pub enum SubmarineDirection {
    Forward,
    Down,
    Up,
}
