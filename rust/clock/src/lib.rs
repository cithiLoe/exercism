use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut mnts = hours * 60 + minutes;
        while mnts < 0 {
            mnts += 1440;
        }
        while mnts >= 1440 {
            mnts -= 1440;
        }
        Clock { minutes: mnts }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.minutes / 60, self.minutes % 60 + minutes)
    }
}
