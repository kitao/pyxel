use crate::sound::Sound;

impl Sound {
    pub fn mml(&mut self, mml_str: &str) {
        if mml_str.is_empty() {
            self.commands.clear();
            return;
        }

        let is_old_mml = true;
        if is_old_mml {
            self.old_mml(mml_str);
            return;
        }

        // TODO
    }
}
