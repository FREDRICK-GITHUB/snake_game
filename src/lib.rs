
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


#[derive(PartialEq)]
enum Direction {
    Up, 
    Right,
    Down,
    Left,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) ->Snake {
        Snake { 
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Right,
        }
    }
}


#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        World { 
            width,
            size: width * width,
            snake: Snake::new(snake_index)
         }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_index = self.snake_head_index();
        let  row = snake_index / self.width;
        let  column = snake_index % self.width;
        

        if self.snake.direction == Direction::Right {
            let next_column = (column + 1) % self.width;
            self.snake.body[0].0 = (row * self.width) + next_column;
        }
        if self.snake.direction == Direction::Left {
            let next_column = (column - 1) % self.width;
            self.snake.body[0].0 = (row * self.width) + next_column;
        }


        if self.snake.direction == Direction::Up {
            let next_row = (row - 1) % self.width;
            self.snake.body[0].0 = (next_row * self.width) + column;
        }
        if self.snake.direction == Direction::Down {
            let next_row = (row + 1) % self.width;
            self.snake.body[0].0 = (next_row * self.width) + column;
        }

    }
}