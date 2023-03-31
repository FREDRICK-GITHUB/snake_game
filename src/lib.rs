use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
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
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_index, 3),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 { return; }

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
        let next_cell = self.generate_next_snake_cell(&self.snake.direction);
        self.snake.body[0] = next_cell;

        let length = self.snake.body.len();

        for i in 1..length {
            self.snake.body[i] = SnakeCell(temporary_array[i - 1].0);
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
