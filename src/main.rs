mod analyzers;

use std::{
    fs,
    path::PathBuf
};

use seedgen::{
    lexer,
    util::{
        settings::{Spawn, Settings},
    },
};

use analyzers::Analyzer;

const SAMPLE_SIZE: usize = 100;

fn main() {
    let analyzers: Vec<Box<dyn Analyzer>> = vec![
        Box::new(analyzers::SpawnLocation {}),
        Box::new(analyzers::LaunchLocation {}),
        Box::new(analyzers::LaunchTiming {}),
    ];

    let mut out = String::new();

    let mut line = Vec::new();
    for analyzer in &analyzers {
        line.push(analyzer.name());
    }
    out.push_str(&line.join(","));
    out.push('\n');

    let mut settings = Settings::default();
    settings.spawn_loc = Spawn::Random;
    let graph = lexer::parse_logic(&PathBuf::from("areas.wotw"), &PathBuf::from("loc_data.csv"), &PathBuf::from("state_data.csv"), &settings.pathsets, false).unwrap();

    for index in 0..SAMPLE_SIZE {
        let progress = format!("\rAnalyzing {}/{} seeds", index + 1, SAMPLE_SIZE);
        eprint!("{}", progress);

        let mut line = Vec::new();
        let seed = seedgen::generate_seed(&graph, &settings, &vec![], None).unwrap().pop().unwrap();

        for analyzer in &analyzers {
            line.push(analyzer.value(&seed));
        }
        out.push_str(&line.join(","));
        out.push('\n');
    }

    eprintln!();

    fs::write("target/stats.csv", out).unwrap();
}
