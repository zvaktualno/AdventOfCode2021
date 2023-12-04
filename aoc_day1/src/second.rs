pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;

    let mut sums: Vec<u32> = vec![];

    let max_samples: usize = 3;

    // Create subsets of size max_samples
    for idx in 0..(&lines.len()-max_samples +1){
        let test: &std::slice::Iter<'_, String> = &lines[idx..idx+max_samples].iter();
        let sum: u32 = test.clone().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>().iter().sum();
        sums.push(sum);
    }

    // Find increased sizes of sums
    let mut prev_val: u32 = sums[0];
    for sum in &sums[1..sums.len()]{
        if sum > &prev_val {
            res += 1;
        }
        prev_val = *sum;
    }
    println!("Part two result {}", res);
}