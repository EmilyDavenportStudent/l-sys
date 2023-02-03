use std::f32::consts::PI;

use crate::system::{BaseSymbols, SystemExpression};

#[derive(Debug, Clone, Copy)]
pub struct Turtle {
    delta_radians: f32,
    line_length: f32,
    current_angle: f32,
}

pub type Lines = Vec<Vec<(f32, f32)>>;

impl Turtle {
    pub fn new(delta: f32, line_length: f32) -> Self {
        Self {
            delta_radians: delta.to_radians(),
            line_length,
            current_angle: PI / 2.0,
        }
    }

    pub fn create_lines_from_expression(&mut self, expression: SystemExpression) -> Lines {
        let mut lines = Vec::<Vec<(f32, f32)>>::new();
        let mut current_line = vec![(0.0, 0.0)];
        for instruction in expression.0 {
            match instruction {
                BaseSymbols::AngleIncrement => self.increment_angle(),
                BaseSymbols::AngleDecrement => self.decrement_angle(),
                BaseSymbols::Forward => {
                    let coord = self.create_next_coordinate(&mut current_line);
                    current_line.push(coord);
                }
                BaseSymbols::ForwardNoLine => {
                    let coord = self.create_next_coordinate(&mut current_line);
                    lines.push(current_line);
                    current_line = vec![coord];
                }
                BaseSymbols::NoAction => (),
            }
        }
        lines.push(current_line);
        lines
    }

    fn increment_angle(&mut self) {
        self.current_angle += self.delta_radians;
    }

    fn decrement_angle(&mut self) {
        self.current_angle -= self.delta_radians;
    }

    fn create_next_coordinate(&self, line: &mut Vec<(f32, f32)>) -> (f32, f32) {
        let x = line.last().unwrap().0 + (self.line_length * (self.current_angle.cos()));
        let y = line.last().unwrap().1 + (self.line_length * (self.current_angle.sin()));
        (x, y)
    }
}
