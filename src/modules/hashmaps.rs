use std::collections;

pub fn hash_maps() {
    let mut scores = collections::HashMap::new();

    scores.insert(String::from("blue_team"), 10);
    scores.insert(String::from("red_team"), 11);

    let mteam = String::from("red_team");
    let team_score = scores.get(&mteam);
    match team_score {
        Some(score) => println!("Team '{}' score: {}", mteam, score),
        None => println!("No score found for team '{mteam}' "),
    }

    println!("Final scores : ");
    for score in &scores {
        println!("Team '{}' | score: {}", score.0, score.1);
    }

    // let b = scores.get("blue");
}

pub fn overwrite_values() {
    let mut scores = collections::HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // Using the entry method:
    scores.entry(String::from("Blue")).or_insert(25);

    println!("Scores : {:?}", scores);
}

pub fn update_values() {
    let text = "hello world wonderful world";
    let mut map = collections::HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // println!("count {}", count);
    }

    println!("hashmap: {:?}", map);
}
