//! jitsi-meet model — self-hosted video conferencing.

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub subject: String,
    pub participants: Vec<String>,
    pub recording: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub rooms: Vec<Room>,
}

impl Default for Model {
    fn default() -> Self { Self { rooms: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_room(&mut self, name: &str, subject: &str) -> bool {
        if !self.rooms.iter().any(|r| r.name == name) {
            self.rooms.push(Room { name: name.into(), subject: subject.into(), participants: vec![], recording: false });
            true
        } else { false }
    }

    pub fn join(&mut self, room: &str, user: &str) -> bool {
        if let Some(r) = self.rooms.iter_mut().find(|r| r.name == room) {
            r.participants.push(user.into());
            true
        } else { false }
    }

    pub fn start_recording(&mut self, room: &str) -> bool {
        if let Some(r) = self.rooms.iter_mut().find(|r| r.name == room) { r.recording = true; true } else { false }
    }

    pub fn list_rooms(&self) -> &[Room] { &self.rooms }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn room_join_record() {
        let mut j = Model::new();
        assert!(j.create_room("meeting", "Weekly"));
        assert!(j.join("meeting", "alice"));
        assert!(j.start_recording("meeting"));
    }
}
