
use std::fs;

pub fn day_1() {
    let calories = fs::read_to_string("/home/haki/dev/advent-of-code-2022-rust/src/calories")
        .expect("File not readable");
    
    let mut calory_list: Vec<i32> = Vec::new();
    let mut total = 0;
    for line in calories.lines() {
        if line.trim().is_empty() {
            println!("Processing new elf");
            calory_list.push(total);
            total = 0;
        } else {
            let calory: i32 = line.trim().parse().unwrap();
            total += calory;
        }
    }

    // last total should be added
    calory_list.push(total);
    println!("{:?}", calory_list);

    let max = calory_list.iter().max().unwrap();
    let idx = calory_list.iter().position(|x| x == max).unwrap();

    println!(
        "Maximum calories carried by {:?} with the amount {:?}.",
        idx + 1,
        max
    );

    calory_list.sort_by(|a, b| b.cmp(a));
    let max_by_three = calory_list[0] + calory_list[1] + calory_list[2];

    println!("The max three calories packers have: {:?}", max_by_three);
}


