use chrono::Local;
use comfy_table::{Row, Cell, Color};
use day_core::modules::task::{Task, CompletedTask, Deadline};

use crate::table::TableFmt;



impl TableFmt for Task {
    fn headers() -> Vec<&'static str> {
        ["Name", "Notes", "Duration", "Deadline", "Urgency"].into()
    }

    fn row(self) -> Row {
        let mut inner = self.event.row();
        for cell in [
            self.deadline
                .as_ref()
                .map(|e| e.to_cell())
                .unwrap_or(Cell::new("")),
            self.urgency.to_cell(),
        ] {
            inner.add_cell(cell);
        }

        inner
    }
}

impl TableFmt for CompletedTask {
    fn headers() -> Vec<&'static str> {
        let mut base = Task::headers();
        base.push("Completed At");
        base
    }

    fn row(self) -> Row {
        let mut base = self.task.row();
        base.add_cell(self.completed_at.to_string().into());
        base
    }
}

#[extension_trait]
pub impl UrgencyTableFmt for u8 {
    fn to_cell(&self) -> Cell {
        let mut base = Cell::new(self.to_string());
        match self {
            0..=3 => base = base.fg(Color::Green),
            4..=5 => base = base.fg(Color::Yellow),
            6..=7 => base = base.fg(Color::Red),
            _ => (),
        };
        match self {
            0..=3 => (),
            4..=5 => base = base.add_attribute(comfy_table::Attribute::Italic),
            6..=7 => base = base.add_attribute(comfy_table::Attribute::Bold),
            _ => (),
        }
        base
    }
}

#[extension_trait]
pub impl DeadlineTableFmt for Deadline {
    fn to_cell(&self) -> Cell {
        let mut base = Cell::new(self.to_string());
        match self {
            Deadline::Date(date) => {
                let now = Local::now().naive_local().date();
                let days = date.signed_duration_since(now).num_days();
                match days {
                    i64::MIN..=-1 => base = base.fg(Color::Red),
                    0..=1 => base = base.fg(Color::Yellow),
                    _ => base = base.fg(Color::Green),
                }
            }
            Deadline::DateTime(date) => {
                let now = Local::now();
                let hours = date.signed_duration_since(now).num_hours();
                match hours {
                    i64::MIN..=-1 => base = base.fg(Color::Red),
                    0..=47 => base = base.fg(Color::Yellow),
                    48..=i64::MAX => base = base.fg(Color::Green),
                }
            }
        }
        base
    }
}

