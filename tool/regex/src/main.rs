use regex::*;

fn main() {
    let automaton: Automaton = dbg!(r"^clk(\d+)([kmgtpxzyrq])hz$".parse().unwrap());
    let matches: Vec<Match> = automaton.input("clk24mhz");
    let captures: Vec<Capture> = matches
        .iter()
        .flat_map(|mat| mat.captures().into_iter())
        .collect();
    dbg!(captures);
}
