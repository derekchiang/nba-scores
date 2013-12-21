#[crate_id = "nba"];

extern mod extra;
extern mod http;

use std::io::buffered::BufferedReader;
use std::libc::funcs::c95::stdlib::exit;

use extra::json;
use extra::json::{List, Object, String, Number};

use http::client::RequestWriter;
use http::method::Get;

static PATTERN: &'static str = "var sbMaster = ";

fn fail() {
    println("Unable to parse ESPN.
Please file an issue at github.com/derekchiang/nba-scores");
    unsafe {
        exit(1);
    }
}

fn main() {
    let request = RequestWriter::new(Get, from_str("http://espn.go.com/nba/").unwrap());
    let mut response = match request.read_response() {
        Ok(response) => BufferedReader::new(response),
        Err(_) => unreachable!(),
    };

    for line in response.lines() {
        match line.find_str(PATTERN) {
            Some(_) => {
                match json::from_str(line.slice_from(PATTERN.len())) {
                    Ok(val) => {
                        match val {
                            Object(mut obj) => {
                                match obj.pop(&~"sports") {
                                    Some(lst) => {
                                        match lst {
                                            List(lst) => {
                                                for sport in lst.move_rev_iter() {
                                                    match sport {
                                                        Object(mut obj) => {
                                                            match obj.pop(&~"sport") {
                                                                Some(String(s)) => {
                                                                    if "nba" == s {
                                                                        match obj.pop(&~"leagues") {
                                                                            Some(List(lst)) => {
                                                                                match lst[0] {
                                                                                    Object(mut obj) => {
                                                                                        match obj.pop(&~"games") {
                                                                                            Some(List(games)) => {
                                                                                                for game in games.move_rev_iter() {
                                                                                                    match game {
                                                                                                        Object(mut obj) => {
                                                                                                            match obj.pop(&~"status") {
                                                                                                                Some(Number(status)) => {
                                                                                                                    match status {
                                                                                                                        1.0 => {
                                                                                                                            match obj.pop(&~"statusText") {
                                                                                                                                Some(String(status_text)) => {
                                                                                                                                    print!("Incoming ({}): ", status_text);
                                                                                                                                },
                                                                                                                                _ => fail()
                                                                                                                            }
                                                                                                                        },
                                                                                                                        3.0 => {
                                                                                                                            print("Final: ");
                                                                                                                        },
                                                                                                                        _ => {
                                                                                                                            print("Live: ");
                                                                                                                        }
                                                                                                                    }
                                                                                                                },
                                                                                                                _ => fail()
                                                                                                            }
                                                                                                            match obj.pop(&~"home") {
                                                                                                                Some(Object(mut home)) => {
                                                                                                                    match (home.pop(&~"location"), home.pop(&~"nickname"), home.pop(&~"score")) {
                                                                                                                        (Some(String(location)), Some(String(nickname)), Some(Number(score))) => {
                                                                                                                            print!("{} {}(home) {} : ", location, nickname, score);
                                                                                                                        },
                                                                                                                        _ => fail()
                                                                                                                    }
                                                                                                                },
                                                                                                                _ => fail()
                                                                                                            };
                                                                                                            match obj.pop(&~"away") {
                                                                                                                Some(Object(mut away)) => {
                                                                                                                    match (away.pop(&~"location"), away.pop(&~"nickname"), away.pop(&~"score")) {
                                                                                                                        (Some(String(location)), Some(String(nickname)), Some(Number(score))) => {
                                                                                                                            println!("{} {} {}(away)", score, location, nickname);
                                                                                                                        },
                                                                                                                        _ => fail()
                                                                                                                    }
                                                                                                                },
                                                                                                                _ => fail()
                                                                                                            };
                                                                                                        },
                                                                                                        _ => fail()
                                                                                                    }
                                                                                                }
                                                                                            },
                                                                                            _ => fail()
                                                                                        }
                                                                                    },
                                                                                    _ => fail()
                                                                                }
                                                                            },
                                                                            _ => fail()
                                                                        }
                                                                    } else {
                                                                        continue;
                                                                    }
                                                                },
                                                                _ => fail()
                                                            }
                                                        },
                                                        _ => fail()
                                                    }
                                                }
                                            },
                                            _ => fail()
                                        }
                                    },
                                    _ => {
                                        fail();
                                    }
                                }
                            },
                            _ => {
                                fail();
                            }
                        }
                    },
                    Err(_) => {
                        fail();
                    }
                };
                return;
            },
            _ => continue
        }
    }

    fail();
}