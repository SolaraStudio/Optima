use crate::layout::box_model::BoxModel;
use crate::layout::fragment::Fragment;
use crate::css::computed::ComputedStyle;

pub struct GridLayout;

#[derive(Debug, Clone)]
pub struct GridTemplate {
    pub columns: Vec<f32>,
    pub rows: Vec<f32>,
    pub column_gap: f32,
    pub row_gap: f32,
}

impl Default for GridTemplate {
    fn default() -> Self {
        GridTemplate { columns: Vec::new(), rows: Vec::new(), column_gap: 0.0, row_gap: 0.0 }
    }
}

impl GridLayout {
    pub fn parse_template(style: &ComputedStyle) -> GridTemplate {
        let mut tmpl = GridTemplate::default();
        if let Some(v) = style.get("grid-template-columns").and_then(|v| v.as_string()) {
            tmpl.columns = Self::parse_tracks(v);
        }
        if let Some(v) = style.get("grid-template-rows").and_then(|v| v.as_string()) {
            tmpl.rows = Self::parse_tracks(v);
        }
        if let Some(v) = style.get("column-gap").and_then(|v| v.as_length()) {
            tmpl.column_gap = v.to_px(16.0);
        }
        if let Some(v) = style.get("row-gap").and_then(|v| v.as_length()) {
            tmpl.row_gap = v.to_px(16.0);
        }
        tmpl
    }

    fn parse_tracks(value: &str) -> Vec<f32> {
        value.split_whitespace()
            .filter_map(|s| s.parse::<f32>().ok())
            .collect()
    }

    pub fn layout_grid_item(col: usize, row: usize, tmpl: &GridTemplate, x: f32, y: f32) -> Fragment {
        let mut frag = Fragment::new();
        let mut cx = x;
        for c in 0..col.min(tmpl.columns.len()) { cx += tmpl.columns[c] + tmpl.column_gap; }
        let mut cy = y;
        for r in 0..row.min(tmpl.rows.len()) { cy += tmpl.rows[r] + tmpl.row_gap; }
        frag.x = cx;
        frag.y = cy;
        frag.width = tmpl.columns.get(col).copied().unwrap_or(100.0);
        frag.height = tmpl.rows.get(row).copied().unwrap_or(100.0);
        frag
    }
}
