use iced::{
    advanced::{layout, mouse, renderer, widget, Layout, Widget},
    Element, Length, Size,
};

use crate::{dmi_model::ParsedState, screens::debugger::StateboxSettings};

/// Statebox - widget, that displays 1 icon state according to StateboxSettings.
#[derive(Debug, Clone)]
pub struct Statebox<'a> {
    pub state: &'a ParsedState,
    pub settings: StateboxSettings,

    pub statebox_settings: StateboxSettings,
}

impl<'a, Message, Theme, Renderer> From<Statebox<'a>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(widget: Statebox<'a>) -> Self {
        Self::new(widget)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Statebox<'a>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        todo!()
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let header: Column<Message> = if self.settings.debug {
            column![
                Space::new(1, 3),
                row![text("State: "), bold_text(state.name.clone())],
                Space::new(1, 3),
                text!("Delay: {:?}", self.state.delay),
                text!("Frames: {}", self.state.frames),
                text!("Directions: {}", self.state.dirs.len()),
                text!("Looping: {:?}", self.state.loop_flag),
                text!("Movement: {}", self.state.movement),
                text!("Rewind: {}", self.state.rewind),
                Space::new(1, 10)
            ]
            .padding(5)
            .spacing(5)
        } else {
            column![bold_text(state.name.clone()), Space::new(1, 10)]
                .padding(5)
                .spacing(5)
                .align_x(Horizontal::Center)
        };

        let display: Grid<Message> = {
            let mut dirs: VecDeque<GridRow<Message>> = self
                .state
                .dirs
                .iter()
                .map(|(direction, _)| {
                    let mut row: GridRow<Message> = GridRow::default();
                    row = row.push(text(direction.to_string()));
                    if self.settings.animated {
                        let animated = {
                            if self.settings.show_resized {
                                self.state.get_animated(direction)
                            } else {
                                self.state.get_original_animated(direction)
                            }
                        };
                        if animated.is_some() {
                            let gif = Gif::new(&animated.unwrap().frames);
                            let gif = button(gif)
                                .on_press(wrap![DebuggerMessage::CopyImage(
                                    self.state.name.clone(),
                                    true,
                                    self.settings.show_resized,
                                    direction.clone(),
                                    None
                                )])
                                .style(|_theme, _status| button::Style {
                                    background: None,
                                    ..Default::default()
                                });
                            row = row.push(gif);
                        }
                    } else {
                        for frame in 0..state.frames {
                            let icon = {
                                if self.settings.show_resized {
                                    self.state
                                        .get_frame(direction, frame as usize)
                                } else {
                                    self.state.get_original_frame(
                                        direction,
                                        frame as usize,
                                    )
                                }
                            };
                            if icon.is_some() {
                                let icon = icon.unwrap();
                                let image_widget: Image = Image::new(
                                    iced::widget::image::Handle::from_rgba(
                                        icon.width(),
                                        icon.height(),
                                        icon.clone().into_bytes(),
                                    ),
                                );
                                let image_widget = button(image_widget)
                                    .on_press(wrap![
                                        DebuggerMessage::CopyImage(
                                            self.state.name.clone(),
                                            false,
                                            self.settings.show_resized,
                                            direction.clone(),
                                            Some(frame as usize)
                                        )
                                    ])
                                    .style(|_theme, _status| button::Style {
                                        background: None,
                                        ..Default::default()
                                    });
                                row = row.push(image_widget);
                            } else {
                                row = row.push(text("?"));
                            }
                        }
                    }
                    row
                })
                .collect();
            if !self.settings.animated && self.state.frames > 1 {
                let mut delay_row: GridRow<Message> = GridRow::new();
                delay_row = delay_row.push(text("Delay"));
                for delay in state.delay.as_ref().unwrap_or(&Vec::new()) {
                    delay_row = delay_row.push(text(delay))
                }
                dirs.push_front(delay_row);
            }
            Grid::with_rows(dirs.into())
                .column_width(self.parsed_dmi.displayed_width as f32 * 1.2)
                .horizontal_alignment(Horizontal::Center)
                .spacing(10)
        };

        let display = Scrollable::with_direction(
            display,
            Direction::Horizontal(Scrollbar::default()),
        );
        container(column![header, display])
            .padding(10)
            .style(|_theme| Style {
                text_color: Some(settings.text_color),
                background: Some(Background::Color(settings.background_color)),
                border: Border {
                    color: Color::BLACK,
                    width: 2.0,
                    radius: Radius::new(5),
                },
                shadow: Shadow::default(),
            })
    }
}
