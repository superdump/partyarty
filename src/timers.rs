use crate::PerfTimers;

use specs::World;
use specs::prelude::*;

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Timer {
    pub enter: Option<Instant>,
    pub total: Duration,
    pub calls: f64,
}

impl Timer {
    pub fn enter(&mut self) {
        self.enter = Some(Instant::now());
    }

    pub fn exit(&mut self) {
        if let Some(enter) = self.enter {
            self.total += enter.elapsed();
            self.calls += 1.0;
            self.enter = None;
        }
    }

    pub fn to_msecs(&self) -> f64 {
        to_msecs(self.total)
    }

    pub fn msecs_per_call(&self) -> f64 {
        self.to_msecs() / self.calls
    }
}

#[derive(Debug)]
pub struct SlidingAverage {
    pub q: VecDeque<f64>,
    pub sum: f64,
    pub max: usize,
}

impl SlidingAverage {
    pub fn append(&mut self, v: f64) -> f64 {
        if self.q.len() == self.max {
            self.sum -= *self.q.front().unwrap_or(&0.0);
            self.q.pop_front();
        }
        self.q.push_back(v);
        self.sum += v;
        self.sum / self.q.len() as f64
    }
}

impl Default for SlidingAverage {
    fn default() -> SlidingAverage {
        SlidingAverage {
            q: VecDeque::new(),
            sum: 0.0,
            max: 10,
        }
    }
}

#[derive(Debug)]
pub struct Timers {
    pub global: Instant,
    pub frames_mean: SlidingAverage,
    pub frames: Timer,
    pub timers: HashMap<&'static str, Timer>,
}

fn to_msecs(d: Duration) -> f64 {
    (d.as_secs() as f64 * 1_000f64) + (d.subsec_nanos() as f64 / 1_000_000f64)
}

impl Timers {
    pub fn print(&self) {
        let elapsed = to_msecs(self.global.elapsed());
        let msecs_per_frame = self.frames_mean.sum / self.frames_mean.q.len() as f64;
        println!(
            "[{:6}] {:9.3} s, {:6.2} Hz, {:7.3} ms",
            self.frames.calls,
            elapsed / 1_000f64,
            1_000f64 / msecs_per_frame,
            msecs_per_frame,
        );
        for (name, timer) in self.timers.iter() {
            println!(
                "\t{:30}: {:6.2} %, {:12} calls, {:9.3} s total ({:9.3} ms/call)",
                name,
                100.0 * timer.to_msecs() / elapsed,
                timer.calls,
                timer.to_msecs() / 1_000f64,
                timer.msecs_per_call(),
            );
        }
    }

    pub fn enter(&mut self, name: &'static str) {
        if name == "frame" {
            self.frames.enter();
        } else {
            if !self.timers.contains_key(&name) {
                self.timers.insert(name.clone(), Timer::default());
            }
            self.timers.get_mut(&name).unwrap().enter();
        }
    }

    pub fn exit(&mut self, name: &'static str) {
        if name == "frame" {
            let before = self.frames.total;
            self.frames.exit();
            let after = self.frames.total;
            self.frames_mean.append(to_msecs(after - before));
        } else {
            if !self.timers.contains_key(&name) {
                self.timers.insert(name.clone(), Timer::default());
            }
            self.timers.get_mut(&name).unwrap().exit();
        }
    }
}

impl Default for Timers {
    fn default() -> Timers {
        Timers {
            global: Instant::now(),
            frames: Timer::default(),
            frames_mean: SlidingAverage::default(),
            timers: HashMap::new(),
        }
    }
}

pub fn timer_enter(world: &mut World, a: &'static str) {
    world.exec(|(mut timers, ): (Write<PerfTimers>, )| {
        timers.0.enter(a);
    });
}

pub fn timer_transition(world: &mut World, a: &'static str, b: &'static str) {
    world.exec(|(mut timers, ): (Write<PerfTimers>, )| {
        timers.0.exit(a);
        timers.0.enter(b);
    });
}

pub fn timer_exit(world: &mut World, a: &'static str) {
    world.exec(|(mut timers, ): (Write<PerfTimers>, )| {
        timers.0.exit(a);
    });
}

pub fn timer_print(world: &mut World) {
    world.exec(|(timers, ): (Read<PerfTimers>, )| {
        if timers.0.frames.calls as u32 % 10 == 0 {
            timers.0.print();
        }
    });
}
