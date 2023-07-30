use chrono::Local;
use comfy_table::{Row, Cell, Color};
use day_core::modules::todos::{Todo, CompletedTodo, Deadline};

use crate::table::TableFmt;


impl TableFmt for Todo {
    fn headers() -> Vec<&'static str> {
        ["Name", "Notes", "Duration", "Deadline", "Urgency"].into()
    }

    fn row(self) -> Row {
        vec![
            self.name.into(),
            self.notes.into(),
            self.duration.to_string().into(),
            self.deadline
                .as_ref()
                .map(|e| e.to_cell())
                .unwrap_or(Cell::new("None")),
            self.urgency.to_cell(),
        ]
        .into()
    }
}

impl TableFmt for CompletedTodo {
    fn headers() -> Vec<&'static str> {
        let mut base = Todo::headers();
        base.push("Completed At");
        base
    }

    fn row(self) -> Row {
        let mut base = self.todo.row();
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
/// Formats a deadline for display in a table. Deadlines that have passed are red, deadlines that are within 48 hours are yellow, and deadlines that are more than 48 hours away are green.
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
