use derivative::Derivative;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
struct WorldEvent {
    time: String,
    date: String,
    tick: i32,
    event_type: String,
}

struct PlayerEvent {}

enum Class {
    Scout,
    Soldier,
    Pyro,
    Demoman,
    Heavy,
    Engineer,
    Medic,
    Sniper,
    Spy,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Class::Scout => write!(f, "Scout"),
            Class::Soldier => write!(f, "Soldier"),
            Class::Pyro => write!(f, "Pyro"),
            Class::Demoman => write!(f, "Demoman"),
            Class::Heavy => write!(f, "Heavy"),
            Class::Engineer => write!(f, "Engineer"),
            Class::Medic => write!(f, "Medic"),
            Class::Sniper => write!(f, "Sniper"),
            Class::Spy => write!(f, "Spy"),
        }
    }
}
struct Team {
    color: String,
    players: Vec<Player>,
    damage: i32,
    healing: i32,
    kills: i32,
    deaths: i32,
}
#[derive(Derivative)]
#[derivative(Debug, Default)]
struct Player {
    name: String,
    id: i32,
    team: String,
    damage: String,
    class: String,
    #[derivative(Default(value = "0"))]
    healing: i32,
}
struct Log {}

struct User<'a> {
    username: &'a str,
    id: i32,
    steam_id: &'a str,
    team: &'a str,
}

fn reprint(re: Regex, line: &str) {
    println!("{}", re.find(line).map(|x| x.as_str()).unwrap_or(""));
}
fn reprint_grp(re: Regex, line: &str) -> User {
    let groups = re.captures(line).unwrap();
    return User {
        username: groups.get(1).unwrap().as_str(),
        id: groups.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        steam_id: groups.get(3).unwrap().as_str(),
        team: groups.get(4).unwrap().as_str(),
    };
}

fn parse_line(line: &String, last_line: &String) {
    let date_regex = Regex::new(r"\d{2}/\d{2}/\d{4}").expect("Failed to parse date");
    let time_regex = Regex::new(r"\d{2}[:]\d{2}[:]\d{2}").expect("Failed to parse time");
    let user_string =
        Regex::new(r##"["](.{0,32})[<](\d{2})[>][<](\[.*\])[>][<](.{3,9})[>]["]"##).expect("fuck");
    reprint(date_regex, line);
    reprint(time_regex, line);
    let user = reprint_grp(user_string, line);
    println!("{}", user.username)
}

fn main() {
    let file = File::open("/home/ubuntu-server/logs_parser/src/logs/log_3383757.log").unwrap();
    let reader = BufReader::new(file);
    let mut last_line = String::from("");
    for line in reader.lines() {
        match line {
            Ok(l) => match last_line.as_str() {
                "" => {
                    last_line = l.clone();
                    parse_line(&l, &last_line);
                }
                _ => {
                    parse_line(&l, &last_line);
                    last_line = l.clone();
                }
            },
            Err(e) => println!("{}", e),
        };
    }
}
