use crate::{app, timer};
use iced::Element;
use iced::widget::{column, row, text, text_input};

#[derive(Default)]
pub struct SetupView;

impl SetupView {
    pub fn view<'view>(
        &'view self,
        settings: &'view timer::Settings,
    ) -> Element<'view, app::Message> {
        column![
            row![
                text("Study time (in seconds):"),
                text_input("Enter time here...", &settings.time_to_study.to_string()).on_input(
                    |new_timer| {
                        app::Message::Setup(timer::Settings {
                            time_to_study: new_timer
                                .parse::<u64>()
                                .unwrap_or(settings.time_to_study),
                            ..*settings
                        })
                    }
                )
            ],
            row![
                text("Relax time (in seconds):"),
                text_input(
                    "Enter relax time here...",
                    &settings.time_to_relax.to_string()
                )
                .on_input(|new_timer| {
                    app::Message::Setup(timer::Settings {
                        time_to_relax: new_timer.parse::<u64>().unwrap_or(settings.time_to_relax),
                        ..*settings
                    })
                })
            ]
        ]
        .into()
    }
}
