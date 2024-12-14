use std::collections::VecDeque;
use std::fs::read_to_string;
use std::str::FromStr;
use regex::Regex;

pub fn count_tokens_to_will_all(input_path: &str, multiplier: Option<usize>) -> i128 {
    let puzzle = extract_puzzle(input_path, multiplier);
    //solve_bfs(&puzzle)
    solve_with_diophantine(&puzzle)
}
#[allow(dead_code)]
fn solve_bfs(puzzle: &Vec<Game>) -> usize {
    puzzle.iter().fold(0, |mut acc, item| {
        if let Some(tokens) = token_to_get_prize(&item) {
            acc += tokens;
        }
        acc
    })
}

fn solve_with_diophantine(puzzle: &Vec<Game>) -> i128 {
    puzzle.iter().fold(0, |mut acc, item| {
        if let Some(tokens) = token_to_get_prize_diophantine(&item) {
            acc += tokens;
        }
        acc
    })
}

fn token_to_get_prize_diophantine(puzzle: &Game) -> Option<i128> {
    let ax = puzzle.button_a.x as i128;
    let bx = puzzle.button_b.x as i128;
    let ay = puzzle.button_a.y as i128;
    let by = puzzle.button_b.y as i128;
    let px = puzzle.prize.x as i128;
    let py = puzzle.prize.y as i128;

    let b = (py * ax - px * ay) / (by * ax - bx * ay);
    let a = (px - b * bx) / ax;
    if (ax * a + bx * b, ay * a + by * b) != (px, py) {
        return None;
    }
    Some(a * 3 + b)
}

fn extract_puzzle(input_game: &str, multiplier: Option<usize>) -> Vec<Game> {
    read_to_string(input_game)
        .unwrap()
        .split("\r\n\r\n")
        .map(|line| map_game(line.split("\r\n").collect(), multiplier))
        .collect()
}

fn map_game(game_str: Vec<&str>, multiplier: Option<usize>) -> Game {
    Game {
        button_a: read_button(game_str.get(0)),
        button_b: read_button(game_str.get(1)),
        prize: read_price(game_str.get(2), multiplier),
    }
}

fn read_price(price_str: Option<&&str>, multiplier: Option<usize>) -> Point {
    if price_str.is_none() {
        panic!("Button must not be empty");
    }
    let button_regex = Regex::new("X=(?<x>[0-9]{1,5}), Y=(?<y>[0-9]{1,5})").unwrap();
    let matched = button_regex.captures_iter(price_str.unwrap()).next().unwrap();
    let mut x: usize = FromStr::from_str(matched.name("x").unwrap().as_str()).unwrap();
    let mut y: usize = FromStr::from_str(matched.name("y").unwrap().as_str()).unwrap();

    if let Some(multiplier) = multiplier {
        x += multiplier;
        y += multiplier;
    }

    Point {
        x,
        y
    }
}

fn read_button(button_str: Option<&&str>) -> Point {
    if button_str.is_none() {
        panic!("Button must not be empty");
    }
    let button_regex = Regex::new("X\\+(?<x>[0-9]{1,2}), Y\\+(?<y>[0-9]{1,2})").unwrap();
    let matched = button_regex.captures_iter(button_str.unwrap()).next().unwrap();
    let x = FromStr::from_str(matched.name("x").unwrap().as_str()).unwrap();
    let y = FromStr::from_str(matched.name("y").unwrap().as_str()).unwrap();
    Point {
        x,
        y
    }
}
#[allow(dead_code)]
fn token_to_get_prize(game: &Game) -> Option<usize> {
    // Define a queue for BFS
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    // Start from the initial position
    queue.push_back((0, 0, 0)); // (position_x, position_y, token_count)

    while let Some((position_x, position_y, token_count)) = queue.pop_front() {
        // Check if the position is the prize
        if position_x == game.prize.x && position_y == game.prize.y {
            return Some(token_count);
        }

        // Avoid revisiting the same state
        if visited.contains(&(position_x, position_y)) {
            continue;
        }
        visited.insert((position_x, position_y));

        // End conditions
        if position_x > game.prize.x || position_y > game.prize.y || token_count > 10000000 {
            continue;
        }

        // Add possible moves by pressing button A
        let position_x_a = position_x + game.button_a.x;
        let position_y_a = position_y + game.button_a.y;
        if position_x_a <= game.prize.x && position_y_a <= game.prize.y {
            queue.push_back((position_x_a, position_y_a, token_count + 3));
        }

        // Add possible moves by pressing button B
        let position_x_b = position_x + game.button_b.x;
        let position_y_b = position_y + game.button_b.y;
        if position_x_b <= game.prize.x && position_y_b <= game.prize.y {
            queue.push_back((position_x_b, position_y_b, token_count + 1));
        }
    }

    // If no path was found
    None
}

