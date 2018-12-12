mod activity;

use std::collections::HashMap;
use std::io::{self, Read};

type Minute = u8;
type GuardId = u32;
type ActivityRecord = HashMap<Minute, u32>;
type ActivityTable = HashMap<GuardId, ActivityRecord>;
type Activity = activity::Activity;

struct ActivitySummary {
    id: GuardId,
    total_mins_sleep: u32,
    max_freq_sleep_min: u8,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let sequence = parse_input(&input);
    let analyzed_input = analyze_sequence(&sequence);

    println!("{}", part1(&analyzed_input));
}

fn parse_input(input: &str) -> Vec<Activity> {
    let mut sorted_input: Vec<_> = input.lines().collect();
    sorted_input.sort_unstable();

    sorted_input.iter().map(|l| l.parse().unwrap()).collect()
}

fn analyze_sequence(sequence: &[Activity]) -> ActivityTable {
    let mut table = ActivityTable::new();
    let mut current_guard: Option<&u32> = None;
    let mut last_sleep_min: Option<&u8> = None;

    for entry in sequence {
        if let activity::Activity::BeginShift(id) = entry {
            current_guard = Some(id);
            last_sleep_min = None;

            continue;
        }

        if let activity::Activity::FallAsleep(cur_minute) = entry {
            assert_eq!(true, current_guard.is_some());
            last_sleep_min = Some(cur_minute);

            continue;
        }

        if let activity::Activity::WakeUp(cur_minute) = entry {
            assert_eq!(true, current_guard.is_some());
            assert_eq!(true, last_sleep_min.is_some());

            let start_sleep = *last_sleep_min.unwrap();

            for m in start_sleep..*cur_minute {
                let activity_record = table
                    .entry(*current_guard.unwrap())
                    .or_insert_with(ActivityRecord::new);

                *activity_record.entry(m).or_default() += 1;
            }

            last_sleep_min = None;

            continue;
        }
    }

    table
}

fn part1(table: &ActivityTable) -> u32 {
    let mut current_most_asleep: Option<ActivitySummary> = None;

    for id in table.keys() {
        let summary = compute_record(*id, &table[id]);

        current_most_asleep = match current_most_asleep {
            None => Some(summary),
            Some(old_summary) => {
                if summary.total_mins_sleep > old_summary.total_mins_sleep {
                    Some(summary)
                } else {
                    Some(old_summary)
                }
            }
        }
    }

    let summary = current_most_asleep.unwrap();

    summary.id * u32::from(summary.max_freq_sleep_min)
}

fn compute_record(id: GuardId, record: &ActivityRecord) -> ActivitySummary {
    let mut total_mins_sleep = 0;
    let mut max_freq_sleep_min = 0;
    let mut max_frequency = 0;

    for min in record.keys() {
        let freq_at_min = record[min];

        total_mins_sleep += freq_at_min;

        if freq_at_min > max_frequency {
            max_freq_sleep_min = *min;
            max_frequency = freq_at_min;
        }
    }

    ActivitySummary {
        id,
        total_mins_sleep,
        max_freq_sleep_min,
    }
}
