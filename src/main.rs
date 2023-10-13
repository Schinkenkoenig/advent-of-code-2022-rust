use std::fs;

fn main() {
    let calories = fs::read_to_string("calories").expect("File not readable");
    

    let mut calory_list:Vec<i32> = Vec::new();
    let mut total = 0;
    for line in calories.lines(){
        if line.trim().is_empty(){
            println!("Processing new elf");
            calory_list.push(total);
            total = 0;
        } else {
          let calory :i32  = line.trim().parse().unwrap();
          total += calory;
        }
    }

    println!("{:?}", calory_list);
            


    println!("Hello, world!");
}
