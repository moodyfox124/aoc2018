use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

const DATA_FOLDER: &str = "./data";

#[derive(Debug)]
struct Chimney {
    claims: Vec<Claim>,
}

impl Chimney {
    fn new() -> Self {
        Self { claims: Vec::new() }
    }

    fn add_claims(&mut self, data: &str) {
        for line in data.lines() {
            self.claims.push(
                line.parse::<Claim>()
                    .expect("Unable to parse line as Claim"),
            );
        }
    }

    fn sizes_by_two_or_more_claims(&self) {
        let mut claim = HashMap::new();
        for c in &self.claims {
            for width in 0..c.width {
                for height in 0..c.height {
                    *claim.entry((c.left + width, c.top + height)).or_insert(0) += 1;
                }
            }
        }
        println!(
            "result {:?}",
            claim
                .iter()
                .filter(|(_key, value)| { **value >= 2 })
                .count()
        );
    }

    fn id_of_non_overlapping_claim(&self) {
        let mut claim = HashMap::new();
        for c in &self.claims {
            for width in 0..c.width {
                for height in 0..c.height {
                    *claim.entry((c.left + width, c.top + height)).or_insert(0) += 1;
                }
            }
        }

        'next_claim: for c in &self.claims {
            for width in 0..c.width {
                for height in 0..c.height {
                    let Some(val) = claim.get(&(c.left + width, c.top + height)) else {
                        panic!("unable to get value by key");
                    };
                    if *val != 1 {
                        continue 'next_claim;
                    }
                }
            }
            println!("result id {}", c.id);
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: i32,
    left: usize,
    top: usize,

    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount_to_skip = 2;
        let position_delimeter = ',';
        let size_delimeter = 'x';
        let (id_str, _) = s.split_once(" @ ").expect("unable to split once");
        let id = id_str[1..].parse().expect("unable to parse id");
        let parts: Vec<&str> = s.split(' ').skip(amount_to_skip).collect();

        let position = parts[0].split(position_delimeter).collect::<Vec<&str>>();
        let left = position[0]
            .parse::<usize>()
            .expect("Unable to parse left value");
        let mut top_str = String::from(position[1]);
        top_str.remove(top_str.len() - 1);
        let top = top_str.parse().expect("Unable to parse top value");

        let sizes = parts[1].split(size_delimeter).collect::<Vec<&str>>();

        let width = sizes[0].parse().expect("Unable to parse width");
        let height = sizes[1].parse().expect("Unable to parse height");

        return Ok(Self {
            id,
            left,
            top,
            height,
            width,
        });
    }
}

fn main() {
    let data =
        fs::read_to_string(format!("{DATA_FOLDER}/day3.txt")).expect("Unable to read input data");
    let mut chimney = Chimney::new();
    chimney.add_claims(&data);

    chimney.sizes_by_two_or_more_claims();

    chimney.id_of_non_overlapping_claim();
}
