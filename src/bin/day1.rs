use std::{collections::HashSet, fs};

const DATA_FOLDER: &str = "./data";

type FrequencyType = i32;

struct Device {
    friquency: FrequencyType,
}

impl Device {
    fn new() -> Self {
        Self { friquency: 0 }
    }

    fn get_frequency(&self) -> FrequencyType {
        self.friquency
    }

    fn calibrate_friquency(&mut self, changes: &str) {
        let sign_split_index = 1;
        for line in changes.lines() {
            let (sign, value) = line.split_at(sign_split_index);
            let amount = value
                .parse::<i32>()
                .expect("Unable to convert value into i32");

            match sign {
                "-" => {
                    self.friquency -= amount;
                }
                "+" => {
                    self.friquency += amount;
                }
                _ => {
                    println!("Unknown sign: {}", sign);
                }
            };
        }
    }

    fn calibrate_friquency_without_repetition(&mut self, changes: &str) {
        let sign_split_index = 1;
        let mut change_list = HashSet::from([self.get_frequency()]);
        loop {
            for line in changes.lines() {
                let (sign, value) = line.split_at(sign_split_index);
                let amount = value
                    .parse::<FrequencyType>()
                    .expect("Unable to conver value into number");

                match sign {
                    "-" => {
                        self.friquency -= amount;
                    }
                    "+" => {
                        self.friquency += amount;
                    }
                    _ => {
                        println!("Unknown sign: {}", sign);
                    }
                }

                if !change_list.insert(self.friquency) {
                    return;
                }
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string(format!("{DATA_FOLDER}/day1.txt")).expect("missing input");
    let mut device = Device::new();
    device.calibrate_friquency(&data);
    println!("result without repetition {}", device.get_frequency());

    let mut device_with_repetition = Device::new();
    device_with_repetition.calibrate_friquency_without_repetition(&data);
    println!(
        "result with repetition {}",
        device_with_repetition.get_frequency()
    );
}
