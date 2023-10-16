use std::fs;

pub fn day_4() {
    let section_assignments =
        fs::read_to_string("/home/haki/dev/advent-of-code-2022-rust/src/section_assignments")
            .expect("File not readable");

    let mut count_everything = 0;
    let mut count_at_all = 0;
    for sa in section_assignments.lines() {
        let (s1, s2) = sa.split_once(",").unwrap();

        let (start1, end1) = s1.split_once("-").unwrap();
        let (start2, end2) = s2.split_once("-").unwrap();

        let (start1, end1): (i32, i32) = (start1.parse().unwrap(), end1.parse().unwrap());
        let (start2, end2): (i32, i32) = (start2.parse().unwrap(), end2.parse().unwrap());

        if start1 >= start2 && end1 <= end2 || start2 >= start1 && end2 <= end1 {
            println!("Found containment in pair {0}, {1}", s1, s2);
            count_everything += 1;
        }

        if end1 >= start2 && end2 >= start1 {
            println!("Found at all containments {0}, {1}", s1, s2);
            count_at_all += 1;
        }


    }

    println!("Found {0} containemnts", count_everything);
    println!("Found {0} containemnts", count_at_all);
}
