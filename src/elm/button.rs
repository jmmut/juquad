use crate::draw::{draw_rect, draw_rect_lines};
use crate::elm::style::Style;
use crate::elm::text::Text;
use crate::elm::widget::{
    Interactable, Renderable, RenderableWidget, Widget, WidgetTrait, Widgets,
};
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::lazy::{Margin, Pad};
use crate::widgets::{interact, Interaction};

pub type Button<I> = Widget<ButtonBase<I>, I>;
pub type RenderButton<I> = fn(widget: &Button<I>, interaction: Interaction);

pub struct ButtonBase<I> {
    pub interaction: Interaction,
    pub input: Box<dyn InputTrait>,
    pub render_button: RenderButton<I>,
    pub on_press: I,
}

impl<I: Clone + 'static> Button<I> {
    pub fn new<Sty: Into<Style>>(
        style: Sty,
        on_press: I,
        children: Widgets<I>,
    ) -> Box<dyn RenderableWidget<I>> {
        Box::new(Self::new_raw(style, on_press, children))
    }
    pub fn new_raw<Sty: Into<Style>>(style: Sty, on_press: I, children: Widgets<I>) -> Self {
        Self::new_generic(
            style.into(),
            Box::new(InputMacroquad),
            render_interactive,
            on_press,
            children,
        )
    }
    pub fn new_text<Sty: Into<Style>>(
        style: Sty,
        on_press: I,
        text: &str,
    ) -> Box<dyn RenderableWidget<I>> {
        Box::new(Self::new_text_raw(style, on_press, text))
    }
    pub fn new_text_raw<Sty: Into<Style>>(style: Sty, on_press: I, text: &str) -> Self {
        let mut button_style = style.into();
        let mut text_style = button_style.clone();
        button_style.pad = Pad::new_symmetric(0.0);
        text_style.margin = Margin::new_symmetric(0.0);
        Self::new_raw(button_style, on_press, vec![Text::new(text_style, text)])
    }
    pub fn new_generic(
        style: Style,
        input: Box<dyn InputTrait>,
        render_button: RenderButton<I>,
        on_press: I,
        children: Widgets<I>,
    ) -> Self {
        let custom = ButtonBase {
            interaction: Interaction::None,
            input,
            render_button,
            on_press,
        };
        Self {
            pos: Default::default(),
            size: None,
            style,
            custom,
            children,
        }
    }
    pub fn interact_raw(&mut self) -> Interaction {
        self.custom.interaction = interact(self.rect(), &self.custom.input);
        self.custom.interaction
    }

    pub fn interaction(&self) -> Interaction {
        self.custom.interaction
    }
}

impl<I: Clone + 'static> Interactable<I> for Button<I> {
    fn interact(&mut self) -> Vec<I> {
        let mut messages = Vec::new();
        if self.interact_raw().is_clicked() {
            messages.push(self.custom.on_press.clone());
        }
        messages
    }
}

impl<I> Renderable for Button<I> {
    fn render_interactive(&self, interaction: Interaction) {
        (self.custom.render_button)(self, interaction)
    }
}

fn render_interactive<I>(widget: &Button<I>, _unused: Interaction) {
    let state_style = widget.style().coloring.choose(widget.custom.interaction);
    draw_rect(widget.rect(), state_style.bg_color);
    draw_rect_lines(
        widget.rect(),
        widget.style.border * 2.0,
        state_style.border_color,
    );
    // if unsafe { DEBUG_WIDGETS } {
    //     draw_debug_widget(widget);
    // }
    for child in widget.children() {
        child.render_interactive(widget.custom.interaction);
    }
}
