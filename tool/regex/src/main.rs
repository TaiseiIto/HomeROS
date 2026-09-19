use {regex::*, std::collections::btree_map::BTreeMap};

fn main() {
    let automaton: Automaton = r"^(clk(?<value>\d+)(?<unit>[kmgtpxzyrq]?)hz,?){2}$"
        .parse()
        .unwrap();
    let matches: Vec<Match> = automaton.input("clk24mhz,clk115200hz");
    let captures: Vec<BTreeMap<String, Vec<Capture>>> =
        matches.iter().map(|mat| mat.captures()).collect();
    dbg!(captures);
}
