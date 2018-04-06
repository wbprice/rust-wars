use piston_window::*;

fn main() {
    let mut window : PistonWindow = WindowSettings::new("Rust Wars", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            rectangle([0.52, 0.80, 0.92, 1.0],
                      [0.0, 0.0, 640.0, 480.0],
                      context.transform,
                      graphics);

            rectangle([1.0, 1.0, 1.0, 1.0],
                      [270.0, 190.0, 100.0, 100.0],
                      context.transform,
                      graphics)
        });
    }
}
