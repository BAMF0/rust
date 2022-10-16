use std::collections::HashMap;

fn main() {
    let list = vec![2,3,4,4,3,1,5,6,2];

    let median = median(&list);
    println!("The median of {:?} is {}", list, median);

    let mode = mode(&list);
    println!("The mode of {:?} is {}", list, mode);
    
    let median_mode = median_mode(&list);
    println!("The median and mode of {:?} is {:?}", list, median_mode);
}

// Given a list of integers returns the median and
// mode as a tuple pair
fn median_mode(list: &Vec<i32>) -> (i32, i32){ 
    (median(list),mode(list))
}

// Given a vector of integers, returns the median
fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let median: Option<&i32> = sorted_list.get(list.len()/2);
    match median {
        Some(median) => *median,
        None => 0,
    }
}

// Given a vector of integers, returns the
// value with the highest number of occurrences
fn mode(list: &Vec<i32>) -> i32 {
    let mut occurrence_map: HashMap<i32, i32> = HashMap::new();

    for n in list {
        let count = occurrence_map.entry(*n).or_insert(0);
        *count += 1;
    }
    let mut highest_key = 0;
    let mut highest_occurrence = 0;

    for (k, v) in occurrence_map.iter() {
        if v > &highest_occurrence {
            highest_occurrence = *v;
            highest_key = *k;
        }
    }
    highest_key
}
