use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TimingEntry {
    pub label: String,
    pub start_ms: f64,
    pub end_ms: Option<f64>,
    pub duration_ms: Option<f64>,
    pub metadata: HashMap<String, String>,
}

impl TimingEntry {
    pub fn new(label: &str, start_ms: f64) -> Self {
        TimingEntry {
            label: label.to_string(),
            start_ms,
            end_ms: None,
            duration_ms: None,
            metadata: HashMap::new(),
        }
    }

    pub fn finish(&mut self, end_ms: f64) {
        self.end_ms = Some(end_ms);
        self.duration_ms = Some(end_ms - self.start_ms);
    }

    pub fn is_complete(&self) -> bool {
        self.end_ms.is_some()
    }

    pub fn duration(&self) -> f64 {
        self.duration_ms.unwrap_or(0.0)
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone)]
pub struct PaintMetrics {
    pub frames_total: u64,
    pub frames_dropped: u64,
    pub paint_time_ms: f64,
    pub composite_time_ms: f64,
    pub total_paint_area: u64,
    pub layers_replayed: u64,
}

impl Default for PaintMetrics {
    fn default() -> Self {
        PaintMetrics {
            frames_total: 0,
            frames_dropped: 0,
            paint_time_ms: 0.0,
            composite_time_ms: 0.0,
            total_paint_area: 0,
            layers_replayed: 0,
        }
    }
}

