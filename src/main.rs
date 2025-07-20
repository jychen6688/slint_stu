use slint::PhysicalPosition;

slint::include_modules!();
fn main() {
    let app = APP::new().unwrap();
    let app_drage_window_weak = app.as_weak();
    app.global::<DragWindow>()
        .on_drag_window(move |delta_x, delta_y| {
            let app = app_drage_window_weak.unwrap();
            let window = app.window();
            let scale = window.scale_factor();

            let position = window.position();
            let x = position.x + (delta_x * scale) as i32;
            let y = position.y + (delta_y * scale) as i32;

            window.set_position(PhysicalPosition::new(x, y));
        });

    app.global::<WindowStore>().on_close(|| {
        let _ = slint::quit_event_loop();
    });

    let app_weak = app.as_weak();
    app.global::<WindowStore>().on_min(move || {
        let app = app_weak.unwrap();
        let window = app.window();
        window.set_minimized(true);
    });

    let app_weak = app.as_weak();
    app.global::<WindowStore>().on_max(move || {
        let app = app_weak.unwrap();
        let window = app.window();
        // 切换 实现最大化：切换全屏/最大化
        // if window.is_fullscreen() {
        //     window.set_fullscreen(false); // 切换回正常
        // } else {
        //     window.set_fullscreen(true); // 进入全屏
        // }
        window.set_maximized(true);
    });

    app.run().unwrap();
}