#[derive(Debug, PartialEq)]
struct Game {
    button_a: Point,
    button_b: Point,
    prize: Point
}

#[derive(Debug, PartialEq)]

struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_token_when_pressing_two_times_second_one_time_first() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 24,
                y: 13
            }
        };
        assert_eq!(token_to_get_prize(&game), Some(5));
    }

    #[test]
    fn should_count_token_when_pressing_two_times_first_one_time_second() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 27,
                y: 11
            }
        };
        assert_eq!(token_to_get_prize(&game), Some(7));
    }

    #[test]
    fn should_count_token_when_pressing_one_time_each_button() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 17,
                y: 8
            }
        };
        assert_eq!(token_to_get_prize(&game), Some(4));
    }

    #[test]
    fn should_count_token_when_pressing_second_button_but_x_not_ok() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 13,
                y: 10
            }
        };
        assert_eq!(token_to_get_prize(&game), None);
    }

    #[test]
    fn should_count_token_when_pressing_second_button() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 14,
                y: 10
            }
        };
        assert_eq!(token_to_get_prize(&game), Some(2));
    }

    #[test]
    fn should_count_token_when_pressing_first_button_two_time_but_y_is_not_ok() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 20,
                y: 8
            }
        };
        assert_eq!(token_to_get_prize(&game), None);
    }

    #[test]
    fn should_count_token_when_pressing_first_button_two_time() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 20,
                y: 6
            }
        };
        assert_eq!(token_to_get_prize(&game), Some(6));
    }

    #[test]
    fn should_count_token_when_pressing_first_button_one_time() {
        let game = Game {
            button_a: Point {
                x: 10,
                y: 3
            },
            button_b: Point {
                x: 7,
                y: 5
            },
            prize: Point {
                x: 10,
                y: 3
            }
        };
        assert_eq!(token_to_get_prize(&game), Some(3));
    }

    #[test]
    fn should_extract_puzzle_with_multiplier() {
        assert_eq!(extract_puzzle("tests/resources/light_puzzle.txt", Some(100)),
                   vec![
                       Game {
                           button_a: Point {
                               x: 94,
                               y: 34
                           },
                           button_b: Point {
                               x: 22,
                               y: 67
                           },
                           prize: Point {
                               x: 8500,
                               y: 5500
                           }
                       },
                       Game {
                           button_a: Point {
                               x: 26,
                               y: 66
                           },
                           button_b: Point {
                               x: 67,
                               y: 21
                           },
                           prize: Point {
                               x: 12848,
                               y: 12276
                           }
                       }
                   ]
        );
    }

    #[test]
    fn should_extract_puzzle_without_multiplier() {
        assert_eq!(extract_puzzle("tests/resources/light_puzzle.txt", None),
            vec![
                Game {
                    button_a: Point {
                        x: 94,
                        y: 34
                    },
                    button_b: Point {
                        x: 22,
                        y: 67
                    },
                    prize: Point {
                        x: 8400,
                        y: 5400
                    }
                },
                Game {
                    button_a: Point {
                        x: 26,
                        y: 66
                    },
                    button_b: Point {
                        x: 67,
                        y: 21
                    },
                    prize: Point {
                        x: 12748,
                        y: 12176
                    }
                }
            ]
        );
    }
}