
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

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let snake_index = self.snake_head_index();
        let  (row, column) = self.index_to_cell(snake_index);

        let (row, column) = match self.snake.direction {
            Direction::Right => {
                (row, (column + 1) % self.width)
            },
            Direction::Left => {
                 (row, (column - 1) % self.width)
            },
            Direction::Up => {
                ((row - 1) % self.width, column)
            },
            Direction::Down => {
                ((row + 1) % self.width, column)
            }
        };

        let next_index = self.cell_to_index( row, column);
        self.set_snake_head(next_index);

    }

    fn set_snake_head(&mut self, index: usize){
        self.snake.body[0].0 = index;
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize){
        (index / self.width, index % self.width)
    }

    fn cell_to_index(&self, row: usize, column: usize) -> usize{
        (row * self.width) + column
    }
}