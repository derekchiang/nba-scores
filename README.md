# NBA Scores

A simple program that prints out the current NBA scores.  It does so by scraping ESPN.

Example output:

```
Final: Denver Nuggets(home) 99 : 103 Phoenix Suns(away)
Final: Dallas Mavericks(home) 108 : 109 Toronto Raptors(away)
Final: Indiana Pacers(home) 114 : 81 Houston Rockets(away)
Final: Miami Heat(home) 122 : 103 Sacramento Kings(away)
Final: Detroit Pistons(home) 106 : 116 Charlotte Bobcats(away)
Final: Cleveland Cavaliers(home) 114 : 111 Milwaukee Bucks(away)
Final: Atlanta Hawks(home) 118 : 85 Utah Jazz(away)
Final: Philadelphia 76ers(home) 121 : 120 Brooklyn Nets(away)
Live: Los Angeles Lakers(home) 51 : 53 Minnesota Timberwolves(away)
```

## Installation

To compile the code, you need to install the following:

1. [Rust](https://github.com/mozilla/rust)
2. [rust-http](https://github.com/chris-morgan/rust-http)

Then:

1. Clone the repo.
2. Compile main.rs to obtain the executable: `rustc -O -o nba main.rs`
3. Put the executable in your `path`, and enjoy!

## License

[WTFPL](http://www.wtfpl.net/).