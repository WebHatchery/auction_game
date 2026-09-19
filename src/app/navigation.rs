//! Navigation yields to the active auction room.
use super::App;
use crate::screens::Screen;
use crate::ui::*;
use macroquad::prelude::*;

impl App {
    pub(super) fn draw_header(&mut self) {
        let in_auction = self.screen == Screen::Auction;
        draw_rectangle(
            0.0,
            0.0,
            ui_width(),
            if in_auction { 44.0 } else { 68.0 },
            PANEL_DARK,
        );
        if button(
            Rect::new(16.0, if in_auction { 0.0 } else { 12.0 }, 72.0, 44.0),
            "Menu",
            true,
            ButtonTone::Ghost,
        ) {
            self.esc_menu_open = !self.esc_menu_open;
            self.esc_settings_open = false;
        }
        if in_auction {
            return;
        }
        label("Auction House Tycoon", 94.0, 42.0, 30, TEXT_BRIGHT);
        label(&format!("Week {}", self.week), 430.0, 41.0, 19, TEXT_DIM);
        label(
            &format!("Registrations {}/2", self.auction_registrations),
            520.0,
            41.0,
            16,
            if self.auction_registrations > 0 {
                POSITIVE
            } else {
                WARNING
            },
        );

        let mut x = ui_width() - 422.0;
        let nav_enabled = self
            .current_auction
            .as_ref()
            .map(|auction| !auction.is_running())
            .unwrap_or(true);

        if button(
            Rect::new(x, 12.0, 130.0, 44.0),
            "Recover",
            nav_enabled,
            ButtonTone::Ghost,
        ) {
            self.screen = Screen::Dashboard;
        }
        x += 142.0;
        if button(
            Rect::new(x, 12.0, 120.0, 44.0),
            "Scout",
            nav_enabled,
            ButtonTone::Ghost,
        ) {
            self.screen = Screen::PropertyList;
        }
        x += 132.0;
        if button(
            Rect::new(x, 12.0, 120.0, 44.0),
            "Portfolio",
            nav_enabled,
            ButtonTone::Ghost,
        ) {
            self.screen = Screen::Portfolio;
        }
    }

    pub(super) fn draw_status_bar(&self) {
        let rect = Rect::new(0.0, ui_height() - 40.0, ui_width(), 40.0);
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, PANEL_DARK);
        label(&self.status, 28.0, ui_height() - 15.0, 17, TEXT_DIM);
    }
}
