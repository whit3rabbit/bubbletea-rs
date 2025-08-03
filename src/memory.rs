//! Memory usage monitoring utilities.
//!
//! This module provides optional memory monitoring features that can be enabled
//! to track memory usage patterns and identify potential issues.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Memory usage statistics and monitoring.
#[derive(Debug, Clone)]
pub struct MemoryMonitor {
    /// Number of active timers
    pub active_timers: Arc<AtomicU64>,
    /// Number of spawned tasks
    pub active_tasks: Arc<AtomicU64>,
    /// Current channel buffer depth
    pub channel_depth: Arc<AtomicU64>,
    /// Total messages processed
    pub messages_processed: Arc<AtomicU64>,
    /// Peak memory usage (if available)
    pub peak_memory_bytes: Arc<AtomicU64>,
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryMonitor {
    /// Create a new memory monitor.
    pub fn new() -> Self {
        Self {
            active_timers: Arc::new(AtomicU64::new(0)),
            active_tasks: Arc::new(AtomicU64::new(0)),
            channel_depth: Arc::new(AtomicU64::new(0)),
            messages_processed: Arc::new(AtomicU64::new(0)),
            peak_memory_bytes: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Increment the timer count.
    pub fn timer_added(&self) {
        self.active_timers.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement the timer count.
    pub fn timer_removed(&self) {
        self.active_timers.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get the current number of active timers.
    pub fn get_active_timers(&self) -> u64 {
        self.active_timers.load(Ordering::Relaxed)
    }

    /// Increment the task count.
    pub fn task_spawned(&self) {
        self.active_tasks.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement the task count.
    pub fn task_completed(&self) {
        self.active_tasks.fetch_sub(1, Ordering::Relaxed);
    }

    /// Get the current number of active tasks.
    pub fn get_active_tasks(&self) -> u64 {
        self.active_tasks.load(Ordering::Relaxed)
    }

    /// Update the channel depth.
    pub fn set_channel_depth(&self, depth: u64) {
        self.channel_depth.store(depth, Ordering::Relaxed);
    }

    /// Get the current channel depth.
    pub fn get_channel_depth(&self) -> u64 {
        self.channel_depth.load(Ordering::Relaxed)
    }

    /// Increment the message count.
    pub fn message_processed(&self) {
        self.messages_processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Get the total number of messages processed.
    pub fn get_messages_processed(&self) -> u64 {
        self.messages_processed.load(Ordering::Relaxed)
    }

    /// Update peak memory usage (if tracking is enabled).
    pub fn update_peak_memory(&self, bytes: u64) {
        loop {
            let current = self.peak_memory_bytes.load(Ordering::Relaxed);
            if bytes <= current {
                break;
            }
            if self
                .peak_memory_bytes
                .compare_exchange_weak(current, bytes, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }

    /// Get the peak memory usage.
    pub fn get_peak_memory_bytes(&self) -> u64 {
        self.peak_memory_bytes.load(Ordering::Relaxed)
    }

    /// Get a snapshot of all current metrics.
    pub fn snapshot(&self) -> MemorySnapshot {
        MemorySnapshot {
            active_timers: self.get_active_timers(),
            active_tasks: self.get_active_tasks(),
            channel_depth: self.get_channel_depth(),
            messages_processed: self.get_messages_processed(),
            peak_memory_bytes: self.get_peak_memory_bytes(),
        }
    }

    /// Reset all counters to zero.
    pub fn reset(&self) {
        self.active_timers.store(0, Ordering::Relaxed);
        self.active_tasks.store(0, Ordering::Relaxed);
        self.channel_depth.store(0, Ordering::Relaxed);
        self.messages_processed.store(0, Ordering::Relaxed);
        self.peak_memory_bytes.store(0, Ordering::Relaxed);
    }

    /// Check if any metrics indicate potential memory issues.
    pub fn check_health(&self) -> MemoryHealth {
        let snapshot = self.snapshot();
        let mut issues = Vec::new();

        // Check for excessive timer accumulation
        if snapshot.active_timers > 100 {
            issues.push(format!("High timer count: {}", snapshot.active_timers));
        }

        // Check for excessive task accumulation
        if snapshot.active_tasks > 50 {
            issues.push(format!("High task count: {}", snapshot.active_tasks));
        }

        // Check for channel backlog
        if snapshot.channel_depth > 1000 {
            issues.push(format!("High channel depth: {}", snapshot.channel_depth));
        }

        MemoryHealth {
            is_healthy: issues.is_empty(),
            issues,
            snapshot,
        }
    }
}

/// A snapshot of memory usage metrics at a point in time.
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    /// Number of currently active timers
    pub active_timers: u64,
    /// Number of currently active async tasks
    pub active_tasks: u64,
    /// Current message channel buffer depth
    pub channel_depth: u64,
    /// Total number of messages processed since startup
    pub messages_processed: u64,
    /// Peak memory usage recorded in bytes
    pub peak_memory_bytes: u64,
}

/// Health check result for memory usage.
#[derive(Debug, Clone)]
pub struct MemoryHealth {
    /// Whether the memory usage is within healthy thresholds
    pub is_healthy: bool,
    /// List of detected issues if any
    pub issues: Vec<String>,
    /// Current memory usage snapshot
    pub snapshot: MemorySnapshot,
}

impl std::fmt::Display for MemorySnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Memory Snapshot - Timers: {}, Tasks: {}, Channel: {}, Messages: {}, Peak Memory: {} bytes",
            self.active_timers,
            self.active_tasks,
            self.channel_depth,
            self.messages_processed,
            self.peak_memory_bytes
        )
    }
}

impl std::fmt::Display for MemoryHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_healthy {
            write!(f, "Memory Health: HEALTHY\n{}", self.snapshot)
        } else {
            write!(
                f,
                "Memory Health: ISSUES DETECTED\nIssues: {}\n{}",
                self.issues.join(", "),
                self.snapshot
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_monitor_basic() {
        let monitor = MemoryMonitor::new();

        assert_eq!(monitor.get_active_timers(), 0);
        assert_eq!(monitor.get_active_tasks(), 0);

        monitor.timer_added();
        monitor.task_spawned();

        assert_eq!(monitor.get_active_timers(), 1);
        assert_eq!(monitor.get_active_tasks(), 1);

        monitor.timer_removed();
        monitor.task_completed();

        assert_eq!(monitor.get_active_timers(), 0);
        assert_eq!(monitor.get_active_tasks(), 0);
    }

    #[test]
    fn test_memory_health_check() {
        let monitor = MemoryMonitor::new();

        // Initially healthy
        let health = monitor.check_health();
        assert!(health.is_healthy);

        // Add many timers to trigger warning
        for _ in 0..150 {
            monitor.timer_added();
        }

        let health = monitor.check_health();
        assert!(!health.is_healthy);
        assert!(!health.issues.is_empty());
    }

    #[test]
    fn test_peak_memory_tracking() {
        let monitor = MemoryMonitor::new();

        monitor.update_peak_memory(1000);
        assert_eq!(monitor.get_peak_memory_bytes(), 1000);

        monitor.update_peak_memory(500); // Should not update
        assert_eq!(monitor.get_peak_memory_bytes(), 1000);

        monitor.update_peak_memory(2000); // Should update
        assert_eq!(monitor.get_peak_memory_bytes(), 2000);
    }
}
