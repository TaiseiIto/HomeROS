use regex::*;

fn main() {
    let automaton: Automaton = r"^clk(\d+)([kmgtpxzyrq])hz$".parse().unwrap();
    let matches: Vec<Match> = automaton.input("clk24mhz");
    let captures: Vec<Capture> = matches.iter().flat_map(|mat| mat.captures()).collect();
    let captures: Vec<&str> = captures.iter().map(Into::into).collect();
    dbg!(captures);
}
