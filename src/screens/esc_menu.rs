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
            self.esc_help_open = false;
            self.settings_session = None;
        }

        let panel = Rect::new(ui_width() * 0.5 - 290.0, 28.0, 580.0, 620.0);
        soft_panel(panel);
        label(
            if self.esc_help_open {
                "Help"
            } else if self.esc_settings_open {
                "Settings"
            } else {
                "Menu"
            },
            panel.x + 28.0,
            panel.y + 48.0,
            30,
            TEXT_BRIGHT,
        );

        if self.esc_help_open {
            self.draw_esc_help(panel);
        } else if self.esc_settings_open {
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
            "Help",
            true,
            ButtonTone::Ghost,
        ) {
            self.esc_help_open = true;
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

        if self.status_timer > 0.0 || status_is_critical(&self.status) {
            label("Recent feedback", button_x, panel.y + 410.0, 16, TEXT_DIM);
            draw_wrapped_text(&self.status, button_x, panel.y + 438.0, button_w, 16, TEXT);
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

    fn draw_esc_help(&mut self, panel: Rect) {
        let copy = match self.screen {
            crate::screens::Screen::Auction => {
                "Auction: tap RAISE for one step, JUMP once for a double step, WAIT & READ ROOM for a rival tell, or WALK AWAY to protect the plan. Tap a rival for details."
            }
            crate::screens::Screen::PropertyDetail(_) => {
                "Research: buy a visible tier, choose a walk-away style, adjust the cap with -10k or +10k, then tap REGISTER TO BID. Tap READ REPORT for the complete findings."
            }
            crate::screens::Screen::Portfolio => {
                "Portfolio: select a holding. Resolve a due repair or rent review first; otherwise lease, improve, hold, or open Finance and Sale preparation."
            }
            _ => {
                "Scout: tap INSPECT to read a listing. Recover: use See Listings, address the selected holding, or Advance Week when no blocker remains."
            }
        };
        draw_wrapped_text(
            copy,
            panel.x + 30.0,
            panel.y + 104.0,
            panel.w - 60.0,
            18,
            TEXT,
        );
        label(
            "Help can be reopened from Menu at any time.",
            panel.x + 30.0,
            panel.y + 300.0,
            16,
            TEXT_DIM,
        );
        if button(
            Rect::new(
                panel.x + 30.0,
                panel.y + panel.h - 68.0,
                panel.w - 60.0,
                48.0,
            ),
            "BACK TO MENU",
            true,
            ButtonTone::Secondary,
        ) {
            self.esc_help_open = false;
        }
    }
}

fn status_is_critical(status: &str) -> bool {
    [
        "Load failed:",
        "Settings failed:",
        "Need ",
        "Not enough",
        "Finish ",
        "Repair ",
        "Resolve ",
        "The bank",
        "This week's",
    ]
    .iter()
    .any(|prefix| status.starts_with(prefix))
}
