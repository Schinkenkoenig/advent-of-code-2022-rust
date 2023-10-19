mod day1;
mod day2;
mod day3;
mod day4;

use day1::day_1;
use day2::day_2;
use day3::day_3;
use day4::day_4;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let [_exe, day] = args.as_slice() else {
        panic!("Please provide at least one parameter");
    };

    let day: u32 = day.parse().unwrap();

    match day {
        1 => {
            println!("Execute day 1");
            day_1();
        }
        2 => {
            println!("Execute day 2");
            day_2();
        },
        3 => {
            println!("Execute day 3");
            day_3();
        }
        4 => {
            println!("Execute day 4");
            day_4();
        },
        _ => {
            todo!("Day {0} not implemented", day);
        }
    }
}
