use eyre::Result;
use regex::Regex;

fn main() -> Result<()>{

    // Interpreting ship's layout
    let re = Regex::new(r"[\[| ](\D)[\]| ] ?")?;
    let input = std::io::read_to_string(std::io::stdin())?;

    // the ship is made of multiple sections (stacks) where the cargo is deposited
    let num_stacks = (input.lines().nth(0).unwrap().len() + 1) / 4;
    let mut ship: Vec<Vec<char>> = vec![vec![]; num_stacks];
    let mut index = 0usize; // This counter is used to divide the input file from SHIP LAYOUT and COMMANDS
    
    for x in input.lines() {
        if !re.is_match(x) {break;};
        for (cap, stack) in re.captures_iter(x).zip(&mut ship){
            let cargo = cap[1].chars().nth(0).unwrap(); 
            if cargo == ' ' {continue;}
            stack.push(cargo);
        };
        index += 1;
    };
    index += 1; // counting the empty line
    
    // preparing ships for both parts
    let mut tmp = ship.clone();
    let mut ship_one: Vec<&mut Vec<char>> = tmp.iter_mut().map(|v| {v.reverse(); v}).collect();
    let mut ship_two: Vec<&mut Vec<char>> = ship.iter_mut().map(|v| {v.reverse(); v}).collect();

    // ____________________________________ PART ONE
    // Interpretation and execution of the commands
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    input.lines().skip(index).for_each(|cmd| {
        // reading commands
        let cap = re.captures(cmd).unwrap();
        let crates = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        // moving part
        for _ in 0..crates {
            let tmp = ship_one[from].pop().unwrap();
            ship_one[to].push(tmp);
        }
    });

    let res: String = ship_one.iter().fold("".to_string(), |acc, stack| {
        format!("{}{}",acc, stack.last().unwrap())
    });
    println!("part one: {}", res);

    // ____________________________________ PART TWO

    input.lines().skip(index).for_each(|cmd| {
        // reading commands
        let cap = re.captures(cmd).unwrap();
        let crates = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        // moving part
        let len = ship_two[from].len();
        let mut tmp = ship_two[from].drain((len - crates)..).collect::<Vec<char>>();
        ship_two[to].append(&mut tmp);
    });

    let res: String = ship_two.iter().fold("".to_string(), |acc, stack| {
        format!("{}{}",acc, stack.last().unwrap())
    });
    println!("part two: {}", res);
    Ok(())
}