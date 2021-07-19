use seedgen::{
    util::settings,
    util::constants::SHOP_PRICES,
};

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

pub struct ShopCostTotal {}
impl Analyzer for ShopCostTotal {
    fn name(&self) -> String { String::from("Shop Cost Total") }
    fn value(&self, seed: &str) -> String {
        let mut shop_states = SHOP_PRICES.iter().map(|(_, _, state)| format!("|8|{}|int|", state)).collect::<Vec<_>>();
        let mut total_price = 0;

        'outer: for line in seed.lines() {
            for (state_index, shop_state) in shop_states.iter().enumerate() {
                if let Some(start_index) = line.find(shop_state) {
                    let untrimmed = &line[start_index + shop_state.len()..];
                    let end_index = untrimmed.find(char::is_whitespace).unwrap_or_else(|| untrimmed.len());
                    let trimmed = &untrimmed[..end_index];

                    let price: u16 = trimmed.parse().unwrap_or_else(|_| panic!("Failed to parse price \"{}\" from \"{}\"", trimmed, line));
                    total_price += price;

                    shop_states.remove(state_index);

                    if shop_states.is_empty() {
                        break 'outer;
                    }
                    break;
                }

            }
        }

        for shop_state in shop_states {
            eprintln!("Warn: couldn't find price for shop item {}", shop_state);
        }

        return total_price.to_string();
    }
}
