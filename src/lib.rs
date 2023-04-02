use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern {
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
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {

        let size = width * width;
        let snake = Snake::new(snake_index, 3);
        let mut reward_cell;

        loop {
            reward_cell = random(size);
            if !snake.body.contains(&SnakeCell(reward_cell)) { break; };
        }



        World {
            width,
            size,
            snake,
            next_cell: None,
            reward_cell,
        }
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

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 { return; }

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
        let temporary_array = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            },
            None => {
                self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
            }
        }

        let length = self.snake.body.len();

        for i in 1..length {
            self.snake.body[i] = SnakeCell(temporary_array[i - 1].0);
        }

        if self.reward_cell == self.snake_head_index() {
            self.snake.body.push(SnakeCell(self.snake.body[1].0));
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
                }else{
                    SnakeCell(snake_index + 1)
                }
            },
            Direction::Left => {
                let threshold = row * self.width;
                if snake_index == threshold {
                    SnakeCell(threshold + (self.width - 1))
                }else{
                    SnakeCell(snake_index - 1)
                }
            },
            Direction::Up => {
                let threshold =  snake_index - (row  * self.width);
                if snake_index == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                }else{
                    SnakeCell(snake_index - self.width)
                }
            },
            Direction::Down => {
                let threshold = snake_index + ((self.width - row) * self.width);
                if snake_index + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                }else{
                    SnakeCell(snake_index + self.width)
                }
            },

            // This implementation works as the match above but more expensive in terms of complexity
            // Direction::Right => SnakeCell((row * self.width) + (snake_index + 1) % self.width),
            // Direction::Left => SnakeCell((row * self.width) + (snake_index - 1) % self.width),
            // Direction::Up => SnakeCell((snake_index - self.width) % self.size),
            // Direction::Down => SnakeCell((snake_index + self.width) % self.size),
        };
    }
}
