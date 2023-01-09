use core::num;
use std::error::Error;
use std::isize::MAX;
use std::path;

use crate::system::{BaseSymbols, SystemExpression};
use crate::turtle::{Lines, Turtle};
use plotters::element::PathElement;
use plotters::prelude::*;
use plotters::style::{RGBAColor, ShapeStyle};

const BLUE: RGBAColor = RGBAColor(0, 0, 255, 1.0);

const STYLE: ShapeStyle = ShapeStyle {
    color: BLUE,
    filled: true,
    stroke_width: 2,
};

const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGHT: u32 = 1000;

pub fn graph_system_expression(
    expression: SystemExpression,
    delta: f32,
) -> Result<(), Box<dyn Error>> {
    let mut turtle = Turtle::new(delta, 1.0);
    let lines = turtle.create_lines_from_expression(expression);
    create_graph_image(lines)?;
    Ok(())
}

fn create_graph_image(lines: Lines) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("test.png", (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let lines = translate_lines_to_positive_coords(lines);
    let max_x = calculate_max_x_from_lines(&lines);
    let max_y = calculate_max_y_from_lines(&lines);
    let chart_max = max_x.max(max_y);
    let mut chart =
        ChartBuilder::on(&root).build_cartesian_2d(0.0f32..chart_max, 0.0f32..chart_max)?;
    let elements = path_elements_from_lines(lines);
    for element in elements {
        chart.draw_series(std::iter::once(element))?;
    }
    root.present()?;
    Ok(())
}

fn path_elements_from_lines(lines: Lines) -> Vec<PathElement<(f32, f32)>> {
    lines
        .into_iter()
        .map(|line| PathElement::new(line, STYLE))
        .collect()
}

fn translate_lines_to_positive_coords(lines: Lines) -> Lines {
    let min_x = calculate_min_x_from_lines(&lines);
    let min_y = calculate_min_y_from_lines(&lines);
    lines
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|coord| (coord.0 + min_x.abs(), coord.1 + min_y.abs()))
                .collect()
        })
        .collect()
}

fn calculate_max_x_from_lines(lines: &Lines) -> f32 {
    lines.iter().fold(0.0, |max, line| {
        let line_max = line.iter().map(|c| c.0).reduce(f32::max).unwrap();
        match line_max > max {
            true => line_max,
            false => max,
        }
    })
}

fn calculate_max_y_from_lines(lines: &Lines) -> f32 {
    lines.iter().fold(0.0, |max, line| {
        let line_max = line.iter().map(|c| c.1).reduce(f32::max).unwrap();
        match line_max > max {
            true => line_max,
            false => max,
        }
    })
}

fn calculate_min_x_from_lines(lines: &Lines) -> f32 {
    lines.iter().fold(0.0, |min, line| {
        let line_min = line.iter().map(|c| c.0).reduce(f32::min).unwrap();
        match line_min < min {
            true => line_min,
            false => min,
        }
    })
}

fn calculate_min_y_from_lines(lines: &Lines) -> f32 {
    lines.iter().fold(0.0, |min, line| {
        let line_min = line.iter().map(|c| c.1).reduce(f32::min).unwrap();
        match line_min < min {
            true => line_min,
            false => min,
        }
    })
}
