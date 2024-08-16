use std::collections::HashMap;

fn main() {
    let teams: [(&str, i32); 3] = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }
    println!("{:?}", teams_map1);

    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
    assert_eq!(teams_map1, teams_map2);

    println!("suc");
}
