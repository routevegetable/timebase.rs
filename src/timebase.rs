


#[derive(Debug, Copy, Clone)]
pub enum TimebaseMode {
    OneShot,
    Repeat
}

#[derive(Debug, Copy, Clone)]
pub struct Timebase {
    now: i32,
    mode: TimebaseMode,
    period: i32,
    trigger: Event,
}

pub struct Frame {
    now: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Input {
    ev: Event
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    when: i32,
}

impl Frame {
    pub fn new(now: i32) -> Self {
        Self{now}
    }
    
    pub fn timebase(&self, mode: TimebaseMode, period: i32, trigger: Event) -> Timebase {
        Timebase {
            now: self.now,
            mode,
            period,
            trigger,
        }
    }

    pub fn trigger(&self, inp: &mut Input) {
        inp.ev.when = self.now;
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Input {
        Input { ev: Event::zero() }
    }
}

impl From<Input> for Event {
    fn from(input: Input) -> Self {
        input.ev
    }
}

impl Event {
    pub fn zero() -> Self {
        Self { when: 0 }
    }
}

impl std::ops::BitOr<Self> for Event {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            when: std::cmp::max(self.when, rhs.when),
        }
    }
}

impl std::ops::BitOrAssign for Event {
    fn bitor_assign(&mut self, rhs: Self) {
        self.when = std::cmp::max(self.when, rhs.when);
    }
}

impl Timebase {
    pub fn between(&self, from: f32, to: f32) -> bool {
        let v = self.get();
        v >= from && v < to
    }

    pub fn scale(&self, from: f32, to: f32) -> f32 {
        let d = to - from;
        from + self.get() * d
    }

    pub fn square(&self) -> f32 {
        if self.between(0.5, 1f32) {
            1f32
        } else {
            0f32
        }
    }

    pub fn seq<const N: usize>(&self) -> [Event; N] {
        let mut n = [Event::zero(); N];

        for (i, ev) in n.iter_mut().enumerate() {
            *ev = self.at(i as f32 / N as f32);
        }
        n
    }

    pub fn wave<const N: usize, T: Copy>(&self, w: [T; N]) -> T {
        let idx = self.scale(0f32, N as f32) as usize;
        w[std::cmp::min(idx, N-1)]
    }
    
    pub fn fountain<const N: usize>(&self, seed: usize, excite: usize) -> [Event; N] {
        let mut n = [Event::zero(); N];

        let mut rng = fastrand::Rng::with_seed(seed as u64);
        for i in 0..N * excite {
            // Random number between 0 and 1
            let r = rng.f32();

            let bin = i / excite;

            let ev = self.at(r);
            if ev.when < self.now {
                /* This event has happened - OR it */
                n[bin] |= ev;
            }
        }

        n
    }

    pub fn top_half(&self) -> bool {
        self.between(0.5, 1f32)
    }

    pub fn circle(&self) -> f32 {
        self.scale(0f32, 2f32 * std::f32::consts::PI)
    }

    pub fn sin(&self) -> f32 {
        f32::sin(self.circle()) * 0.5 + 0.5
    }

    pub fn sync(&self) -> Event {
        self.at(0f32)
    }

    fn time_since_trigger(&self) -> i32 {
        self.now - self.trigger.when
    }

    pub fn at(&self, target: f32) -> Event {
        match self.mode {
            TimebaseMode::OneShot => Event {
                when: self.trigger.when + (self.period as f32 * target) as i32,
            },
            TimebaseMode::Repeat => {
                let t = self.time_since_trigger();

                let time_in_cycle = t % self.period;

                let cycle_start_time = t - time_in_cycle;

                let target_time_in_cycle = (self.period as f32 * target) as i32;

                if time_in_cycle >= target_time_in_cycle {
                    /* Trigger is in this cycle */
                    Event {
                        when: self.trigger.when + cycle_start_time + target_time_in_cycle,
                    }
                } else {
                    /* Trigger is in last cycle */
                    Event {
                        when: self.trigger.when + cycle_start_time - self.period + target_time_in_cycle,
                    }
                }
            }
        }
    }

    pub fn shift(&self, shift: i32) -> Timebase {
        Timebase {
            now: self.now,
            mode: self.mode,
            period: self.period,
            trigger: Event{when:self.trigger.when + shift},
        }
    }

    pub fn get(&self) -> f32 {
        let t = self.time_since_trigger();

        match self.mode {
            TimebaseMode::OneShot => {
                if t < 0 {
                    0f32
                } else if t >= self.period {
                    1f32
                } else {
                    t as f32 / self.period as f32
                }
            }
            TimebaseMode::Repeat => {
                let m = t % self.period;
                m as f32 / self.period as f32
            }
        }
    }
}