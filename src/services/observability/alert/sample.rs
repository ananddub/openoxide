use super::metric::MetricKind;
use std::collections::VecDeque;

const FIRE_RATIO: f64 = 0.8;

#[derive(Clone, Copy, Debug)]
pub struct MetricSample {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
}

impl MetricSample {
    pub fn value_for(&self, metric: MetricKind) -> f64 {
        match metric {
            MetricKind::Cpu => self.cpu_percent,
            MetricKind::Memory => self.memory_percent,
            MetricKind::Disk => self.disk_percent,
        }
    }
}

#[derive(Debug)]
pub struct SampleWindow {
    capacity: usize,
    samples: VecDeque<bool>,
}

impl SampleWindow {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            samples: VecDeque::with_capacity(capacity.max(1)),
        }
    }

    pub fn push(&mut self, breached: bool) -> bool {
        if self.samples.len() == self.capacity {
            self.samples.pop_front();
        }
        self.samples.push_back(breached);

        if self.samples.len() < self.capacity {
            return false;
        }

        let breaches = self.samples.iter().filter(|b| **b).count();
        breaches as f64 / self.samples.len() as f64 >= FIRE_RATIO
    }

    pub fn reset(&mut self) {
        self.samples.clear();
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}
