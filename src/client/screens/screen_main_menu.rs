use crate::client::screens::Screen;
use crate::client::settings::Settings;
use crate::client::{App, UserContext, settings};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Flex;
use ratatui::prelude::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Color, Constraint, Layout, Line, Stylize};
use ratatui::style::Color::Gray;
use ratatui::text::{ToLine, ToText};
use ratatui::widgets::{Block, Paragraph, RatatuiMascot};
use rust_i18n::t;

pub struct ScreenMainMenu {
    chosen: usize,
}

impl ScreenMainMenu {
    pub fn new() -> ScreenMainMenu {
        ScreenMainMenu { chosen: 0 }
    }
}

impl Screen for ScreenMainMenu {
    fn render(&mut self, user_context: &UserContext, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(5), Constraint::Min(5)]).split(frame.area());
        let banner_chunk = Layout::horizontal([Constraint::Length(41)])
            .flex(Flex::Center)
            .split(chunks[0]);
        let s = "\
'|| '||'  '|'   |     '||''|.   '||''|.
 '|. '|.  .'   |||     ||   ||   ||   ||
  ||  ||  |   |  ||    ||''|'    ||...|'
   ||| |||   .''''|.   ||   |.   ||
    |   |   .|.  .||. .||.  '|' .||.    ";
        frame.render_widget(Paragraph::new(s.to_text().blue()), banner_chunk[0]);
        let subchunks = Layout::vertical([Constraint::Length(3), Constraint::Min(5)])
            .margin(1)
            .flex(Flex::Center)
            .split(chunks[1]);
        let name_chunk = Layout::horizontal([Constraint::Length(22)])
            .flex(Flex::Center)
            .split(subchunks[0]);
        let block = Block::bordered();
        frame.render_widget(Block::bordered(), name_chunk[0]);
        frame.render_widget(
            Line::from(user_context.settings.nickname.clone()).bold(),
            block.inner(name_chunk[0]),
        );
        let button_chunks = Layout::horizontal([Constraint::Max(30)])
            .flex(Flex::Center)
            .split(subchunks[1]);
        let button_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(button_chunks[0]);
        let button_keys = [
            "ui.main_menu.singleplayer",
            "ui.main_menu.multiplayer",
            "ui.main_menu.deck_edit",
            "ui.main_menu.language",
            "ui.main_menu.quit",
        ];
        for i in 0..button_keys.len() {
            let mut text = t!(button_keys[i]).to_string();
            if self.chosen == i && i == 3 {
                text = format!("◀  {}  ▶", text);
            }
            let mut line = Line::from(text).bold().centered();
            if self.chosen == i {
                line = line.black().bg(Gray);
            }
            frame.render_widget(line, button_chunks[i]);
        }
    }

    fn on_key_input(
        &mut self,
        user_context: &mut UserContext,
        event: KeyEvent,
    ) -> Option<Box<dyn FnOnce(&mut App)>> {
        if event.is_release() {
            return None;
        }
        match event.code {
            KeyCode::Char(c) if c.is_ascii_alphanumeric() || c == '_' => {
                if user_context.settings.nickname.len() < 12 {
                    user_context.settings.nickname.push(c);
                    user_context.settings.save_to_disk();
                }
            }
            KeyCode::Backspace => {
                if user_context.settings.nickname.len() > 0 {
                    user_context.settings.nickname.pop();
                    user_context.settings.save_to_disk();
                }
            },
            KeyCode::Up if self.chosen > 0 => self.chosen -= 1,
            KeyCode::Down if self.chosen < 4 => self.chosen += 1,
            KeyCode::Enter =>{
                match self.chosen {
                    0=>{}
                    1=>{}
                    2=>{}
                    4=>{
                        return Some(Box::new(|app|app.exit()));
                    }
                    _=>{}
                }
            }
            KeyCode::Left if self.chosen == 3 => {
                let index = settings::LANGUAGES
                    .iter()
                    .position(|&x| x == user_context.settings.language.as_str());
                let index = if let Some(index) = index {
                    if index == 0 {
                        settings::LANGUAGES.len() - 1
                    } else {
                        index - 1
                    }
                } else {
                    0
                };
                user_context.settings.language = settings::LANGUAGES[index].to_string();
                user_context.settings.save_to_disk();
                rust_i18n::set_locale(settings::LANGUAGES[index]);
            }
            KeyCode::Right if self.chosen == 3 => {
                let index = settings::LANGUAGES
                    .iter()
                    .position(|&x| x == user_context.settings.language.as_str());
                let index = if let Some(index) = index {
                    if index == settings::LANGUAGES.len() - 1 {
                        0
                    } else {
                        index + 1
                    }
                } else {
                    0
                };
                user_context.settings.language = settings::LANGUAGES[index].to_string();
                user_context.settings.save_to_disk();
                rust_i18n::set_locale(settings::LANGUAGES[index]);
            }
            _ => {}
        }
        None
    }
}
