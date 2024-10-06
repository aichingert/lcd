pub struct MyCalendar {
    dates: Vec<(i32, i32)>,
}

impl Default for MyCalendar {
    fn default() -> Self {
        Self::new()
    }
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    pub fn new() -> Self {
        Self { dates: Vec::new() }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        for date in self.dates.iter() {
            if date.0 < end && date.1 > start {
                return false;
            }
        }

        self.dates.push((start, end));
        true
    }
}
