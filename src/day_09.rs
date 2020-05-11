use std::collections::HashMap;
use std::collections::VecDeque;

use regex::Regex;

/// This struct is used to represent an instance of the elf marble game described in AoC 2018 Day 9.
struct MarbleGame {
    num_players: u64,
    last_marble_points: u64,
    last_marble_placed: u64,
    marbles: VecDeque<u64>,
    current_marble: usize,
    current_player: u64,
    player_scores: HashMap<u64, u64>,
}

impl MarbleGame {
    /// Creates a new instance of the MarbleGame.
    ///
    /// Current player is set to 1 (the first player up). The last marble played is set to 0, which
    /// is added to the circle prior to the first player taking their first turn. All player scores
    /// are initialised to 0.
    pub fn new(num_players: u64, last_marble_points: u64) -> Self {
        Self {
            num_players: num_players,
            last_marble_points: last_marble_points,
            last_marble_placed: 0,
            marbles: {
                let mut marbs = VecDeque::new();
                marbs.push_front(0);
                marbs
            },
            current_marble: 0,
            current_player: 1,
            player_scores: {
                let mut scores = HashMap::<u64, u64>::new();
                for id in 1..num_players + 1 {
                    scores.insert(id, 0);
                }
                scores
            },
        }
    }

    /// Gets the maximum score across all player scores.
    pub fn get_winning_score(&self) -> u64 {
        return *self.player_scores.values().max().unwrap();
    }

    /// Plays out the marble game. Returns false if they game had not already been played.
    /// Otherwise, returns true.
    pub fn play_game(&mut self) -> bool {
        if self.last_marble_placed == self.last_marble_points {
            return true;
        }
        loop {
            // Check if we are at the end of the game
            if self.last_marble_placed == self.last_marble_points {
                return false;
            }
            // Increment the last marble placed
            self.last_marble_placed += 1;
            // if self.last_marble_placed % 100000 == 0 {
            //     println!(
            //         "D9_P2 - Placing marble {} out of {}...",
            //         self.last_marble_placed, self.last_marble_points
            //     );
            // }
            // Check if we are at special case of multiple-of-23
            if self.last_marble_placed % 23 == 0 {
                // Update score of current player
                *self.player_scores.get_mut(&self.current_player).unwrap() +=
                    self.last_marble_placed;
                // Get index of marble seven to counter-clockwise
                let marb_to_remove = {
                    if self.current_marble < 7 {
                        let delta = 7 - self.current_marble - 1;
                        self.marbles.len() - 1 - delta
                    } else {
                        self.current_marble - 7
                    }
                };
                // Update the current marble index
                if marb_to_remove == self.marbles.len() - 1 {
                    self.current_marble = 0;
                } else {
                    self.current_marble = marb_to_remove;
                }
                // Get score of marb to remove and add to score of current player
                let rem_mark_score = self.marbles.remove(marb_to_remove).unwrap();
                *self.player_scores.get_mut(&self.current_player).unwrap() += rem_mark_score;
            // Print out updated score
            //// println!("[{} // {}] Score update [{}] - {}", self.last_marble_placed, self.last_marble_points, self.current_player, self.player_scores.get(&self.current_player).unwrap());
            } else {
                // Determine index to insert marble at
                let index = (self.current_marble + 2) % self.marbles.len();
                self.marbles.insert(index, self.last_marble_placed);
                self.current_marble = index;
            }
            // Go to next player - go back to first player after last player has turn
            self.current_player = {
                let next_player = self.current_player + 1;
                if next_player > self.num_players {
                    1
                } else {
                    next_player
                }
            }
        }
    }
}

#[aoc_generator(day9)]
fn generate_input(input: &str) -> (u64, u64) {
    // Trim leading and trailing whitespace from input
    let input = input.trim();
    // Create regex to extract marble game parameters
    let input_regex = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    // Extract marble game parameters from input
    for capture in input_regex.captures_iter(input) {
        let num_players = capture[1].parse::<u64>().unwrap();
        let last_marble_points = capture[2].parse::<u64>().unwrap();
        return (num_players, last_marble_points);
    }
    panic!("Day9_gen - should not get here!");
}

#[aoc(day9, part1)]
fn solve_part_1(marble_game_params: &(u64, u64)) -> u64 {
    let mut marble_game = MarbleGame::new(marble_game_params.0, marble_game_params.1);
    marble_game.play_game();
    return marble_game.get_winning_score();
}

#[aoc(day9, part2)]
fn solve_part_2(marble_game_params: &(u64, u64)) -> u64 {
    let mut marble_game = MarbleGame::new(marble_game_params.0, marble_game_params.1 * 100);
    marble_game.play_game();
    return marble_game.get_winning_score();
}
