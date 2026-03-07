pub struct Settings {
    pub nickname: String,
}

impl Settings {
    pub fn new() -> Settings {
        Settings::default()
    }

    pub fn save_to_disk(&mut self){

    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            nickname: "Player".to_string(),
        }
    }
}
