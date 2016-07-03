#[macro_use] extern crate conrod;
#[macro_use] extern crate gfx;
extern crate find_folder;
extern crate piston_window;
extern crate vecmath;
extern crate cgmath;
extern crate walkdir;
extern crate syntect;
mod particle_renderer;
mod layout;

use std::path::Path;
use std::env;

pub fn main() {
    use conrod::{self, Colorable, Labelable, Positionable, Sizeable, Widget, Button};
    use piston_window::{EventLoop, Glyphs, PistonWindow, OpenGL, UpdateEvent, WindowSettings, Window, Event, Input, Motion};

    // Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
    type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
    type Ui = conrod::Ui<Backend>;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // PistonWindow has two type parameters, but the default type is
    // PistonWindow<T = (), W: Window = GlutinWindow>. To change the Piston backend,
    // specify a different type in the let binding, e.g.
    // let window: PistonWindow<(), Sdl2Window>.
    let mut window: PistonWindow = WindowSettings::new("Control Panel", [1300, 1000])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    // Conrod's main object.
    let mut ui = {
        // Load a font. `Glyphs` is provided to us via piston_window and gfx, though you may use
        // any type that implements `CharacterCache`.
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone()).unwrap();
        Ui::new(glyph_cache, conrod::Theme::default())
    };

    window.set_ups(60);

    let args: Vec<String> = env::args().collect();
    let path : &str = if args.len() >= 2 {
        &args[1]
    } else {
        "src"
    };
    let layout = layout::layout_dir(Path::new(path));
    let mut particle_renderer = particle_renderer::ParticleRenderer::new(
        &mut window.factory, window.output_color.clone(), window.window.draw_size(), &layout);

    while let Some(e) = window.next() {
        // Pass each `Event` to the `Ui`.
        ui.handle_event(&e);

        if let Event::Input(Input::Move(Motion::MouseScroll(x,y))) = e {
            particle_renderer.scroll_canvas(x as f32,y as f32);
        }

        e.update(|_| ui.set_widgets(|ref mut ui| {
            // The `widget_ids` macro is a easy, safe way of generating unique `WidgetId`s.
            widget_ids! {
                // An ID for the background widget, upon which we'll place our custom button.
                // BACKGROUND,
                // The WidgetId we'll use to plug our widget into the `Ui`.
                ZOOM_IN_BUTTON,
                ZOOM_OUT_BUTTON,
            }

            // Create an instance of our custom widget.
            Button::new()
                .color(conrod::color::rgb(0.0, 0.3, 0.1))
                .top_left_with_margins(10.0, 10.0)
                .w_h(100.0, 50.0)
                .label_color(conrod::color::WHITE)
                .label("Zoom in")
                .react(|| particle_renderer.zoom(1.5))
                .set(ZOOM_IN_BUTTON, ui);
            Button::new()
                .color(conrod::color::rgb(0.0, 0.3, 0.1))
                .right(10.0)
                .w_h(100.0, 50.0)
                .label_color(conrod::color::WHITE)
                .label("Zoom out")
                .react(|| particle_renderer.zoom(0.66))
                .set(ZOOM_OUT_BUTTON, ui);
        }));

        window.draw_3d(&e, |w| particle_renderer.render(&mut w.encoder));
        // Draws the whole Ui (in this case, just our widget) whenever a change occurs.
        window.draw_2d(&e, |c, g| ui.draw(c, g));
    }
}
