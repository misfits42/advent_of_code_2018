use regex::Regex;
use std::collections::HashMap;

/// Struct used to represent a watch period conducted by a guard. The sleep periods are inclusive of
/// the start time, but do not include the last time (which is the time the guard work up again).
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct GuardWatch {
    id: u64,
    start_watch: String,
    sleep_periods: Vec<(String, String)>,
}

impl GuardWatch {
    pub fn new(id: u64, start_watch: String) -> Self {
        Self {
            id: id,
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
    // Process each guard watch and add up all minutes asleep by each guard
    let mut sleep_sheets: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    let mut sleep_totals: HashMap<u64, u64> = HashMap::new();
    // Initialise variables to keep track of sleepiest guard
    let mut sleepiest_guard = 0;
    let mut max_sleep = 0;
    // Process each sleep period from each guard watch
    for watch in input {
        for sleep_period in &watch.sleep_periods {
            let sleep_mins = calculate_sleep_period_minutes(sleep_period);
            // Check if guard has already been seen
            if sleep_sheets.contains_key(&watch.id) {
                let existing_sleep_sheet = sleep_sheets.get_mut(&watch.id).unwrap();
                for i in sleep_mins.0..sleep_mins.1 {
                    *existing_sleep_sheet.get_mut(&i).unwrap() += 1;
                }
                *sleep_totals.get_mut(&watch.id).unwrap() += sleep_mins.1 - sleep_mins.0;
            } else { // Add guard sleep times new
                let mut new_sleep_sheet: HashMap<u64, u64> = create_new_guard_sleep_sheet();
                for i in sleep_mins.0..sleep_mins.1 {
                    *new_sleep_sheet.get_mut(&i).unwrap() += 1;
                }
                sleep_sheets.insert(watch.id, new_sleep_sheet);
                sleep_totals.insert(watch.id, sleep_mins.1 - sleep_mins.0);
            }
            // Check if we have a new max sleep value - have we found a more sleepy guard?
            if *sleep_totals.get(&watch.id).unwrap() > max_sleep {
                max_sleep = *sleep_totals.get(&watch.id).unwrap();
                sleepiest_guard = watch.id;
            }
        }
    }
    // Find minute when guard was asleep most
    max_sleep = 0;
    let mut max_sleep_minute = 0;
    for (k, v) in sleep_sheets.get(&sleepiest_guard).unwrap().iter() {
        if *v > max_sleep {
            max_sleep = *v;
            max_sleep_minute = *k;
        }
    }
    return sleepiest_guard * max_sleep_minute;
}

#[aoc(day4, part2)]
fn solve_part_2(input: &Vec<GuardWatch>) -> u64 {
    // Keep track of guard sleep sheets
    let mut sleep_sheets: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    // Keep track of target guard and sleep minute
    let mut target_guard = 0;
    let mut target_minute = 0;
    let mut max_minute_sleep = 0;
    // Process each guard watch and sleep period contained within
    for watch in input {
        for sleep_period in &watch.sleep_periods {
            let sleep_mins = calculate_sleep_period_minutes(sleep_period);
            // Check if guard already seen
            if sleep_sheets.contains_key(&watch.id) {
                let existing_sleep_sheet = sleep_sheets.get_mut(&watch.id).unwrap();
                for minute in sleep_mins.0..sleep_mins.1 {
                    // Increment sleep for minute
                    *existing_sleep_sheet.get_mut(&minute).unwrap() += 1;
                    // Check if incremented sleep makes current guard the target
                    if *existing_sleep_sheet.get(&minute).unwrap() > max_minute_sleep {
                        max_minute_sleep = *existing_sleep_sheet.get(&minute).unwrap();
                        target_minute = minute;
                        target_guard = watch.id;
                    }
                }
            } else { // Add new guard to sleep sheets
                let mut new_sleep_sheet: HashMap<u64, u64> = create_new_guard_sleep_sheet();
                for minute in sleep_mins.0..sleep_mins.1 {
                    *new_sleep_sheet.get_mut(&minute).unwrap() += 1;
                    // Check if a new target guard has been found
                    if *new_sleep_sheet.get(&minute).unwrap() > max_minute_sleep {
                        max_minute_sleep = *new_sleep_sheet.get(&minute).unwrap();
                        target_minute = minute;
                        target_guard = watch.id;
                    }
                }
                // Add new guard sleep sheet to tracker
                sleep_sheets.insert(watch.id, new_sleep_sheet);
            }
        }
    }
    return target_guard * target_minute;
}

/// Extracts the minutes field from the two timestamps in the sleep period.
fn calculate_sleep_period_minutes(sleep_period: &(String, String)) -> (u64, u64) {
    // Create regex to extract minutes field
    let minute_regex = Regex::new(r"(.*?) (\d+):(\d+)").unwrap();
    for s_capture in minute_regex.captures_iter(&sleep_period.0) {
        // Find sleep start minute
        let sleep_minute = s_capture[3].parse::<u64>().unwrap();
        // Find sleep end minute (minute of awakening)
        for a_capture in minute_regex.captures_iter(&sleep_period.1) {
            let awake_minute = a_capture[3].parse::<u64>().unwrap();
            // Add the sleep period to the sleep sheet
            return (sleep_minute, awake_minute);
        }
    }
    panic!("D4 - should not get here!");
}

/// Creates a new guard sleep sheet with slot for each minute initialised to 0.
fn create_new_guard_sleep_sheet() -> HashMap<u64, u64> {
    let mut sleep_sheet: HashMap<u64, u64> = HashMap::new();
    for i in 0..60 {
        sleep_sheet.insert(i, 0);
    }
    return sleep_sheet;
}
