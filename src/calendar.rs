use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use fltk::{
    app, draw,
    enums::{Align, Color, Font, FrameType},
    menu,
    prelude::*,
    group, table, window,
};
use std::{cell::RefCell, rc::Rc};

const DAYS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Defines a calendar dialog
pub struct Calendar {
    wind: window::Window,
    table: table::TableRow,
    month_choice: menu::Choice,
    year_choice: menu::Choice,
}

impl Calendar {
    /// Creates a new calendar dialog
    pub fn new(x: i32, y: i32) -> Self {
        if !app::is_initialized() {
            app::App::default();
        }
        // get today's date
        let local: DateTime<Local> = Local::now();
        let curr = (local.month() - 1) as i32;
        let curr_year = local.year();
        group::Group::set_current(None::<&group::Group>);
        // create window with month and year choice widgets
        let mut wind = window::Window::new(x, y, 400, 300, "Calendar");
        let mut month_choice = menu::Choice::new(100, 5, 80, 40, "");
        month_choice.add_choice("Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec");
        month_choice.set_value(curr);
        let mut year_choice = menu::Choice::new(220, 5, 80, 40, "");
        for i in 1900..curr_year + 1 {
            year_choice.add_choice(&format!("{}", i));
        }
        year_choice.set_value(curr_year - 1900);
        // Create a table with the days of the selected month
        let mut table = table::TableRow::new(5, 50, 390, 250, "");
        table.set_type(table::TableRowSelectMode::Single);
        table.set_rows(5);
        table.set_cols(7);
        table.set_col_header(true);
        table.set_col_width_all(table.width() / 7);
        table.set_row_height_all(table.height() / 4 - table.col_header_height());
        table.end();
        wind.make_modal(true);
        wind.end();
        wind.show();

        let curr = Rc::from(RefCell::from(curr + 1));
        let curr_year = Rc::from(RefCell::from(curr_year));

        table.draw_cell({
            let curr = curr.clone();
            let curr_year = curr_year.clone();
            move |t, ctx, row, col, x, y, w, h| {
                let curr_year = curr_year.borrow();
                let curr = curr.borrow();
                let first =
                NaiveDate::from_ymd_opt(*curr_year, *curr as u32, 1).unwrap().weekday() as i32;
                let day_idx = col + first;
                let day_idx = if day_idx > 6 { day_idx - 7 } else { day_idx };
                match ctx {
                    table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                    table::TableContext::ColHeader => {
                        let day = DAYS[day_idx as usize];
                        draw_header(day, x, y, w, h)
                    }
                    table::TableContext::Cell => {
                        let day = row * 7 + col + 1;
                        let max_days = match *curr {
                            1 => 31,
                            2 => {
                                if *curr_year % 4 == 0 {
                                    29
                                } else {
                                    28
                                }
                            }
                            3 => 31,
                            4 => 30,
                            5 => 31,
                            6 => 30,
                            7 => 31,
                            8 => 31,
                            9 => 30,
                            10 => 31,
                            11 => 30,
                            12 => 31,
                            _ => unreachable!(),
                        };
                        if day < (max_days + 1) {
                            draw_data(day, x, y, w, h, t.is_selected(row, col));
                        }
                    }
                    _ => (),
                }
            }
        });

        // redraw table when the month changes
        month_choice.set_callback(move |c| {
            *curr.borrow_mut() = c.value() + 1;
            c.parent().unwrap().redraw();
        });

        // redraw table when the year changes
        year_choice.set_callback(move |c| {
            *curr_year.borrow_mut() = c.value() + 1900;
            c.parent().unwrap().redraw();
        });

        // choose the day by double clicking a cell
        table.set_callback(|t| {
            if app::event_clicks() {
                t.top_window().unwrap().hide();
            }
            
        });

        // Keep the window shown awaiting input
        while wind.shown() {
            app::wait();
        }

        Self {
            wind,
            table,
            month_choice,
            year_choice,
        }
    }
    /// Creates a new calendar widget with a default position
    pub fn default() -> Self {
        let mut s = Self::new(0, 0);
        s.wind.free_position();
        s
    }
    /// Get the date selected by the calendar dialog
    pub fn get_date(&self) -> Option<chrono::naive::NaiveDate> {
        // get table selection
        let (r, c, _, _) = self.table.get_selection();
        if r == -1 || c == -1 {
            None
        } else {
            let day = r * 7 + c + 1;
            NaiveDate::from_ymd_opt(
                self.year_choice.value() + 1900,
                self.month_choice.value() as u32 + 1,
                day as u32,
            )
        }
    }
}

// draw header with day names
fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

// draw the numbers
fn draw_data(day: i32, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xbcd9ea));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(&format!("{}", day), x, y, w, h, Align::Center);
    draw::pop_clip();
}