impl PaintMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_frame(&mut self, paint_ms: f64, composite_ms: f64, area: u64) {
        self.frames_total += 1;
        self.paint_time_ms += paint_ms;
        self.composite_time_ms += composite_ms;
        self.total_paint_area += area;
    }

    pub fn record_dropped_frame(&mut self) {
        self.frames_total += 1;
        self.frames_dropped += 1;
    }

    pub fn drop_rate(&self) -> f32 {
        if self.frames_total == 0 {
            0.0
        } else {
            (self.frames_dropped as f32 / self.frames_total as f32) * 100.0
        }
    }

    pub fn avg_paint_time(&self) -> f64 {
        if self.frames_total == 0 {
            0.0
        } else {
            self.paint_time_ms / self.frames_total as f64
        }
    }

    pub fn avg_composite_time(&self) -> f64 {
        if self.frames_total == 0 {
            0.0
        } else {
            self.composite_time_ms / self.frames_total as f64
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
pub struct LayoutMetrics {
    pub layout_count: u64,
    pub total_layout_time_ms: f64,
    pub reflow_count: u64,
    pub elements_laid_out: u64,
    pub dirty_nodes: u32,
}

impl Default for LayoutMetrics {
    fn default() -> Self {
        LayoutMetrics {
            layout_count: 0,
            total_layout_time_ms: 0.0,
            reflow_count: 0,
            elements_laid_out: 0,
            dirty_nodes: 0,
        }
    }
}

impl LayoutMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_layout(&mut self, duration_ms: f64, elements: u64) {
        self.layout_count += 1;
        self.total_layout_time_ms += duration_ms;
        self.elements_laid_out += elements;
    }

    pub fn record_reflow(&mut self) {
        self.reflow_count += 1;
    }

    pub fn set_dirty_nodes(&mut self, count: u32) {
        self.dirty_nodes = count;
    }

    pub fn avg_layout_time(&self) -> f64 {
        if self.layout_count == 0 {
            0.0
        } else {
            self.total_layout_time_ms / self.layout_count as f64
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub heap_used_bytes: u64,
    pub heap_allocated_bytes: u64,
    pub external_bytes: u64,
    pub array_buffer_bytes: u64,
    pub dom_node_count: u32,
    pub dom_tree_depth: u32,
    pub event_listener_count: u32,
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        MemoryMetrics {
            heap_used_bytes: 0,
            heap_allocated_bytes: 0,
            external_bytes: 0,
            array_buffer_bytes: 0,
            dom_node_count: 0,
            dom_tree_depth: 0,
            event_listener_count: 0,
        }
    }
}

impl MemoryMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heap_used_mb(&self) -> f64 {
        self.heap_used_bytes as f64 / (1024.0 * 1024.0)
    }

    pub fn heap_allocated_mb(&self) -> f64 {
        self.heap_allocated_bytes as f64 / (1024.0 * 1024.0)
    }

    pub fn fragmentation_ratio(&self) -> f64 {
        if self.heap_allocated_bytes == 0 {
            0.0
        } else {
            let free = self.heap_allocated_bytes - self.heap_used_bytes;
            free as f64 / self.heap_allocated_bytes as f64
        }
    }

    pub fn total_memory_mb(&self) -> f64 {
        (self.heap_used_bytes + self.external_bytes + self.array_buffer_bytes) as f64
            / (1024.0 * 1024.0)
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub timing: Vec<TimingEntry>,
    pub paint: PaintMetrics,
    pub layout: LayoutMetrics,
    pub memory: MemoryMetrics,
    pub navigation_start_ms: f64,
    pub dom_content_loaded_ms: Option<f64>,
    pub load_complete_ms: Option<f64>,
    pub marks: HashMap<String, f64>,
    pub measures: HashMap<String, f64>,
    pub active_timers: HashMap<String, f64>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            timing: Vec::new(),
            paint: PaintMetrics::default(),
            layout: LayoutMetrics::default(),
            memory: MemoryMetrics::default(),
            navigation_start_ms: 0.0,
            dom_content_loaded_ms: None,
            load_complete_ms: None,
            marks: HashMap::new(),
            measures: HashMap::new(),
            active_timers: HashMap::new(),
        }
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark(&mut self, name: &str, time_ms: f64) {
        self.marks.insert(name.to_string(), time_ms);
    }

    pub fn measure(&mut self, name: &str, start_mark: &str, end_mark: &str) -> Option<f64> {
        let start = self.marks.get(start_mark).copied()?;
        let end = self.marks.get(end_mark).copied()?;
        let duration = end - start;
        self.measures.insert(name.to_string(), duration);
        Some(duration)
    }

    pub fn start_timer(&mut self, label: &str, time_ms: f64) {
        self.active_timers.insert(label.to_string(), time_ms);
    }

    pub fn end_timer(&mut self, label: &str, end_ms: f64) -> Option<f64> {
        let start = self.active_timers.remove(label)?;
        let duration = end_ms - start;
        self.measures.insert(label.to_string(), duration);
        Some(duration)
    }

    pub fn start_timing(&mut self, label: &str, start_ms: f64) {
        self.timing.push(TimingEntry::new(label, start_ms));
    }

    pub fn end_timing(&mut self, label: &str, end_ms: f64) {
        if let Some(entry) = self.timing.iter_mut().rev().find(|t| t.label == label && !t.is_complete()) {
            entry.finish(end_ms);
        }
    }

    pub fn get_timing(&self, label: &str) -> Option<&TimingEntry> {
        self.timing.iter().rev().find(|t| t.label == label)
    }

    pub fn all_timings(&self, label: &str) -> Vec<&TimingEntry> {
        self.timing.iter().filter(|t| t.label == label).collect()
    }

    pub fn set_navigation_start(&mut self, time_ms: f64) {
        self.navigation_start_ms = time_ms;
    }

    pub fn set_dom_content_loaded(&mut self, time_ms: f64) {
        self.dom_content_loaded_ms = Some(time_ms);
    }

    pub fn set_load_complete(&mut self, time_ms: f64) {
        self.load_complete_ms = Some(time_ms);
    }

    pub fn time_to_dom_content_loaded(&self) -> Option<f64> {
        self.dom_content_loaded_ms.map(|t| t - self.navigation_start_ms)
    }

    pub fn time_to_load(&self) -> Option<f64> {
        self.load_complete_ms.map(|t| t - self.navigation_start_ms)
    }

    pub fn long_tasks(&self, threshold_ms: f64) -> Vec<&TimingEntry> {
        self.timing
            .iter()
            .filter(|t| t.duration() >= threshold_ms)
            .collect()
    }

    pub fn summary(&self) -> PerformanceSummary {
        PerformanceSummary {
            total_timings: self.timing.len(),
            completed_timings: self.timing.iter().filter(|t| t.is_complete()).count(),
            total_paint_frames: self.paint.frames_total,
            dropped_frames: self.paint.frames_dropped,
            total_layouts: self.layout.layout_count,
            dom_nodes: self.memory.dom_node_count,
            heap_mb: self.memory.heap_used_mb(),
        }
    }

    pub fn clear(&mut self) {
        self.timing.clear();
        self.marks.clear();
        self.measures.clear();
        self.active_timers.clear();
        self.paint.reset();
        self.layout.reset();
        self.dom_content_loaded_ms = None;
        self.load_complete_ms = None;
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub total_timings: usize,
    pub completed_timings: usize,
    pub total_paint_frames: u64,
    pub dropped_frames: u64,
    pub total_layouts: u64,
    pub dom_nodes: u32,
    pub heap_mb: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_entry() {
        let mut t = TimingEntry::new("render", 100.0);
        assert!(!t.is_complete());
        assert_eq!(t.duration(), 0.0);

        t.finish(150.0);
        assert!(t.is_complete());
        assert_eq!(t.duration(), 50.0);
    }

    #[test]
    fn test_timing_metadata() {
        let t = TimingEntry::new("paint", 0.0)
            .with_metadata("region", "viewport")
            .with_metadata("layers", "5");
        assert_eq!(t.metadata.get("region").unwrap(), "viewport");
        assert_eq!(t.metadata.get("layers").unwrap(), "5");
    }

    #[test]
    fn test_paint_metrics() {
        let mut paint = PaintMetrics::new();
        paint.record_frame(8.0, 2.0, 1920 * 1080);
        paint.record_frame(16.0, 4.0, 1920 * 1080);
        paint.record_dropped_frame();

        assert_eq!(paint.frames_total, 3);
        assert_eq!(paint.frames_dropped, 1);
        assert!((paint.drop_rate() - 33.33).abs() < 0.1);
        assert!((paint.avg_paint_time() - 12.0).abs() < 0.01);
    }

    #[test]
    fn test_paint_metrics_reset() {
        let mut paint = PaintMetrics::new();
        paint.record_frame(8.0, 2.0, 100);
        paint.reset();
        assert_eq!(paint.frames_total, 0);
    }

    #[test]
    fn test_layout_metrics() {
        let mut layout = LayoutMetrics::new();
        layout.record_layout(5.0, 100);
        layout.record_layout(3.0, 50);
        layout.record_reflow();

        assert_eq!(layout.layout_count, 2);
        assert!((layout.avg_layout_time() - 4.0).abs() < 0.01);
        assert_eq!(layout.reflow_count, 1);
        assert_eq!(layout.elements_laid_out, 150);
    }

    #[test]
    fn test_memory_metrics() {
        let mut mem = MemoryMetrics::new();
        mem.heap_used_bytes = 50 * 1024 * 1024;
        mem.heap_allocated_bytes = 80 * 1024 * 1024;
        mem.external_bytes = 10 * 1024 * 1024;

        assert!((mem.heap_used_mb() - 50.0).abs() < 0.01);
        assert!((mem.total_memory_mb() - 60.0).abs() < 0.01);
        assert!((mem.fragmentation_ratio() - 0.375).abs() < 0.01);
    }

    #[test]
    fn test_performance_marks_and_measures() {
        let mut perf = PerformanceMetrics::new();
        perf.mark("start", 0.0);
        perf.mark("end", 100.0);

        let duration = perf.measure("total", "start", "end");
        assert_eq!(duration, Some(100.0));
        assert_eq!(perf.measures.get("total"), Some(&100.0));

        let missing = perf.measure("bad", "start", "nonexistent");
        assert_eq!(missing, None);
    }

    #[test]
    fn test_performance_timers() {
        let mut perf = PerformanceMetrics::new();
        perf.start_timer("api_call", 100.0);
        let duration = perf.end_timer("api_call", 250.0);
        assert_eq!(duration, Some(150.0));
        assert_eq!(perf.measures.get("api_call"), Some(&150.0));

        let missing = perf.end_timer("unknown", 300.0);
        assert_eq!(missing, None);
    }

    #[test]
    fn test_performance_timing() {
        let mut perf = PerformanceMetrics::new();
        perf.start_timing("render", 0.0);
        perf.start_timing("layout", 10.0);
        perf.end_timing("render", 50.0);
        perf.end_timing("layout", 30.0);

        let render = perf.get_timing("render").unwrap();
        assert_eq!(render.duration(), 50.0);
    }

    #[test]
    fn test_performance_navigation() {
        let mut perf = PerformanceMetrics::new();
        perf.set_navigation_start(0.0);
        perf.set_dom_content_loaded(200.0);
        perf.set_load_complete(500.0);

        assert_eq!(perf.time_to_dom_content_loaded(), Some(200.0));
        assert_eq!(perf.time_to_load(), Some(500.0));
    }

    #[test]
    fn test_long_tasks() {
        let mut perf = PerformanceMetrics::new();
        perf.start_timing("short", 0.0);
        perf.end_timing("short", 10.0);
        perf.start_timing("long", 0.0);
        perf.end_timing("long", 100.0);

        let long = perf.long_tasks(50.0);
        assert_eq!(long.len(), 1);
        assert_eq!(long[0].label, "long");
    }

    #[test]
    fn test_performance_summary() {
        let mut perf = PerformanceMetrics::new();
        perf.start_timing("t1", 0.0);
        perf.end_timing("t1", 10.0);
        perf.paint.record_frame(8.0, 2.0, 100);

        let summary = perf.summary();
        assert_eq!(summary.total_timings, 1);
        assert_eq!(summary.completed_timings, 1);
        assert_eq!(summary.total_paint_frames, 1);
    }

    #[test]
    fn test_performance_clear() {
        let mut perf = PerformanceMetrics::new();
        perf.mark("m1", 0.0);
        perf.paint.record_frame(8.0, 2.0, 100);
        perf.clear();
        assert!(perf.marks.is_empty());
        assert_eq!(perf.paint.frames_total, 0);
    }
}
