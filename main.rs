#[crate_id = "nba"];
#[feature(macro_rules)];

extern mod extra;
extern mod http;

use std::io::buffered::BufferedReader;
use std::os::set_exit_status;

use extra::json;
use extra::json::{List, Object, String, Number};

use http::client::RequestWriter;
use http::method::Get;

static PATTERN: &'static str = "var sbMaster = ";

fn fail() {
    println("Unable to parse ESPN.");
    println("Please file an issue at github.com/derekchiang/nba-scores");
    set_exit_status(1);
}

macro_rules! take_or_fail(($val:expr, $ok:pat => $out:expr) => {
    match $val {
        $ok => $out,
        _ => {
            fail();
            return;
        }
    }
})

fn main() {
    let request = RequestWriter::new(Get, from_str("http://espn.go.com/nba/").unwrap());
    let mut response = match request.read_response() {
        Ok(response) => BufferedReader::new(response),
        Err(_) => unreachable!(),
    };

    for line in response.lines() {
        if line.find_str(PATTERN).is_none() {
            continue;
        }
        let val = take_or_fail!(json::from_str(line.slice_from(PATTERN.len())), Ok(val) => val);
        let mut obj = take_or_fail!(val, Object(obj) => obj);
        let lst = take_or_fail!(obj.pop(&~"sports"), Some(List(lst)) => lst);
        for sport in lst.move_rev_iter() {
            let mut obj = take_or_fail!(sport, Object(obj) => obj);
            let s = take_or_fail!(obj.pop(&~"sport"), Some(String(s)) => s);
            if "nba" == s {
                let lst = take_or_fail!(obj.pop(&~"leagues"), Some(List(lst)) => lst);
                let mut obj = take_or_fail!(lst[0], Object(obj) => obj);
                let games = take_or_fail!(obj.pop(&~"games"), Some(List(games)) => games);
                for game in games.move_rev_iter() {
                    let mut obj = take_or_fail!(game, Object(obj) => obj);
                    let status = take_or_fail!(obj.pop(&~"status"), Some(Number(status)) => status);
                    match status {
                        1.0 => {
                            let status_text = take_or_fail!(obj.pop(&~"statusText"), Some(String(status_text)) => status_text);
                            print!("Incoming ({}): ", status_text);
                        },
                        3.0 => {
                            print("Final: ");
                        },
                        _ => {
                            print("Live: ");
                        }
                    };
                    let mut home = take_or_fail!(obj.pop(&~"home"), Some(Object(home)) => home);
                    let (location, nickname, score) = take_or_fail!((home.pop(&~"location"), home.pop(&~"nickname"), home.pop(&~"score")),
                        (Some(String(location)), Some(String(nickname)), Some(Number(score))) => (location, nickname, score));
                    print!("{} {}(home) {} : ", location, nickname, score);
                    let mut away = take_or_fail!(obj.pop(&~"away"), Some(Object(away)) => away);
                    let (location, nickname, score) = take_or_fail!((away.pop(&~"location"), away.pop(&~"nickname"), away.pop(&~"score")),
                        (Some(String(location)), Some(String(nickname)), Some(Number(score))) => (location, nickname, score));
                    println!("{} {} {}(away)", score, location, nickname);
                }
                return;
            } else {
                continue;
            }
        }
    }
}