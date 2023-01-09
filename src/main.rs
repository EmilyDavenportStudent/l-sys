mod system;
use std::f32::consts::PI;

use system::System;

mod turtle;
use turtle::Turtle;

mod graph;
use graph::graph_system_expression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = System::from_axiom("F");
    s.add_rule("F=>F+F-F-F-G+F+F+F-F");
    s.add_rule("G=>GGG");
    graph_system_expression(s.do_n_iterations(4), 90.0)?;
    Ok(())
}
