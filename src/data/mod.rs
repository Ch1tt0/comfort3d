pub(crate) struct GameInfo {
    name: String,
    description: String,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            name: String::from("Comfort3D"),
            description: String::from(""), // TODO!
        }
    }
}

impl GameInfo {
    pub fn get_game_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_game_description(&self) -> String {
        self.description.clone()
    }
}
