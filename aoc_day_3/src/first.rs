pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let mut numbers: Vec<Vec<bool>> = vec![];    
    for line in lines{
        let test = line.chars().map(|x| x=='1').collect();
        numbers.push(test);   
    }

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    let number_length = numbers[0].len();

    for number in 0..number_length{
        let ones = numbers.iter().filter(|x| x[number]==true).count();
        if ones > numbers.len()/2{
            gamma_rate |= 1 << (number_length - number - 1);
        }
        else {
            epsilon_rate |= 1 << (number_length - number - 1);      
        }
    }

    res += gamma_rate * epsilon_rate;
    println!("Gamma: {gamma_rate}, Epsilon: {epsilon_rate}");
    println!("Part one result {}", res);
}