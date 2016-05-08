#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate vecmath;
mod arc_button;
mod gen_arc;

pub fn main() {
    use conrod::{self, Colorable, Labelable, Positionable, Sizeable, Widget};
    use piston_window::{EventLoop, Glyphs, PistonWindow, OpenGL, UpdateEvent, WindowSettings};
    use arc_button::ArcButton;

    // Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
    type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
    type Ui = conrod::Ui<Backend>;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // PistonWindow has two type parameters, but the default type is
    // PistonWindow<T = (), W: Window = GlutinWindow>. To change the Piston backend,
    // specify a different type in the let binding, e.g.
    // let window: PistonWindow<(), Sdl2Window>.
    let mut window: PistonWindow = WindowSettings::new("Control Panel", [1200, 800])
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

    while let Some(e) = window.next() {
        // Pass each `Event` to the `Ui`.
        ui.handle_event(&e);

        e.update(|_| ui.set_widgets(|ref mut ui| {

            // Sets a color to clear the background with before the Ui draws our widget.
            conrod::Canvas::new().color(conrod::color::DARK_RED).set(BACKGROUND, ui);

            // The `widget_ids` macro is a easy, safe way of generating unique `WidgetId`s.
            widget_ids! {
                // An ID for the background widget, upon which we'll place our custom button.
                BACKGROUND,
                // The WidgetId we'll use to plug our widget into the `Ui`.
                ARC_BUTTON,
            }

            // Create an instance of our custom widget.
            ArcButton::new()
                .color(conrod::color::rgb(0.0, 0.3, 0.1))
                .middle_of(BACKGROUND)
                .w_h(256.0, 256.0)
                .label_color(conrod::color::WHITE)
                .label("Circular Button")
                // This is called when the user clicks the button.
                .react(|| println!("Click"))
                // Add the widget to the conrod::Ui. This schedules the widget it to be
                // drawn when we call Ui::draw.
                .set(ARC_BUTTON, ui);
        }));

        // Draws the whole Ui (in this case, just our widget) whenever a change occurs.
        window.draw_2d(&e, |c, g| ui.draw_if_changed(c, g));
    }
}
