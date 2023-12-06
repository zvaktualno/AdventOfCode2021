fn determine_major_value(nums: &Vec<&Vec<bool>>, pos: usize) -> bool {
    let ones: usize = nums.iter().filter(|x| x[pos]==true).count();
    let zeros: usize = nums.len() - ones;

    return ones >= zeros;
}

fn vector_to_int(num: &Vec<bool>) -> u64 {
    let mut res: u64 = 0;
    for (idx, n) in num.iter().rev().enumerate() {
        if *n{
            res |= 1 << idx; 
        }        
    }
    return res;
}

pub fn solve(lines: &Vec<String>) {
    // Parse all lines, creating vector of bools for each number
    let mut numbers: Vec<Vec<bool>> = vec![];    
    for line in lines{
        let test = line.chars().map(|x| x=='1').collect();
        numbers.push(test);   
    }
    let number_length: usize = numbers[0].len();


    let mut largest_numbers: Vec<&Vec<bool>> = numbers.iter().collect();
    let mut smallest_numbers: Vec<&Vec<bool>> = numbers.iter().collect();

    for pos in 0..number_length{
        let major_value = determine_major_value(&largest_numbers, pos);
        let minor_value = !determine_major_value(&smallest_numbers, pos);

        if largest_numbers.len() > 1 {
            largest_numbers = largest_numbers.into_iter().filter(|x: &&Vec<bool>| x[pos] == major_value).collect();
        }
        if smallest_numbers.len() > 1 {
            smallest_numbers = smallest_numbers.into_iter().filter(|x: &&Vec<bool>| x[pos] == minor_value).collect();
        }
    }


    let oxygen_generator_rating = vector_to_int(largest_numbers[0]);
    let co2_scrubber_rating = vector_to_int(smallest_numbers[0]);
 
    println!("Part two result {}", oxygen_generator_rating * co2_scrubber_rating);
}