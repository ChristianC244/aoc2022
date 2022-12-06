use eyre::Result;
use std::cmp::Reverse;

fn main() -> Result<()>{

    let input = std::io::read_to_string(std::io::stdin())?;

    let input = input.lines()
        .map(|x| {
            match x {
                "" => 0,
                v => v.parse::<usize>().unwrap()
            }
        })
        .collect::<Vec<usize>>();
    
    println!("Max calories carried: {}", find_max_sum(&input));
    let step_two = find_top_three(&input);
    println!("Top three calories carried: {:?} -> sum: {}", step_two, step_two.iter().sum::<usize>());

    Ok(())
}
// Part one
fn find_max_sum(input: &Vec<usize>) -> usize{
    let mut c = 0usize;
    let mut max = 0usize;
    input.iter().for_each(|x| {
        match x {
            0 => {
                max = std::cmp::max(max,c);
                c = 0;
            }
            _ => c += x
        }
    });

    max  
}
// Part two
fn find_top_three(input: &Vec<usize>) -> Vec<usize> {
    let mut c = 0usize;
    let mut max: Vec<usize> = Vec::new();
    
    input.iter().for_each(|x| {
        match x {
            0 => {
                max.push(c);
                c = 0;
            }
            _ => c += x
        }
    });
    
    max.sort_by_key(|w| Reverse(*w));
    max[..3].to_vec()
}
