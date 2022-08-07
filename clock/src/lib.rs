use core::fmt;
#[derive(PartialEq, Eq)]
pub struct Clock {
    hours : i32,
    minutes : i32
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours, minutes}.normalize()
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.normalize()

    }

    pub fn to_string(self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    fn normalize(mut self) -> Self{
        let mut carry_hours: i32 = 0;
        if self.minutes.abs() > 59 {
            carry_hours = self.minutes.signum();
            carry_hours *= self.minutes.abs() / 60;
            self.minutes = self.minutes.signum() * (self.minutes.abs() % 60 );
        }

        if self.minutes < 0 {
            self.minutes = 60 + self.minutes;
            carry_hours -= 1;
        }

        self.hours += carry_hours;

        self.hours = self.hours.signum() * (self.hours.abs() % 24);

        if self.hours < 0 {
            self.hours = 24 + self.hours;
        }

        self
    }
}


impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}