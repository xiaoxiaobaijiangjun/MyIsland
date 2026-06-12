use std::time::{Duration, Instant};

pub struct WaterReminder {
    last_reminder: Instant,
    pub active: bool,
    pub title: String,
    pub message: String,
    pub show_drank: bool,
}

impl WaterReminder {
    pub fn new() -> Self {
        Self {
            last_reminder: Instant::now(),
            active: false,
            title: String::new(),
            message: String::new(),
            show_drank: false,
        }
    }

    pub fn update(&mut self, enabled: bool, interval_min: u32, start_hour: u32, end_hour: u32) {
        self.active = false;
        self.show_drank = false;

        if !enabled { return; }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
        let h = ((now.as_secs() % 86400) / 3600) as u32;
        if h < start_hour || h >= end_hour { return; }

        let interval = Duration::from_secs(interval_min as u64 * 60);
        if self.last_reminder.elapsed() >= interval {
            self.active = true;
            self.title = "💧 Drink Water".into();
            self.message = format!("Time for a glass! (Every {}min)", interval_min);
        }
    }

    pub fn dismiss(&mut self) {
        self.last_reminder = Instant::now();
        self.active = false;
    }
}
