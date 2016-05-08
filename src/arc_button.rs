// This code is based on MIT-licensed button widget code from Conrod
// found at https://github.com/PistonDevelopers/conrod/blob/master/src%2Fwidget%2Fbutton.rs

use conrod::{
    Backend, Color, Colorable, CommonBuilder, FontSize, IndexSlot, Labelable, Sizeable,
    Positionable, Text, UpdateArgs, Widget, WidgetKind, FramedRectangle, Polygon, Point
};
use conrod::events::InputProvider;
use gen_arc;


/// A pressable button widget whose reaction is triggered upon release.
pub struct ArcButton<'a, F> {
    common: CommonBuilder,
    maybe_label: Option<&'a str>,
    /// The reaction for the ArcButton. The reaction will be triggered upon release of the ArcButton.
    maybe_react: Option<F>,
    /// Unique styling for the ArcButton.
    pub style: Style,
}

/// Unique kind for the widget.
pub const KIND: WidgetKind = "ArcButton";

widget_style!{
    KIND;
    /// Unique styling for the ArcButton.
    style Style {
        /// Color of the ArcButton's pressable area.
        - color: Color { theme.shape_color }
        /// The color of the ArcButton's label.
        - label_color: Color { theme.label_color }
        /// The font size of the ArcButton's label.
        - label_font_size: FontSize { theme.font_size_medium }
    }
}

/// Represents the state of the ArcButton widget.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    rectangle_idx: IndexSlot,
    label_idx: IndexSlot,
}

impl<'a, F> ArcButton<'a, F> {

    /// Create a ArcButton context to be built upon.
    pub fn new() -> Self {
        ArcButton {
            common: CommonBuilder::new(),
            maybe_react: None,
            maybe_label: None,
            style: Style::new(),
        }
    }

    builder_methods!{
        pub react { maybe_react = Some(F) }
    }
}


impl<'a, F> Widget for ArcButton<'a, F>
    where F: FnOnce(),
{
    type State = State;
    type Style = Style;

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }

    fn unique_kind(&self) -> WidgetKind {
        KIND
    }

    fn init_state(&self) -> State {
        State {
            rectangle_idx: IndexSlot::new(),
            label_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the ArcButton.
    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, style, rect, mut ui, .. } = args;

        let button_color = {
            let input = ui.widget_input(idx);
            if input.mouse_left_click().is_some() {
                self.maybe_react.map(|react_function| react_function());
            }

            let style_color = style.color(ui.theme());
            input.mouse_left_button_down().map(|_| {
                style_color.clicked()
            }).or_else(|| {
                input.maybe_mouse_position().map(|_| style_color.highlighted())
            }).unwrap_or(style_color)
        };

        // FramedRectangle widget.
        let rectangle_idx = state.view().rectangle_idx.get(&mut ui);
        let (xy, dim) = rect.xy_dim();
        let points = gen_arc::ArcIter::new(xy, rect.w()/4.0, rect.w()/2.0, 0.1, 4.2);
        let ptvec : Vec<Point> = points.clone().collect();
        println!("{:?}", ptvec);
        Polygon::fill(points)
            .wh(dim)
            .xy(xy)
            .graphics_for(idx)
            .color(button_color)
            .set(rectangle_idx, &mut ui);

        // Label widget.
        if let Some(label) = self.maybe_label {
            let label_idx = state.view().label_idx.get(&mut ui);
            let color = style.label_color(ui.theme());
            let font_size = style.label_font_size(ui.theme());
            Text::new(label)
                .middle_of(rectangle_idx)
                .graphics_for(idx)
                .color(color)
                .font_size(font_size)
                .set(label_idx, &mut ui);
        }

    }

}


impl<'a, F> Colorable for ArcButton<'a, F> {
    builder_method!(color { style.color = Some(Color) });
}

impl<'a, F> Labelable<'a> for ArcButton<'a, F> {
    builder_methods!{
        label { maybe_label = Some(&'a str) }
        label_color { style.label_color = Some(Color) }
        label_font_size { style.label_font_size = Some(FontSize) }
    }
}
