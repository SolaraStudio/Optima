
pub struct TableLayout;

#[derive(Debug, Clone, Default)]
pub struct TableGrid {
    pub columns: usize,
    pub rows: usize,
    pub column_widths: Vec<f32>,
    pub row_heights: Vec<f32>,
}

impl TableLayout {
    pub fn layout_table(column_count: usize, row_count: usize, available_width: f32) -> TableGrid {
        let col_width = available_width / column_count as f32;
        TableGrid {
            columns: column_count,
            rows: row_count,
            column_widths: vec![col_width; column_count],
            row_heights: vec![30.0; row_count],
        }
    }

    pub fn cell_position(grid: &TableGrid, col: usize, row: usize, table_x: f32, table_y: f32) -> (f32, f32, f32, f32) {
        let mut x = table_x;
        for c in 0..col { x += grid.column_widths[c]; }
        let mut y = table_y;
        for r in 0..row { y += grid.row_heights[r]; }
        (x, y, grid.column_widths[col], grid.row_heights[row])
    }
}
