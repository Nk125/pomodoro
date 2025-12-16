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
        let display_study_time = match settings.time_to_study {
            0 => "",
            n => &n.to_string(),
        };

        let display_relax_time = match settings.time_to_relax {
            0 => "",
            n => &n.to_string(),
        };

        column![
            row![
                text("Study time (in seconds):"),
                text_input("Enter time here...", display_study_time).on_input(|new_timer| {
                    app::Message::Setup(timer::Settings {
                        time_to_study: crate::ui::num_input::num_input(
                            settings.time_to_study,
                            &new_timer,
                        ),
                        ..*settings
                    })
                })
            ],
            row![
                text("Relax time (in seconds):"),
                text_input("Enter relax time here...", display_relax_time).on_input(|new_timer| {
                    app::Message::Setup(timer::Settings {
                        time_to_relax: crate::ui::num_input::num_input(
                            settings.time_to_relax,
                            &new_timer,
                        ),
                        ..*settings
                    })
                })
            ]
        ]
        .into()
    }
}
