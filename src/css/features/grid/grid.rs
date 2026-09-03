#[derive(Debug, Clone, PartialEq)]
pub enum GridTrack {
    Fixed(f32),
    Fraction(f32),
    Auto,
    MinMax(Box<GridTrack>, Box<GridTrack>),
    MinContent,
    MaxContent,
}

impl GridTrack {
    pub fn resolve(&self, available: f32) -> f32 {
        match self {
            GridTrack::Fixed(v) => *v,
            GridTrack::Fraction(_) => 0.0,
            GridTrack::Auto => 0.0,
            GridTrack::MinMax(min, max) => {
                let min_val = min.resolve(available);
                let max_val = max.resolve(available);
                min_val.clamp(0.0, max_val)
            }
            GridTrack::MinContent => 0.0,
            GridTrack::MaxContent => 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridPlacement {
    pub column_start: Option<i32>,
    pub column_end: Option<i32>,
    pub row_start: Option<i32>,
    pub row_end: Option<i32>,
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self::new()
    }
}

impl GridPlacement {
    pub fn new() -> Self {
        GridPlacement {
            column_start: None,
            column_end: None,
            row_start: None,
            row_end: None,
        }
    }

    pub fn span(columns: i32, rows: i32) -> Self {
        GridPlacement {
            column_start: None,
            column_end: Some(columns),
            row_start: None,
            row_end: Some(rows),
        }
    }

    pub fn column_span(&self) -> i32 {
        match (self.column_start, self.column_end) {
            (Some(s), Some(e)) => (e - s).max(1),
            (None, Some(e)) => e,
            _ => 1,
        }
    }

    pub fn row_span(&self) -> i32 {
        match (self.row_start, self.row_end) {
            (Some(s), Some(e)) => (e - s).max(1),
            (None, Some(e)) => e,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridItem {
    pub placement: GridPlacement,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
}

impl Default for GridItem {
    fn default() -> Self {
        Self::new()
    }
}

impl GridItem {
    pub fn new() -> Self {
        GridItem {
            placement: GridPlacement::new(),
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }

    pub fn with_placement(mut self, placement: GridPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn clamp_width(&self, w: f32) -> f32 {
        let min = self.min_width.unwrap_or(0.0);
        let max = self.max_width.unwrap_or(f32::INFINITY);
        w.clamp(min, max)
    }

    pub fn clamp_height(&self, h: f32) -> f32 {
        let min = self.min_height.unwrap_or(0.0);
        let max = self.max_height.unwrap_or(f32::INFINITY);
        h.clamp(min, max)
    }
}

#[derive(Debug, Clone)]
pub struct GridContainer {
    pub template_columns: Vec<GridTrack>,
    pub template_rows: Vec<GridTrack>,
    pub column_gap: f32,
    pub row_gap: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for GridContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl GridContainer {
    pub fn new() -> Self {
        GridContainer {
            template_columns: Vec::new(),
            template_rows: Vec::new(),
            column_gap: 0.0,
            row_gap: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn with_columns(mut self, columns: Vec<GridTrack>) -> Self {
        self.template_columns = columns;
        self
    }

    pub fn with_rows(mut self, rows: Vec<GridTrack>) -> Self {
        self.template_rows = rows;
        self
    }

    pub fn with_column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn with_row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self
    }

    pub fn column_count(&self) -> usize {
        self.template_columns.len()
    }

    pub fn row_count(&self) -> usize {
        self.template_rows.len()
    }

    pub fn resolve_column_sizes(&self) -> Vec<f32> {
        self.resolve_tracks(&self.template_columns, self.width, self.column_gap)
    }

    pub fn resolve_row_sizes(&self) -> Vec<f32> {
        self.resolve_tracks(&self.template_rows, self.height, self.row_gap)
    }

    fn resolve_tracks(&self, tracks: &[GridTrack], available: f32, gap: f32) -> Vec<f32> {
        if tracks.is_empty() {
            return Vec::new();
        }
        let gap_total = (tracks.len() as f32 - 1.0).max(0.0) * gap;
        let distributable = available - gap_total;
        let fixed_total: f32 = tracks
            .iter()
            .map(|t| match t {
                GridTrack::Fixed(v) => *v,
                _ => 0.0,
            })
            .sum();
        let fraction_total: f32 = tracks
            .iter()
            .map(|t| match t {
                GridTrack::Fraction(f) => *f,
                _ => 0.0,
            })
            .sum();
        let remaining = distributable - fixed_total;
        let per_fraction = if fraction_total > 0.0 {
            remaining.max(0.0) / fraction_total
        } else {
            0.0
        };
        tracks
            .iter()
            .map(|t| match t {
                GridTrack::Fixed(v) => *v,
                GridTrack::Fraction(f) => per_fraction * f,
                GridTrack::Auto => {
                    let auto_count = tracks
                        .iter()
                        .filter(|t| matches!(t, GridTrack::Auto))
                        .count() as f32;
                    if auto_count > 0.0 {
                        remaining.max(0.0) / auto_count
                    } else {
                        0.0
                    }
                }
                GridTrack::MinMax(min, _max) => min.resolve(distributable),
                GridTrack::MinContent => 0.0,
                GridTrack::MaxContent => 0.0,
            })
            .collect()
    }

    pub fn layout(&self, items: &[GridItem]) -> Vec<GridLayoutBox> {
        let col_sizes = self.resolve_column_sizes();
        let row_sizes = self.resolve_row_sizes();
        let mut col_offsets = Vec::new();
        let mut offset = 0.0;
        for (i, &size) in col_sizes.iter().enumerate() {
            col_offsets.push(offset);
            offset += size;
            if i < col_sizes.len() - 1 {
                offset += self.column_gap;
            }
        }
        let mut row_offsets = Vec::new();
        let mut offset = 0.0;
        for (i, &size) in row_sizes.iter().enumerate() {
            row_offsets.push(offset);
            offset += size;
            if i < row_sizes.len() - 1 {
                offset += self.row_gap;
            }
        }
        let col_count = col_sizes.len() as i32;
        let row_count = row_sizes.len() as i32;
        let mut next_col = 1;
        let mut next_row = 1;
        let mut boxes = Vec::new();
        for item in items {
            let cs = item.placement.column_start.unwrap_or(next_col);
            let ce = item.placement.column_end.unwrap_or(cs + 1);
            let rs = item.placement.row_start.unwrap_or(next_row);
            let re = item.placement.row_end.unwrap_or(rs + 1);
            let ci = (cs - 1).max(0) as usize;
            let ri = (rs - 1).max(0) as usize;
            let span_cols = (ce - cs).max(1) as usize;
            let span_rows = (re - rs).max(1) as usize;
            let x = col_offsets.get(ci).copied().unwrap_or(0.0);
            let y = row_offsets.get(ri).copied().unwrap_or(0.0);
            let mut w: f32 = col_sizes[ci..].iter().take(span_cols).sum();
            w += self.column_gap * (span_cols as f32 - 1.0).max(0.0);
            let mut h: f32 = row_sizes[ri..].iter().take(span_rows).sum();
            h += self.row_gap * (span_rows as f32 - 1.0).max(0.0);
            if let Some(fixed_w) = item.width {
                w = fixed_w;
            }
            if let Some(fixed_h) = item.height {
                h = fixed_h;
            }
            w = item.clamp_width(w);
            h = item.clamp_height(h);
            boxes.push(GridLayoutBox {
                x,
                y,
                width: w,
                height: h,
            });
            next_col = ce + 1;
            if next_col > col_count {
                next_col = 1;
                next_row = re + 1;
                if next_row > row_count {
                    next_row = row_count;
                }
            }
        }
        boxes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridLayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_track_fixed() {
        let track = GridTrack::Fixed(100.0);
        assert_eq!(track.resolve(500.0), 100.0);
    }

    #[test]
    fn test_grid_track_fraction() {
        let track = GridTrack::Fraction(1.0);
        assert_eq!(track.resolve(500.0), 0.0);
    }

    #[test]
    fn test_grid_placement_span() {
        let p = GridPlacement::span(3, 2);
        assert_eq!(p.column_span(), 3);
        assert_eq!(p.row_span(), 2);
    }

    #[test]
    fn test_grid_placement_explicit() {
        let p = GridPlacement {
            column_start: Some(1),
            column_end: Some(4),
            row_start: Some(1),
            row_end: Some(3),
        };
        assert_eq!(p.column_span(), 3);
        assert_eq!(p.row_span(), 2);
    }

    #[test]
    fn test_resolve_column_sizes_fixed() {
        let grid = GridContainer {
            template_columns: vec![GridTrack::Fixed(100.0), GridTrack::Fixed(200.0)],
            width: 400.0,
            ..GridContainer::new()
        };
        let sizes = grid.resolve_column_sizes();
        assert_eq!(sizes, vec![100.0, 200.0]);
    }

    #[test]
    fn test_resolve_column_sizes_fraction() {
        let grid = GridContainer {
            template_columns: vec![
                GridTrack::Fixed(100.0),
                GridTrack::Fraction(1.0),
                GridTrack::Fraction(1.0),
            ],
            width: 500.0,
            column_gap: 10.0,
            ..GridContainer::new()
        };
        let sizes = grid.resolve_column_sizes();
        assert_eq!(sizes[0], 100.0);
        assert!((sizes[1] - 195.0).abs() < 0.01);
        assert!((sizes[2] - 195.0).abs() < 0.01);
    }

    #[test]
    fn test_resolve_with_gap() {
        let grid = GridContainer {
            template_columns: vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            width: 210.0,
            column_gap: 10.0,
            ..GridContainer::new()
        };
        let sizes = grid.resolve_column_sizes();
        assert_eq!(sizes, vec![100.0, 100.0]);
    }

    #[test]
    fn test_grid_item_clamp() {
        let item = GridItem {
            min_width: Some(50.0),
            max_width: Some(200.0),
            ..GridItem::new()
        };
        assert_eq!(item.clamp_width(10.0), 50.0);
        assert_eq!(item.clamp_width(100.0), 100.0);
        assert_eq!(item.clamp_width(300.0), 200.0);
    }

    #[test]
    fn test_layout_simple() {
        let grid = GridContainer {
            template_columns: vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            template_rows: vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            width: 200.0,
            height: 100.0,
            ..GridContainer::new()
        };
        let items = vec![
            GridItem::new().with_placement(GridPlacement {
                column_start: Some(1),
                column_end: Some(2),
                row_start: Some(1),
                row_end: Some(2),
            }),
            GridItem::new().with_placement(GridPlacement {
                column_start: Some(2),
                column_end: Some(3),
                row_start: Some(1),
                row_end: Some(2),
            }),
        ];
        let boxes = grid.layout(&items);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0].x, 0.0);
        assert_eq!(boxes[0].y, 0.0);
        assert_eq!(boxes[1].x, 100.0);
        assert_eq!(boxes[1].y, 0.0);
    }

    #[test]
    fn test_layout_auto_placement() {
        let grid = GridContainer {
            template_columns: vec![GridTrack::Fixed(100.0), GridTrack::Fixed(100.0)],
            template_rows: vec![GridTrack::Fixed(50.0)],
            width: 200.0,
            height: 50.0,
            ..GridContainer::new()
        };
        let items = vec![GridItem::new(), GridItem::new()];
        let boxes = grid.layout(&items);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0].x, 0.0);
        assert_eq!(boxes[1].x, 100.0);
    }

    #[test]
    fn test_column_and_row_count() {
        let grid = GridContainer {
            template_columns: vec![
                GridTrack::Fixed(100.0),
                GridTrack::Fixed(100.0),
                GridTrack::Fixed(100.0),
            ],
            template_rows: vec![GridTrack::Fixed(50.0), GridTrack::Fixed(50.0)],
            ..GridContainer::new()
        };
        assert_eq!(grid.column_count(), 3);
        assert_eq!(grid.row_count(), 2);
    }
}
