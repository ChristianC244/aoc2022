use eyre::Result;
use std::collections::{HashSet, HashMap};

fn main() -> Result<()>{

    let input = std::io::read_to_string(std::io::stdin())?;
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    Ok(())
}

fn part_one(input: &str) -> usize {

    let mut priority_sum = 0usize;
    let mut checker: HashSet<char> = HashSet::new();

    for rucksack in input.lines() {
        let len = rucksack.len();
        checker.clear();
        rucksack[..len/2].chars().for_each(| c | {checker.insert(c);});

        let c = rucksack[len/2..].chars().find(|c| checker.contains(c) ).unwrap() as u8;

        let prio = value_of(c);

        priority_sum += prio as usize;
    }

return     priority_sum
}

fn part_two(input: &str) -> usize {

    let m: [u8; 3] = [1, 2, 4];
    let mut i = 0usize;
    let mut checker: HashMap<char, u8> = HashMap::new();

    let mut priority = 0usize;
    for rucksack in input.lines() {

        
        rucksack.chars().for_each(|item| {
            match *checker.entry(item).or_insert(m[i]) & m[i] {
                0 => {*checker.get_mut(&item).unwrap() += m[i]},
                _ => ()
            };
        });
        
        if i == 2 {
            let c = checker.iter().max_by_key(|(_, val)| *val).unwrap().0;
            let c = value_of(*c as u8);
            priority += c as usize;
            checker.clear();
        }

        i = (i + 1) % 3;
        
    }
    priority
}

fn value_of(c: u8) -> u8 {
    
    if c < 'a' as u8 {
        //caps
        c as u8 - 38
    }else {
        c as u8 - 96
    }

}