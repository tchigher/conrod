//!
//! A simple demonstration of how to instantiate an `Image` widget.
//!

#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;


#[cfg(feature = "backend-piston_window")]
fn main() {
    use conrod::{Canvas, Colorable, Image, Positionable, Theme, Widget, color};
    use conrod::backend::piston_window::Ui;
    use piston_window::{EventLoop, Flip, Glyphs, PistonWindow, Texture, TextureSettings,
                        UpdateEvent, WindowSettings};
    use std::sync::Arc;

    // Construct the window.
    let window: PistonWindow =
        WindowSettings::new("Image Widget Demonstration", [800, 600])
            .exit_on_esc(true).vsync(true).samples(4).build().unwrap();

    // Get the path to our `assets` directory (where the fonts and images are).
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    // construct our `Ui`.
    let mut ui = {
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    // The texture to use for the `Image`.
    let rust_logo = {
        let path = assets.join("images/rust.png");
        let factory = &mut *window.factory.borrow_mut();
        Arc::new(Texture::from_path(factory, &path, Flip::None, &TextureSettings::new()).unwrap())
    };

    // Poll events from the window.
    for event in window.ups(60) {
        ui.handle_event(&event);
        event.draw_2d(|c, g| ui.draw_if_changed(c, g));
        event.update(|_| ui.set_widgets(|mut ui| {
            widget_ids!(CANVAS, RUST_LOGO);
            Canvas::new().color(color::LIGHT_BLUE).set(CANVAS, &mut ui);
            Image::from_texture(rust_logo.clone())
                .middle_of(CANVAS)
                .set(RUST_LOGO, &mut ui);
        }));
    }
}


#[cfg(not(feature = "backend-piston_window"))]
fn main() {
    println!("This example requires the \"backend-piston_window\" feature. Use the feature like so \
              `cargo run --release --features \"backend-piston_window\" --example <example_name>`");
}
