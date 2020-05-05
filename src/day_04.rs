use regex::Regex;

/// Struct used to represent a watch period conducted by a guard. The sleep periods are inclusive of
/// the start time, but do not include the last time (which is the time the guard work up again).
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct GuardWatch {
    guard_id: u64,
    start_watch: String,
    sleep_periods: Vec<(String, String)>,
}

impl GuardWatch {
    pub fn new(guard_id: u64, start_watch: String) -> Self {
        Self {
            guard_id: guard_id,
            start_watch: start_watch,
            sleep_periods: vec![],
        }
    }

    pub fn add_sleep_period(&mut self, sleep_start: String, sleep_end: String) {
        self.sleep_periods.push((sleep_start, sleep_end));
    }
}

#[aoc_generator(day4)]
fn generate_input(input: &str) -> Vec<GuardWatch> {
    // Sort the observations so they are in date-time order
    let mut lines_vec: Vec<&str> = input.lines().map(|x| x.trim()).collect::<Vec<&str>>();
    lines_vec.sort();
    // Pre-process observations to split into guard watches
    let mut watch_slots: Vec<Vec<&str>> = vec![];
    for line in lines_vec.into_iter() {
        // Add new watch slot when guard comes on watch
        if line.contains("Guard") {
            watch_slots.push(vec![line]);
        } else { // Add asleep or awake observation to most recent guard watch started
            watch_slots.last_mut().unwrap().push(line);
        }
    }
    // Process each of the watch slots
    let mut results: Vec<GuardWatch> = vec![];
    for watch in watch_slots {
        let guard_regex = Regex::new(r"\[(.*?)\] Guard #(\d+)").unwrap();
        for capture in guard_regex.captures_iter(watch[0]) {
            // Extract watch start time and guard_id
            let start_watch = capture[1].to_owned();                
            let guard_id = capture[2].parse::<u64>().unwrap();
            // Create new GuardWatch
            let mut guard_watch = GuardWatch::new(guard_id, start_watch);
            // Process other lines from current watch to determine sleep periods
            for i in (1..watch.len()).step_by(2) {
                let sleep_regex = Regex::new(r"\[(.*?)\] falls asleep").unwrap();
                let awake_regex = Regex::new(r"\[(.*?)\] wakes up").unwrap();
                // Extract the time of guard falling asleep and waking up
                for sleep_capture in sleep_regex.captures_iter(watch[i]) {
                    // Assume that sleep and wake entries are always in pairs (if present).
                    let sleep_start = sleep_capture[1].to_owned();
                    for awake_capture in awake_regex.captures_iter(watch[i+1]) {
                        let sleep_end = awake_capture[1].to_owned();
                        guard_watch.add_sleep_period(sleep_start, sleep_end);
                        break;
                    }
                    break;
                }
            }
            results.push(guard_watch);
            break;
        }
    }
    return results;
}

#[aoc(day4, part1)]
fn solve_part_1(input: &Vec<GuardWatch>) -> u64 {
    // for item in input {
    //     println!("{:?}", item);
    // }
    return 1;
}