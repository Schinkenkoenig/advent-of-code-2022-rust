use std::fs;

pub fn day_2() {
    let round_strategies =
        fs::read_to_string("rps_strategy")
            .expect("File not readable");

    let mut total_points_1 = 0;
    let mut total_points_2 = 0;
    for round in round_strategies.lines() {
        let choices: Vec<_> = round.split(" ").collect();

        if choices.len() != 2 {
            continue;
        }

        let opponent = choices[0];
        let player = choices[1];


        let round_pts_1 = compute_result_points_1(opponent, player);
        let round_pts_2 = compute_result_points_2(opponent, player);

        total_points_1 += round_pts_1;
        total_points_2 += round_pts_2;

        println!("{0}:{1},{2}, {3}", opponent, player, round_pts_1, round_pts_2);
    }

    println!("Total points: {0}, {1}", total_points_1, total_points_2);
}

fn compute_result_points_1(opponent: &str, player: &str) -> i32 {
    let choice_points = match player {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    let result_points = match (opponent, player) {
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("C", "Y") => 0,
        ("A", "X") => 3,
        ("B", "Y") => 3,
        ("C", "Z") => 3,
        ("A", "Y") => 6,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        _ => 0,
    };

    return result_points + choice_points;
}

fn compute_result_points_2(opponent: &str, player: &str) -> i32 {
    let new_player = match (opponent, player) {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => "Whatever",
    };
    
    return compute_result_points_1(opponent, new_player)
}
