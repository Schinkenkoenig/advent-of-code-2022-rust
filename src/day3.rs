use std::fs;

pub fn day_3() {
    let rucksacks = fs::read_to_string("/home/haki/dev/advent-of-code-2022-rust/src/rucksack")
        .expect("File not readable");

    let mut sum_prio = 0;

    for rs in rucksacks.lines() {
        let length = rs.len();
        let split_point = length / 2;

        let first_sack = &rs[0..split_point];
        let second_sack = &rs[split_point..length];

        let common_character = get_common_character(first_sack, second_sack);

        let priority = get_priority(common_character);

        sum_prio += priority;

        println!("{0}, common {1}, prio {2}", rs, common_character, priority);
    }

    println!("Sum of priorities is: {0}", sum_prio);

    let lines: Vec<&str> = rucksacks.lines().collect();

    sum_prio = 0;
    for x in lines.chunks(3){
        let l1 = x[0];
        let l2 = x[1];
        let l3 = x[2];
        let mut common: char = ' ';
        for c in l1.chars(){
            if l2.chars().any(|x| x == c) && l3.chars().any(|x| x == c){
                common = c;
                break;
            }
        }

        let priority = get_priority(common);
        sum_prio += priority;
        println!("{0}", priority)
    }


    println!("Sum of priorities is: {0}", sum_prio);
}

fn get_common_character(s1: &str, s2: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1;
            };
        }
    }

    panic!("Not expected behavior common character should be there");
}

fn get_priority(c: char) -> u32 {
    let digit = c as u32;

    if c.is_ascii_lowercase() {
        return digit - 96;
    }

    if c.is_ascii_uppercase() {
        return 26 + digit - 64;
    }

    panic!("Not expected behavior char should upper or lowercase alphabetical.");
}
