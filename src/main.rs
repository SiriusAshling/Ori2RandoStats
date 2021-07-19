mod analyzers;

use std::{
    fs,
    path::PathBuf
};

use seedgen::{
    lexer,
    util::settings::Settings,
};

use analyzers::Analyzer;

const SAMPLE_SIZE: usize = 500;

fn main() {
    let analyzers: Vec<Box<dyn Analyzer>> = vec![
        Box::new(analyzers::SpawnLocation {}),
        Box::new(analyzers::LaunchLocation {}),
        Box::new(analyzers::LaunchTiming {}),
        Box::new(analyzers::ShopCostTotal {}),
    ];

    let mut out = String::new();

    let mut line = Vec::new();
    for analyzer in &analyzers {
        line.push(analyzer.name());
    }
    out.push_str(&line.join(","));
    out.push('\n');

    let mut settings = Settings::default();
    settings.presets.push(PathBuf::from("gorlek"));
    settings = settings.apply_presets().unwrap();
    let headers = vec![];
    let graph = lexer::parse_logic(&PathBuf::from("areas.wotw"), &PathBuf::from("loc_data.csv"), &PathBuf::from("state_data.csv"), &settings.pathsets, false).unwrap();

    for index in 0..SAMPLE_SIZE {
        let progress = format!("\rAnalyzing {}/{} seeds", index + 1, SAMPLE_SIZE);
        eprint!("{}", progress);

        let mut line = Vec::new();
        let seed = seedgen::generate_seed(&graph, settings.clone(), &headers, None).unwrap().0.pop().unwrap();

        for analyzer in &analyzers {
            line.push(analyzer.value(&seed));
        }
        out.push_str(&line.join(","));
        out.push('\n');
    }

    eprintln!();

    fs::write("target/stats.csv", out).unwrap();
}
