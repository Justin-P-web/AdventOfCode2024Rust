use std::io::{stdout, stdin, Write};

fn main() -> std::io::Result<()> {
    let mut s=String::new();
    print!("Enter filepath: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let contents: Vec<String> = std::fs::read_to_string(s)
    .expect("Failed to read input")
    .split("\n")
    .map(|line| line.to_string())
    .collect();
    let mut leftCol = Vec::new();
    let mut rightCol = Vec::new();
    for index in 0..contents.len() {
        if contents[index].len() == 0{
            continue;
        }
        let v : Vec::<&str> = contents[index].split_whitespace().collect();
        leftCol.push(v[0].trim().to_owned());
        rightCol.push(v[1].trim().to_owned());
    }
    leftCol.sort();
    rightCol.sort();
    let mut sum = 0;
    for index in 0..leftCol.len() {
        let left = leftCol[index].parse::<i32>();
        let right = rightCol[index].parse::<i32>();
        if left.is_err() || right.is_err(){
            continue;
        }
        sum += (left.unwrap() - right.unwrap()).abs()
    }
    println!("{}", sum);
    Ok(())
}