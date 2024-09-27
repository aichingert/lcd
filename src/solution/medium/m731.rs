#[derive(Default)]
pub  struct MyCalendarTwo {
    dates: Vec<(bool, i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    pub fn new() -> Self {
        Self { dates: Vec::new() }
    }
    
    pub fn book(&mut self, start: i32, end: i32) -> bool {
        let mut ov = Vec::new();

        for (i, date) in self.dates.iter().enumerate() {
            if date.1 < end && date.2 > start {
                if date.0 {
                    return false;
                }

                ov.push(i);
            }
        }

        for over in ov {
            let (_, s, e) = self.dates[over];
            self.dates.push((true, s.max(start), e.min(end)));
        }

        self.dates.push((false, start, end));
        true
    }
}
