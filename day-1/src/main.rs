use std::{error::Error, fs::File, io::{BufRead, BufReader}};

fn main() {
    const FILE_PATH: &str = "src/data/lists.txt";
    let list_result = get_lists(FILE_PATH);
    let (left_list, right_list) = match list_result {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error reading lists: {}", e);
            return;
        }
    };

    let total_diff = get_list_diff(&left_list, &right_list);
    let total_similarity = get_list_similarity(&left_list, &right_list);

    println!("Total difference between lists: {}", total_diff);
    println!("Total similarity between lists: {}", total_similarity);
}

/// Gets the two lists from the filepath specified. Sorts them, too.
fn get_lists(filepath: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    read_and_sort_lists(reader)
}

fn read_and_sort_lists(buf: BufReader<File>) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in buf.lines() {
        let line = line?;
        let mut split = line.split_whitespace();

        let left = split.next().ok_or("No left value found")?.parse::<u32>()?;
        let right = split.next().ok_or("No right value found")?.parse::<u32>()?;

        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    Ok((left_list, right_list))
}

fn get_list_diff(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    // the two arrays are sorted, and of the same length. so we can just iterate through it once, rather than
    // having to do a nested loop. this is O(n) rather than O(n^2). i think? could be wrong :^)
    let mut total_diff = 0;
    for i in 0..left_list.len() {
        let diff = (left_list[i] as i32 - right_list[i] as i32).abs();
        total_diff += diff;
    }

    total_diff as u32
}

fn get_list_similarity(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let mut total_similarity = 0;
    for left in left_list {
        let mut right_clones = 0;
        for right in right_list {
            if left == right {
                right_clones += 1;
            }
        }

        total_similarity += right_clones * left;
    }

    total_similarity
}
