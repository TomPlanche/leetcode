#[derive(Default)]
struct MyCalendar {
    events: std::collections::BTreeSet<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let left = self
            .events
            .range((start, 0)..)
            .next()
            .map_or(true, |&(s, _)| s >= end); // no overlap

        let right = self
            .events
            .range(..(start, 0))
            .rev()
            .next()
            .map_or(true, |&(_, e)| e <= start); // no overlap

        left && right && self.events.insert((start, end))
    }
}
