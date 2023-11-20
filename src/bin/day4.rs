use std::{cmp::Ordering, fs, str::FromStr};

#[derive(Debug)]
struct Schedule {
    entries: Vec<ScheduleEntry>,
}

impl Schedule {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn add_entries(&mut self, data: &str) {
        for line in data.lines() {
            let entry = line
                .parse::<ScheduleEntry>()
                .expect("unable to parse entry");
            self.entries.push(entry);
        }
        self.entries.sort_by(|a, b| {
            if a.year == b.year {
                if a.month == b.month {
                    if a.day == b.day {
                        if a.hour == b.hour {
                            if a.minute > b.minute {
                                return Ordering::Greater;
                            } else {
                                return Ordering::Less;
                            }
                        } else {
                            if a.hour > b.hour {
                                return Ordering::Greater;
                            } else {
                                return Ordering::Less;
                            }
                        }
                    } else {
                        if a.day > b.day {
                            return Ordering::Greater;
                        } else {
                            return Ordering::Less;
                        }
                    }
                } else {
                    if a.month > b.month {
                        return Ordering::Greater;
                    } else {
                        return Ordering::Less;
                    }
                }
            } else {
                if a.year > b.year {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        });
    }

    fn fill_id_gaps(&mut self) {
        let mut curret_id = None;
        self.entries.iter_mut().for_each(|v| {
            if v.action == GuardAction::StartWork {
                curret_id = v.guard_id;
            } else {
                v.guard_id = curret_id
            }
        })
    }
}

#[derive(Debug)]
struct ScheduleEntry {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    guard_id: Option<usize>,
    action: GuardAction,
}

impl FromStr for ScheduleEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date_time, details) = s.split_once("] ").expect("Unable to split by '] '");
        let (date, time) = date_time[1..]
            .split_once(' ')
            .expect("Unable to split date_time");

        let splitted_date = date.split('-').collect::<Vec<&str>>();

        let year = splitted_date[0].parse().expect("Unable to parse year");
        let month = splitted_date[1].parse().expect("Unable to parse month");
        let day = splitted_date[2].parse().expect("Unable to parse dat");

        let (hours_str, minutes_str) = time.split_once(':').expect("Unable to split time");

        let hour = hours_str.parse().expect("Unable to parse hours");
        let minute = minutes_str.parse().expect("Unable to parse minutes");

        let action_description = details.split_whitespace().take(2).collect::<Vec<&str>>();
        let guard_id;
        let action;

        if action_description.starts_with(&["wakes"]) {
            action = GuardAction::WakesUp;
            guard_id = None;
        } else if action_description.starts_with(&["falls"]) {
            guard_id = None;
            action = GuardAction::Sleep;
        } else {
            action = GuardAction::StartWork;
            let id_str = action_description[1];
            guard_id = Some(id_str[1..].parse().expect("Unable to parse id_str"));
        }

        return Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            guard_id,
            action,
        });
    }
}

#[derive(Debug, PartialEq)]
enum GuardAction {
    Sleep,
    WakesUp,
    StartWork,
}

fn main() {
    let data = fs::read_to_string("./data/day4.txt").expect("Unable to read input data");
    let mut schedule = Schedule::new();

    schedule.add_entries(&data);

    schedule.fill_id_gaps();

    println!("{:?}", schedule.entries);
}
