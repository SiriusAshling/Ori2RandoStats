use seedgen::util::settings;

pub trait Analyzer {
    fn name(&self) -> String;
    fn value(&self, seed: &str) -> String;
}

pub struct SpawnLocation {}
impl Analyzer for SpawnLocation {
    fn name(&self) -> String { String::from("Spawn Location") }
    fn value(&self, seed: &str) -> String {
        settings::read_spawn(seed).unwrap()
    }
}

pub struct LaunchLocation {}
impl Analyzer for LaunchLocation {
    fn name(&self) -> String { String::from("Launch Location") }
    fn value(&self, seed: &str) -> String {
        let pattern = "Launch from";
        for line in seed.lines() {
            if let Some(index) = line.find(pattern) {
                let location = &line[index + pattern.len() + 1..];
                let location = location.splitn(2, ' ').next().unwrap();
                return location.to_string();
            }
        }

        panic!("No Launch in seed!");
    }
}

pub struct LaunchTiming {}
impl Analyzer for LaunchTiming {
    fn name(&self) -> String { String::from("Launch Timing") }
    fn value(&self, seed: &str) -> String {
        let pattern = "Launch from";
        for (index, line) in seed.lines().enumerate() {
            if line.find(pattern).is_some() {
                return index.to_string();
            }
        }

        panic!("No Launch in seed!");
    }
}
