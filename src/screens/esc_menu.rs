use crate::app::App;
use crate::ui::*;
use macroquad::prelude::*;

const MENU_SHADE: Color = Color::new(0.020, 0.022, 0.024, 0.68);

impl App {
    pub(crate) fn draw_esc_menu(&mut self) {
        draw_rectangle(0.0, 0.0, ui_width(), ui_height(), MENU_SHADE);
        if button(
            Rect::new(16.0, 12.0, 72.0, 44.0),
            "Close",
            true,
            ButtonTone::Secondary,
        ) {
            self.esc_menu_open = false;
            self.esc_settings_open = false;
            self.settings_session = None;
        }

        let panel = Rect::new(ui_width() * 0.5 - 290.0, 28.0, 580.0, 620.0);
        soft_panel(panel);
        label(
            if self.esc_settings_open {
                "Settings"
            } else {
                "Menu"
            },
            panel.x + 28.0,
            panel.y + 48.0,
            30,
            TEXT_BRIGHT,
        );

        if self.esc_settings_open {
            self.draw_esc_settings(panel);
        } else {
            self.draw_esc_actions(panel);
        }
    }

    fn draw_esc_actions(&mut self, panel: Rect) {
        let button_x = panel.x + 36.0;
        let button_w = panel.w - 72.0;
        let mut y = panel.y + 88.0;

        if button(
            Rect::new(button_x, y, button_w, 44.0),
            "Save",
            true,
            ButtonTone::Primary,
        ) {
            self.save_game();
        }
        y += 56.0;
        if button(
            Rect::new(button_x, y, button_w, 44.0),
            "Load",
            true,
            ButtonTone::Secondary,
        ) {
            self.load_game();
        }
        y += 56.0;
        if button(
            Rect::new(button_x, y, button_w, 44.0),
            "Settings",
            true,
            ButtonTone::Ghost,
        ) {
            self.esc_settings_open = true;
            self.open_settings();
        }
        y += 56.0;
        if button(
            Rect::new(button_x, y, button_w, 44.0),
            "Menu",
            true,
            ButtonTone::Ghost,
        ) {
            self.return_to_title();
        }
        y += 56.0;
        if button(
            Rect::new(button_x, y, button_w, 44.0),
            "Exit",
            true,
            ButtonTone::Danger,
        ) {
            macroquad::miniquad::window::quit();
        }
    }

    fn draw_esc_settings(&mut self, panel: Rect) {
        self.draw_settings_editor(Rect::new(
            panel.x + 24.0,
            panel.y + 72.0,
            panel.w - 48.0,
            panel.h - 88.0,
        ));
    }
}
