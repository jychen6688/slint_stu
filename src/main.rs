use slint::PhysicalPosition;

slint::include_modules!();
fn main() {
    let app = APP::new().unwrap();
    let app_drage_window_weak = app.as_weak();
    app.on_drag_window(move |delta_x, delta_y| {
        let app = app_drage_window_weak.unwrap();
        let window = app.window();
        let scale = window.scale_factor();

        let position = window.position();
        let x = position.x + (delta_x * scale) as i32;
        let y = position.y + (delta_y * scale) as i32;

        window.set_position(PhysicalPosition::new(x, y));
    });

    app.run().unwrap();
}
