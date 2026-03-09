use crate::client::screens::Screen;
use crate::client::screens::screen_main_menu::ScreenMainMenu;
use crate::client::settings::Settings;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame};
use rust_i18n::i18n;
use std::io;
use std::time::Duration;

mod screens;
mod settings;

pub fn run() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

struct UserContext {
    settings: Settings,
}

impl UserContext {
    fn new() -> UserContext {
        UserContext {
            settings: Settings::load_from_disk(),
        }
    }
}
struct App {
    should_close: bool,
    screen: Box<dyn Screen>,
    user_context: UserContext,
}

impl App {
    fn new() -> App {
        App {
            should_close: false,
            screen: Box::new(ScreenMainMenu::new()),
            user_context: UserContext::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.init();
        while !self.should_close {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn init(&mut self) {
        rust_i18n::set_locale(self.user_context.settings.language.as_str());
    }

    fn render(&mut self, frame: &mut Frame) {
        self.screen.render(&self.user_context, frame);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if !event::poll(Duration::from_millis(50))? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(event) => {
                if let Some(c) = self.screen.on_key_input(&mut self.user_context, event) {
                    c(self);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.should_close = true;
    }
}
