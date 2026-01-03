//! # Concept: Performance Instrumentation
//! 
//! This module provides the engine's observability tools. It implements a 
//! stack-based hierarchical profiler and maintains rolling session statistics 
//! to identify CPU bottlenecks and monitor frame rate stability.

use std::collections::{HashMap, VecDeque};
use std::time::Instant;

const SMOOTHING_WINDOW: usize = 100;
const FRAME_BUDGET_MICROS: f64 = 16666.6; 

/// Stores duration metrics for named measurement scopes within a single frame.
#[derive(Clone, Debug, Default)]
pub struct FrameMetrics {
    pub scopes: HashMap<String, u64>,
}

/// A ring buffer used to calculate rolling averages for smoothed telemetry.
#[derive(Clone, Debug)]
struct RingBuffer {
    buffer: VecDeque<u64>,
    sum: u64,
    size: usize,
}

impl RingBuffer {
    fn new(size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(size),
            sum: 0,
            size,
        }
    }

    fn push(&mut self, value: u64) {
        if self.buffer.len() == self.size
            && let Some(popped) = self.buffer.pop_front()
        {
            self.sum -= popped;
        }
        self.buffer.push_back(value);
        self.sum += value;
    }

    fn average(&self) -> f64 {
        if self.buffer.is_empty() { 0.0 } else { self.sum as f64 / self.buffer.len() as f64 }
    }
}

/// The primary manager for performance tracking and instrumentation.
#[derive(Clone)]
pub struct Benchmarker {
    pub min_fps: u32,
    pub max_fps: u32,
    pub avg_fps: u32,
    total_fps_sum: u64,
    pub samples: u32,
    scope_stack: Vec<(String, Instant)>,
    current_frame_metrics: FrameMetrics,
    history: HashMap<String, RingBuffer>,
}

impl Benchmarker {
    pub fn new() -> Self {
        Self {
            min_fps: u32::MAX,
            max_fps: 0,
            avg_fps: 0,
            total_fps_sum: 0,
            samples: 0,
            scope_stack: Vec::with_capacity(16),
            current_frame_metrics: FrameMetrics::default(),
            history: HashMap::new(),
        }
    }

    /// Resets all session-wide statistics.
    pub fn reset(&mut self) {
        self.min_fps = u32::MAX;
        self.max_fps = 0;
        self.avg_fps = 0;
        self.total_fps_sum = 0;
        self.samples = 0;
        self.history.clear();
    }

    pub fn update_fps(&mut self, current_fps: u32) {
        if current_fps == 0 { return; }
        if current_fps < self.min_fps { self.min_fps = current_fps; }
        if current_fps > self.max_fps { self.max_fps = current_fps; }
        self.total_fps_sum += current_fps as u64;
        self.samples += 1;
        self.avg_fps = (self.total_fps_sum / self.samples as u64) as u32;
    }

    pub fn push(&mut self, name: &str) { self.scope_stack.push((name.to_string(), Instant::now())); }

    pub fn pop(&mut self) {
        if let Some((name, start_time)) = self.scope_stack.pop() {
            let duration = start_time.elapsed().as_micros() as u64;
            *self.current_frame_metrics.scopes.entry(name).or_insert(0) += duration;
        }
    }

    pub fn end_frame(&mut self) {
        for (name, duration) in &self.current_frame_metrics.scopes {
            self.history.entry(name.clone()).or_insert_with(|| RingBuffer::new(SMOOTHING_WINDOW)).push(*duration);
        }
        self.current_frame_metrics.scopes.clear();
        self.scope_stack.clear();
    }

    pub fn get_sorted_metrics(&self) -> Vec<(String, f64)> {
        let mut results = Vec::new();
        for (name, ring_buffer) in &self.history {
            let avg_micros = ring_buffer.average();
            let percent = (avg_micros / FRAME_BUDGET_MICROS) * 100.0;
            results.push((name.clone(), percent));
        }
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

impl Default for Benchmarker {
    fn default() -> Self {
        Self::new()
    }
}