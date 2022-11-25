pub fn part1(input: String) {
    let vec: Vec<u32> = input.trim().split('\n').map(|n| n.parse::<u32>().unwrap()).collect();
    let mut answers: Vec<DayOneDepth> = vec![];
    let mut answer_vec = vec![];

    for (pos, _) in vec.iter().enumerate() {
        if pos != 0 {
            if vec[pos] >= vec[pos - 1] {
                answers.push(DayOneDepth::Deeper);
            } else if vec[pos] == vec[pos - 1] {
                answers.push(DayOneDepth::Same)
            } else {
                answers.push(DayOneDepth::Shallower)
            }
        }
    };

    for a in answers {
        if a == DayOneDepth::Deeper {
            answer_vec.push(true);
        }
    };

    println!("{:?}", answer_vec.len())
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum DayOneDepth {
    Deeper,
    Shallower,
    Same
}
