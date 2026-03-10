use crate::draw::draw_rect;
use crate::elm::style::Style;
use crate::elm::text::Text;
use crate::elm::widget::{Interactable, Renderable, Widget, WidgetTrait, Widgets};
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::widgets::button::draw_panel_border;
use crate::widgets::{interact, Interaction};

pub type Button<I> = Widget<ButtonBase<I>, I>;
pub type RenderButton<I> = fn(widget: &Button<I>, interaction: Interaction);
pub type OnPress<I> = fn(interaction: Interaction) -> I;

pub struct ButtonBase<I> {
    pub interaction: Interaction,
    pub input: Box<dyn InputTrait>,
    pub render_button: RenderButton<I>,
    pub on_press: I,
}

impl<I: Clone + 'static> Button<I> {
    pub fn new_text<Sty: Into<Style>>(
        style: Sty,
        text: &str,
        on_press: I,
    ) -> Self {
        let style = style.into();
        let text = Text::<I>::new(&style, text);
        let text = Box::new(text);
        Self::new_generic(
            style,
            Box::new(InputMacroquad),
            render_interactive,
            on_press,
            vec![text],
        )
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

impl<I> Renderable for Button<I> {
    fn render_interactive(&self, interaction: Interaction) {
        (self.custom.render_button)(self, interaction)
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
fn render_interactive<I>(widget: &Button<I>, _unused: Interaction) {
    let state_style = widget.style().coloring.choose(widget.custom.interaction);
    draw_rect(widget.rect(), state_style.bg_color);
    draw_panel_border(widget.rect(), state_style);
    // if unsafe { DEBUG_WIDGETS } {
    //     draw_debug_widget(widget);
    // }
    for child in widget.children() {
        child.render_interactive(widget.custom.interaction);
    }
}
