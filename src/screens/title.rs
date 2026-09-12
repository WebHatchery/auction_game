use crate::app::App;
use crate::ui::*;
use macroquad::prelude::*;

const SHADE: Color = Color::new(0.025, 0.028, 0.030, 0.40);

impl App {
    pub(crate) fn draw_title_screen(&mut self) {
        draw_title_background(&self.title_background);
        draw_rectangle(0.0, 0.0, ui_width(), ui_height(), SHADE);

        if self.title_settings_open {
            self.draw_title_settings();
        } else {
            self.draw_title_menu();
            self.draw_title_status();
        }
    }

    fn draw_title_menu(&mut self) {
        let button_x = 92.0;
        let button_w = 328.0;
        let mut y = 318.0;

        if button(
            Rect::new(button_x, y, button_w, 48.0),
            "Start A Portfolio",
            true,
            ButtonTone::Primary,
        ) {
            self.start_new_game();
        }
        y += 62.0;
        if button(
            Rect::new(button_x, y, button_w, 48.0),
            "Load Game",
            true,
            ButtonTone::Secondary,
        ) {
            self.load_game_from_title();
        }
        y += 62.0;
        if button(
            Rect::new(button_x, y, button_w, 48.0),
            "Settings",
            true,
            ButtonTone::Ghost,
        ) {
            self.title_settings_open = true;
            self.open_settings();
        }
        y += 62.0;
        if button(
            Rect::new(button_x, y, button_w, 48.0),
            "Exit Game",
            true,
            ButtonTone::Danger,
        ) {
            macroquad::miniquad::window::quit();
        }
    }

    fn draw_title_settings(&mut self) {
        let panel = Rect::new(58.0, 28.0, 650.0, 630.0);
        soft_panel(panel);
        label("Settings", panel.x + 28.0, panel.y + 48.0, 30, TEXT_BRIGHT);
        self.draw_settings_editor(Rect::new(
            panel.x + 24.0,
            panel.y + 72.0,
            panel.w - 48.0,
            panel.h - 88.0,
        ));
    }

    fn draw_title_status(&self) {
        let is_error = self.status.starts_with("Load failed:");
        let panel = Rect::new(92.0, 574.0, 520.0, 72.0);
        soft_panel(panel);
        label(
            if is_error { "Load status" } else { "Desk note" },
            panel.x + 16.0,
            panel.y + 23.0,
            16,
            if is_error { NEGATIVE } else { ACCENT },
        );
        label_fit(
            &self.status,
            panel.x + 16.0,
            panel.y + 50.0,
            panel.w - 32.0,
            16,
            TEXT_BRIGHT,
        );
    }
}

fn draw_title_background(texture: &Texture2D) {
    let texture_size = texture.size();
    let scale = (ui_width() / texture_size.x).max(ui_height() / texture_size.y);
    let dest_size = vec2(texture_size.x * scale, texture_size.y * scale);
    let x = (ui_width() - dest_size.x) * 0.5;
    let y = (ui_height() - dest_size.y) * 0.5;

    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            ..Default::default()
        },
    );
}
