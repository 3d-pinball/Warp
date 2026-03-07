use crate::client::{App, UserContext};
use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;

pub mod screen_main_menu;

pub trait Screen {
    fn render(&mut self, user_context: &UserContext, frame: &mut Frame);

    fn on_key_input(
        &mut self,
        user_context: &mut UserContext,
        event: KeyEvent,
    ) -> Option<Box<dyn FnOnce(&mut App)>> {
        None
    }
}
