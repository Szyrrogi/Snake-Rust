extern crate piston_window;

use piston_window::*;
use std::collections::LinkedList;
use std::iter::FromIterator;
use std::io;

const GRID_SIZE: f64 = 20.0;

type Position = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: LinkedList<Position>,
    direction: Direction,
}

impl Snake {
    fn new(start_pos: Position) -> Self {
        let mut body = LinkedList::new();
        body.push_back(start_pos);
        body.push_back((start_pos.0, start_pos.1 - 1));
        body.push_back((start_pos.0, start_pos.1 - 2));

        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn move_forward(&mut self, grid_width: i32, grid_height: i32, wrap_around: bool, portals: &Option<(Position, Position)>) -> bool {
        let &head = self.body.front().expect("Snake has no body!");
        let mut new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        if let Some((portal_a, portal_b)) = portals {
            if new_head == *portal_a {
                new_head = *portal_b;
            } else if new_head == *portal_b {
                new_head = *portal_a;
            }
        }

        if wrap_around {
            new_head.0 = (new_head.0 + grid_width) % grid_width;
            new_head.1 = (new_head.1 + grid_height) % grid_height;
        } else if new_head.0 < 0 || new_head.0 >= grid_width || new_head.1 < 0 || new_head.1 >= grid_height {
            return false;
        }

        self.body.push_front(new_head);
        self.body.pop_back();
        true
    }

    fn grow(&mut self) {
        if let Some(&tail) = self.body.back() {
            self.body.push_back(tail);
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn check_collision(&self) -> bool {
        let &head = self.body.front().expect("Snake has no head!");
        self.body.iter().skip(1).any(|&pos| pos == head)
    }
}

struct Food {
    position: Position,
}

impl Food {
    fn new(x: i32, y: i32) -> Self {
        Food { position: (x, y) }
    }

    fn relocate(&mut self, snake1: &Snake, snake2: &Snake, grid_width: i32, grid_height: i32) {
        loop {
            let new_position = (
                (rand::random::<u32>() % grid_width as u32) as i32,
                (rand::random::<u32>() % grid_height as u32) as i32,
            );

            if !snake1.body.contains(&new_position) && !snake2.body.contains(&new_position) {
                self.position = new_position;
                break;
            }
        }
    }
}

struct Rock {
    position: Position,
}

impl Rock {
    fn new_random(grid_width: i32, grid_height: i32, snake1: &Snake, snake2: &Snake, food: &Food) -> Self {
        loop {
            let position = (
                (rand::random::<u32>() % grid_width as u32) as i32,
                (rand::random::<u32>() % grid_height as u32) as i32,
            );

            if !snake1.body.contains(&position)
                && !snake2.body.contains(&position)
                && position != food.position
            {
                return Rock { position };
            }
        }
    }
}

struct GameState {
    snake1: Snake,
    snake2: Snake,
    food: Food,
    rocks: Vec<Rock>,
    portal_positions: Option<(Position, Position)>,
    game_over: bool,
    winner: Option<String>,
}

impl GameState {
    fn new(grid_width: i32, grid_height: i32, enable_portals: bool) -> Self {
        let snake1 = Snake::new((5, 5));
        let snake2 = Snake::new((15, 15));
        let food = Food::new(10, 10);

        let num_rocks = get_number_of_rocks();
        let rocks = (0..num_rocks)
            .map(|_| Rock::new_random(grid_width, grid_height, &snake1, &snake2, &food))
            .collect();


        let portal_positions = if enable_portals {
            let portal_a = (
                (rand::random::<u32>() % grid_width as u32) as i32,
                (rand::random::<u32>() % grid_height as u32) as i32,
            );
            let portal_b = (
                (rand::random::<u32>() % grid_width as u32) as i32,
                (rand::random::<u32>() % grid_height as u32) as i32,
            );
            Some((portal_a, portal_b))
        } else {
            None
        };

        GameState {
            snake1,
            snake2,
            food,
            rocks,
            portal_positions,
            game_over: false,
            winner: None,
        }
    }

    fn check_snake_collisions(&mut self) {
        let head1 = self.snake1.body.front().unwrap();
        let head2 = self.snake2.body.front().unwrap();

        if self.snake1.body.contains(head2) {
            self.game_over = true;
            self.winner = Some("Wygrał Gracz 1!".to_string());
        } else if self.snake2.body.contains(head1) {
            self.game_over = true;
            self.winner = Some("Wygrał Gracz 2!".to_string());
        } else if head1 == head2 {
            self.game_over = true;
            self.winner = Some("Remis!".to_string());
        }

        for rock in &self.rocks {
            if head1 == &rock.position {
                self.game_over = true;
                self.winner = Some("Wygrał Gracz 2!".to_string());
            } else if head2 == &rock.position {
                self.game_over = true;
                self.winner = Some("Wygrał Gracz 1!".to_string());
            }
        }
    }
}

fn read_input() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<i32>().expect("Please enter a valid number")
}


fn get_grid_size() -> (i32, i32) {
    println!("Wprowadź szerokość mapy: ");
    let grid_width = read_input();

    println!("Wprowadź wysokość mapy: ");
    let grid_height = read_input();

    (grid_width, grid_height)
}

fn get_number_of_rocks() -> usize {
    println!("Wyprowadź ilość kamieni: ");
    let num_rocks = read_input();
    num_rocks as usize
}


fn choose_wrap_around() -> bool {
    println!("Czy ma występować przechodzenie przez ściany? (yes/no): ");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Wprowadź 'yes' albo 'no'."),
        }
    }
}

