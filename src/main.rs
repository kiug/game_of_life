use raylib::prelude::*;

mod grid;
mod simulation;
use simulation::*;

fn main() {
    const GRID_SIZE: u16 = 480;
    const CELL_SIZE: u16 = 8;
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Game of Life")
        .build();

    rl.set_target_fps(8);

    let mut simulation = Simulation::new(GRID_SIZE, GRID_SIZE, CELL_SIZE);

    while !rl.window_should_close() {
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text(format!("FPS {}", fps).as_str(), 490, 10, 20, Color::GRAY);

        simulation.update();
        simulation.draw(d);
    }
}
