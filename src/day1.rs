pub fn part1(input: String) {
    let vec: Vec<u32> = input.trim().split('\n').map(|n| n.parse::<u32>().unwrap()).collect();
    let mut answers = vec![];

    for (pos, _) in vec.iter().enumerate() {
        if pos != 0 {
            if vec[pos] >= vec[pos - 1] {
                answers.push(true);
            }
        }
    };

    println!("{:?}", answers.len())
}
