use eyre::Result;
use std::collections::HashMap;

#[derive(Debug)]
enum Filetype {
    Dir, 
    File
}
type FileInfo = (usize, Filetype);

fn main() -> Result<()>{

    let input = std::io::read_to_string(std::io::stdin())?;
    let mut size = 0;
    let mut wd: Vec<(&str, usize)> = vec![];
    let mut filesystem: HashMap<String, HashMap<String, FileInfo>> = HashMap::new();

    input.lines().for_each(|line|{
        let args = line.split(" ").collect::<Vec<&str>>();
        match (args[0], args[1]) {
            ("$", "ls") => (),
            // Changing directory
            ("$", "cd") => {
                match args[2] {
                    // Exiting dir
                    ".." => {
                        let (dir_name, dir_size) = wd.pop().unwrap();
                        let curr_wd = wd_absolute_path(&wd);
                        let dir_info: FileInfo = (size, Filetype::Dir); 

                        filesystem.get_mut(&curr_wd)
                            .unwrap()
                            .insert(dir_name.to_string(), dir_info);

                        size += dir_size;
                    },
                    // Entering dir
                    dir_name => {
                        wd.push((dir_name, size));
                        size = 0;
                    }
                };
            },
            // Information af file structure
            // directory
            ("dir", dir_name) => {
                let curr_wd = wd_absolute_path(&wd);
                let dir_info: FileInfo = (0, Filetype::Dir); 

                filesystem.entry(curr_wd)
                    .or_insert(HashMap::new())
                    .insert(dir_name.to_string(), dir_info);
            },
            // file
            (weight, file_name) => {
                let weight = weight.parse::<usize>().unwrap();
                let curr_wd = wd_absolute_path(&wd);
                let file_info: FileInfo = (weight, Filetype::File);

                filesystem.entry(curr_wd)
                    .or_insert(HashMap::new())
                    .insert(file_name.to_string(), file_info);
                
                size += weight;
            }
        }
    });
    // Backtracking the stack
    while let Some((dir_name, dir_size)) = wd.pop() {
        if dir_name == "/" { break; }
        let curr_wd = wd_absolute_path(&wd);
        let dir_info: FileInfo = (size, Filetype::Dir);

        filesystem.get_mut(&curr_wd)
            .unwrap()
            .insert(dir_name.to_string(), dir_info);

        size += dir_size;
    }
    
    // END Input parsing
    let res = filesystem.iter()
        .map(|(_, content)| {
        content.iter()
            .filter(|(_, f_info)| matches!(f_info.1 ,Filetype::Dir))
            .map(|(_, f_info)| f_info.0)
            .collect::<Vec<usize>>()
        }).flatten()
        .fold(0usize, |acc, weight| {
            if weight <= 100_000 {acc + weight}
            else {acc}
        });

    println!("part one: {}", res);
    
    // PART TWO
    let used_space: usize = filesystem.get("/").unwrap().iter().map(|(_, (size, _))| size).sum();
    let free_space = 70_000_000 - used_space;

    let res = filesystem.iter()
        .map(|(_, content)| {
        content.iter()
            .filter(|(_, f_info)| matches!(f_info.1 ,Filetype::Dir))
            .map(|(_, f_info)| f_info.0)
            .collect::<Vec<usize>>()
        }).flatten()
        .filter(|weight| {
            free_space + weight >= 30_000_000
        })
        .min()
        .unwrap();
    
        println!("part two: {}", res);
    Ok(())
}

fn wd_absolute_path(wd: &Vec<(&str, usize)>) -> String {
    if wd.len() == 1 { return "/".to_string() }
    wd.iter().skip(1).map(|(name, _)| "/".to_string() + name).collect::<String>()
}