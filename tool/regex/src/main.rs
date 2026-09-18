use regex::*;

fn main() {
    let automaton: Automaton = r"^clk(\d+)([kmgtpxzyrq])hz$".parse().unwrap();
    dbg!(&automaton);
    let matches: Vec<Match> = automaton.input("clk24mhz");
    dbg!(&matches);
    let captures: Vec<Capture> = matches.iter().flat_map(|mat| mat.captures()).collect();
    dbg!(&captures);
    let captures: Vec<&str> = captures.iter().map(Into::into).collect();
    dbg!(&captures);
}
