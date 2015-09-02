use time;

fn time_now() -> f32 {
    time::precise_time_s() as f32
}

#[derive(Debug, Clone)]
pub struct Delta {
    delta: f32,
    last: f32,
}

impl Delta {
    pub fn new() -> Delta {
        Delta {
            delta: 0.0,
            last: time_now(),
        }
    }

    pub fn update(&mut self) -> f32 {
        let now = time_now();
        self.delta = now - self.last;
        self.last = now;

        self.delta
    }
}
