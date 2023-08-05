use comfy_table::{presets::UTF8_FULL, ContentArrangement, Row, Table as ComfyTable, Cell, Color};
use day_core::time::HourMinute;

pub trait TableFmt
where
    Self: Sized,
{
    fn headers() -> Vec<&'static str>;
    fn row(self) -> Row;

    fn to_comfy_table(self) -> ComfyTable {
        let mut table = ComfyTable::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(&Self::headers());

        table.add_row(self.row());

        table
    }

    fn print_single(self) {
        println!("{}", self.to_comfy_table());
    }

    fn iter_table(iter: impl IntoIterator<Item = Self>) -> ComfyTable {
        let mut table = ComfyTable::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(&Self::headers());

        for item in iter {
            table.add_row(item.row());
        }

        table
    }

    fn print_iter(iter: impl IntoIterator<Item = Self>) {
        println!("{}", Self::iter_table(iter));
    }
}

#[extension_trait]
pub impl DurationTableFmt for HourMinute {
    fn to_cell_duration(&self) -> Cell {
        let mut base = Cell::new(self.to_string());
        match self.to_minutes() {
            0..=30 => base = base.fg(Color::Green),
            31..=60 => base = base.fg(Color::Yellow),
            _ => base = base.fg(Color::Red),
        }
        base
    }
}