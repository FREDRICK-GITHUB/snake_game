use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        let snake = Snake::new(snake_index, 3);
        let size = width * width;

        World {
            width,
            size,
            reward_cell: World::generate_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
        }
    }

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            };
        }

        reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have Won!"),
            Some(GameStatus::Lost) => String::from("You have Lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No Status"),
        }
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // cannot return a reference to JS because of borrowing rules
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //     self.snake.body
    // }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let temporary_array = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
                    }
                }

                let length = self.snake.body.len();

                for i in 1..length {
                    self.snake.body[i] = SnakeCell(temporary_array[i - 1].0);
                }

                if self.reward_cell == self.snake_head_index() {
                    if self.snake_length() < self.size {
                        self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = 1000;
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        }
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;

        return match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_index + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
            Direction::Left => {
                let threshold = row * self.width;
                if snake_index == threshold {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Up => {
                let threshold = snake_index - (row * self.width);
                if snake_index == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                let threshold = snake_index + ((self.width - row) * self.width);
                if snake_index + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
        };
    }
}
