use crate::client::screens::Screen;
use crate::client::{App, UserContext};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Flex;
use ratatui::prelude::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Constraint, Layout, Line, Stylize};
use ratatui::text::ToText;
use ratatui::widgets::{Block, Paragraph};

pub struct ScreenMainMenu {
    chosen: u8,
}

impl ScreenMainMenu {
    pub fn new() -> ScreenMainMenu {
        ScreenMainMenu { chosen: 0 }
    }
}

impl Screen for ScreenMainMenu {
    fn render(&mut self, user_context: &UserContext, frame: &mut Frame) {
        let chunks =
            Layout::new(Vertical, [Constraint::Length(5), Constraint::Min(5)]).split(frame.area());
        frame.render_widget(
            Paragraph::new(
                "\
'|| '||'  '|'     |     '||''|.   '||''|.
 '|. '|.  .'     |||     ||   ||   ||   ||
  ||  ||  |     |  ||    ||''|'    ||...|'
   ||| |||     .''''|.   ||   |.   ||
    |   |     .|.  .||. .||.  '|' .||.     "
                    .to_text()
                    .blue(),
            )
            .centered(),
            chunks[0],
        );
        let subchunks = Layout::new(Vertical, [Constraint::Length(3), Constraint::Min(4)])
            .margin(1)
            .flex(Flex::Center)
            .split(chunks[1]);
        let name_chunk = Layout::new(Horizontal, [Constraint::Length(22)])
            .flex(Flex::Center)
            .split(subchunks[0]);
        let block = Block::bordered();
        frame.render_widget(Block::bordered(), name_chunk[0]);
        frame.render_widget(
            Line::from(user_context.settings.nickname.clone()).bold(),
            block.inner(name_chunk[0]),
        );
        let button_chunks = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ],
        )
        .split(subchunks[1]);
        frame.render_widget(
            Line::from("Singleplayer").bold().centered(),
            button_chunks[0],
        );
        frame.render_widget(
            Line::from("Multiplayer").bold().centered(),
            button_chunks[1],
        );
        frame.render_widget(Line::from("Deck Edit").bold().centered(), button_chunks[2]);
        frame.render_widget(Line::from("Quit").bold().centered(), button_chunks[3]);
    }

    fn on_key_input(
        &mut self,
        user_context: &mut UserContext,
        event: KeyEvent,
    ) -> Option<Box<dyn FnOnce(&mut App)>> {
        if event.is_release(){
            return None;
        }
        match event.code {
            KeyCode::Up => self.chosen = (self.chosen - 1).clamp(0, 3),
            KeyCode::Down => self.chosen = (self.chosen + 1).clamp(0, 3),
            KeyCode::Char(c) if c.is_ascii_alphanumeric() || c == '_' => {
                if user_context.settings.nickname.len() < 12 {
                    user_context.settings.nickname.push(c);
                }
            }
            KeyCode::Backspace => {
                if user_context.settings.nickname.len() > 0 {
                    user_context.settings.nickname.pop();
                }
            }
            _ => {}
        }
        None
    }
}
