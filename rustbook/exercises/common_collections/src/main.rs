use std::collections::HashMap;
use std::io;

fn main() {
    let list = vec![2,3,4,4,3,1,5,6,2];

     let median = median(&list);
     println!("The median of {:?} is {}", list, median);

     let mode = mode(&list);
     println!("The mode of {:?} is {}", list, mode);
  
     let median_mode = median_mode(&list);
     println!("The median and mode of {:?} is {:?}", list, median_mode);

    let s1 = "first";
    let s2 = "apple";
    let s3 = "if apples can be oranges";
    println!("Pig latin of \"{}\" is \"{}\"", s1, to_pig_latin(s1));
    println!("Pig latin of \"{}\" is \"{}\"", s2, to_pig_latin(s2));
    println!("Pig latin of \"{}\" is \"{}\"", s3, to_pig_latin(s3));

    let s1 = "Add Sally to Engineering";
    let s2 = "Pizza";
    println!("{:?}", parse_add(s1));
    println!("{:?}", parse_add(s2));
    
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("Sally"), String::from("Engineering"));
    let parse = parse_get("Engineering", &map);
    println!("{:?}", parse);

    add_to_company_interface();
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

// Given a string, return the string converted to pig latin.
// Pig latin:
// - The first consonant of each word is moved to the end 
// of the word is moved to the end of the word and "ay" ias added,
// so "first" becomes "irst-fay".
//
// - Words that start with a vowel have "hay" added to the end instead
// ("apple" becomes "apple-hay")
fn to_pig_latin(str: &str) -> String {
    let mut pig_latin = String::new();

    for s in str.split_whitespace() {
        let pig_str = pig_latinize(s);
        pig_latin.push_str(&pig_str); 
        pig_latin.push_str(" ");
    }

    pig_latin
}

// Given a string, return the pig latin of the based on the 
// first character of the string.
// NOTE: works only as intended on single word strings
fn pig_latinize(str: &str) -> String {
    let first_char = str.chars().nth(0);

    match first_char {
        Some(char) => 
            if is_vowel(&char) {
                format!("{}{}", str, "hay") 
            } else {
                format!("{}{}{}", &str[1..], char, "ay")
            },
        None => String::from("") 
    }
}

fn is_vowel(c: &char) -> bool {
    const VOWELS: [char; 6] = ['a','e','i','o','u','y'];  
    
    let lower_c = c.to_ascii_lowercase();
    VOWELS.contains(&lower_c)
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn add_to_company_interface() {
    let mut company_map: HashMap<String, String> = HashMap::new();

    loop {
        println!("Input a request. It can be of the form: ");
        println!("'Add <name> to <department>' or 'Get <department>'\n");
        let mut request = String::new();

        io::stdin()
            .read_line(&mut request)
            .expect("Failed to read line");

            match parse_get(&request, &company_map) {
                Some(v) => println!("{:?}", v),
                None => { 
                    match parse_add(&request) {
                        Some((n,d)) => {
                            company_map.entry(String::from(n)).or_insert(String::from(d));
                            println!("{} was added to {}", n, d);
                        },
                        None => {
                            println!("Incorrect format. Try using format:");
                            println!("'Add Sally to Engineering' or 'Get Engineering'")
                        }
                    }
                }
            }
    }
}

// Given a string of the form "Add <name> to <department>" 
// returns a tuple of Some(name, department) or None
fn parse_add(str: &str) -> Option<(&str, &str)>{
    let str_args: Vec<&str> = str.split_whitespace()
                                 .filter(|&s| !s.to_ascii_lowercase().eq("add") && 
                                              !s.to_ascii_lowercase().eq("to")) // Remove argument keyword
                                 .collect();
    if str_args.len() == 2 && str.split_whitespace().count() == 4{ // argument number requirements
        match (str_args.get(0), str_args.get(1)) {
            (Some(n), Some (d)) => Some((n,d)),
            _ => None 
        }
    } else {
        None
    }
}

// Given a string of the form "Get <department>"
// returns Some(Vec<&str>) containing employees or None
fn parse_get(str: &str, map: &HashMap<String, String>) -> Option<Vec<String>> {
    let str_args: Vec<&str> = str.split_whitespace()
                                 .filter(|&s| !s.to_ascii_lowercase().eq("get")) // Remove argument keyword
                                 .collect();

    if str_args.len() != 1 {
        None
    } else {
        match str_args.get(0) {
            Some(d) => {
                let mut employees: Vec<String> = Vec::new();
                for (k,v) in map.iter() {
                    if v == d {
                        employees.push(k.clone());
                    } 
                }
                employees.sort();
                Some(employees)
            },
            None => None
        }
    }
}
