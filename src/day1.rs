pub fn part1(input: String) {
    let vec: Vec<u32> = input.trim().split('\n').map(|n| n.parse::<u32>().unwrap()).collect();
    let mut answers = vec![];

    for (pos, _) in vec.iter().enumerate() {
        if pos != 0 {
            if vec[pos] > vec[pos - 1] {
                answers.push(true);
            }
        }
    };

    println!("{:?}", answers.len())
}

pub fn part2(input: String) {
    let starting_vec: Vec<u32> = input.trim().split('\n').map(|n| n.parse::<u32>().unwrap()).collect();
    let mut vec: Vec<u32> = vec![];

    for (pos, _) in starting_vec.iter().enumerate() {
        let p2 = pos + 3;
        if p2 <=starting_vec.len() {
            let n1 = starting_vec[pos];
            let n2 = starting_vec[pos + 1];
            let n3 = starting_vec[pos + 2];
            // vec.push(format!("{} + {} + {} = {}", n1, n2, n3, n1+n2+n3));
            vec.push(n1+n2+n3)
        }
    }

    // for l in vec.clone() {
    //     println!("{}", l);
    // }

    let mut answers = vec![];

    for (pos, _) in vec.iter().enumerate() {
        if pos != 0 {
            if vec[pos] > vec[pos - 1] {
                answers.push(true);
            } // else {
            //     answers.push(false);
            // }
        }
    };

    println!("{:?}", answers.len());
    // for a in answers {
    //     println!("{}", a);
    // }
}