fn choose_enable_portals() -> bool {
    println!("Czy mają występować portale? (yes/no): ");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Wprowadź 'yes' albo 'no'."),
        }
    }
}

fn main() {
    let (grid_width, grid_height) = get_grid_size();
    let wrap_around = choose_wrap_around();
    let enable_portals = choose_enable_portals();

    let mut window: PistonWindow = WindowSettings::new("Snake", [
        (GRID_SIZE * grid_width as f64) as u32,
        (GRID_SIZE * grid_height as f64) as u32,
    ])
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = GameState::new(grid_width, grid_height, enable_portals);
    let mut last_update = std::time::Instant::now();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let new_direction_snake1 = match key {
                Key::Up if !matches!(game.snake1.direction, Direction::Down) => Some(Direction::Up),
                Key::Down if !matches!(game.snake1.direction, Direction::Up) => Some(Direction::Down),
                Key::Left if !matches!(game.snake1.direction, Direction::Right) => Some(Direction::Left),
                Key::Right if !matches!(game.snake1.direction, Direction::Left) => Some(Direction::Right),
                _ => None,
            };

            if let Some(dir) = new_direction_snake1 {
                game.snake1.change_direction(dir);
            }

            let new_direction_snake2 = match key {
                Key::W if !matches!(game.snake2.direction, Direction::Down) => Some(Direction::Up),
                Key::S if !matches!(game.snake2.direction, Direction::Up) => Some(Direction::Down),
                Key::A if !matches!(game.snake2.direction, Direction::Right) => Some(Direction::Left),
                Key::D if !matches!(game.snake2.direction, Direction::Left) => Some(Direction::Right),
                _ => None,
            };

            if let Some(dir) = new_direction_snake2 {
                game.snake2.change_direction(dir);
            }
        }

        if last_update.elapsed().as_millis() > 150 {
            if !game.game_over {
                if !game.snake1.move_forward(grid_width, grid_height, wrap_around, &game.portal_positions) {
                    game.game_over = true;
                    game.winner = Some("Wygrał Gracz 2!".to_string());
                }

                if !game.snake2.move_forward(grid_width, grid_height, wrap_around, &game.portal_positions) {
                    game.game_over = true;
                    game.winner = Some("Wygrał Gracz 1!".to_string());
                }

                if game.snake1.body.front().unwrap() == &game.food.position {
                    game.snake1.grow();
                    game.food.relocate(&game.snake1, &game.snake2, grid_width, grid_height);
                }

                if game.snake2.body.front().unwrap() == &game.food.position {
                    game.snake2.grow();
                    game.food.relocate(&game.snake1, &game.snake2, grid_width, grid_height);
                }

                game.check_snake_collisions();
            }

            last_update = std::time::Instant::now();
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for &pos in &game.snake1.body {
                rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    [
                        pos.0 as f64 * GRID_SIZE,
                        pos.1 as f64 * GRID_SIZE,
                        GRID_SIZE,
                        GRID_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }

            for &pos in &game.snake2.body {
                rectangle(
                    [0.0, 0.0, 1.0, 1.0],
                    [
                        pos.0 as f64 * GRID_SIZE,
                        pos.1 as f64 * GRID_SIZE,
                        GRID_SIZE,
                        GRID_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }

            for rock in &game.rocks {
                rectangle(
                    [0.5, 0.5, 0.5, 1.0],
                    [
                        rock.position.0 as f64 * GRID_SIZE,
                        rock.position.1 as f64 * GRID_SIZE,
                        GRID_SIZE,
                        GRID_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }

            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [
                    game.food.position.0 as f64 * GRID_SIZE,
                    game.food.position.1 as f64 * GRID_SIZE,
                    GRID_SIZE,
                    GRID_SIZE,
                ],
                c.transform,
                g,
            );

            if let Some((portal_a, portal_b)) = game.portal_positions {
                rectangle(
                    [0.0, 1.0, 1.0, 1.0],
                    [
                        portal_a.0 as f64 * GRID_SIZE,
                        portal_a.1 as f64 * GRID_SIZE,
                        GRID_SIZE,
                        GRID_SIZE,
                    ],
                    c.transform,
                    g,
                );
                rectangle(
                    [0.0, 1.0, 1.0, 1.0],
                    [
                        portal_b.0 as f64 * GRID_SIZE,
                        portal_b.1 as f64 * GRID_SIZE,
                        GRID_SIZE,
                        GRID_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }

            if game.game_over {
                if let Some(winner) = &game.winner {
                    println!("{}", winner);
                }
            }
        });
    }
}
