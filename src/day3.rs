use std::fs;

pub fn day_3() {
    let rucksacks = fs::read_to_string("rucksack").expect("File not readable");

    let mut sum_prio = 0;

    for rs in rucksacks.lines() {
        let length = rs.len();
        let split_point = length / 2;

        let first_sack = &rs[0..split_point];
        let second_sack = &rs[split_point..length];

        let common_character = get_common_character(first_sack, second_sack).unwrap();

        let priority = get_priority(common_character);

        sum_prio += priority;

        println!("{0}, common {1}, prio {2}", rs, common_character, priority);
    }

    println!("Sum of priorities is: {0}", sum_prio);

    let lines: Vec<_> = rucksacks.lines().collect();

    sum_prio = 0;
    for line_triple in lines.chunks(3) {
        let [l1, l2, l3] = line_triple else {
            panic!("There should 3 in in it!");
        };

        let common = l1
            .chars()
            .find(|c| l2.chars().any(|x| x == *c) && l3.chars().any(|x| x == *c))
            .unwrap();

        let priority = get_priority(common);
        sum_prio += priority;
        println!("{0}", priority)
    }

    println!("Sum of priorities is: {0}", sum_prio);
}

fn get_common_character(s1: &str, s2: &str) -> Option<char> {
    let common_char_opt = s1
        .chars()
        .find(|c| s2.chars().any(|x| x == *c));

    return common_char_opt;
}

fn get_priority(c: char) -> u32 {
    let digit = c as u32;

    match c {
        _d if c.is_ascii_lowercase() => return digit - 96,
        _d if c.is_ascii_uppercase() => return digit - 64 + 26,
        _ => {
            panic!("Not expected behavior char should upper or lowercase alphabetical.");
        }
    }
}
