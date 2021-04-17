/*!
# fltk-rs-calendar

A calendar dialog for fltk-rs. It's separated into its own crate since it requires a dependency on `chrono`, also the Calendar::get_date() method returns a chrono::NaiveDate.

## Usage
Add the following to your Cargo.toml:
```toml
[dependencies]
fltk-calendar = "0.3"
```

Then the dialog can be instatiated use the Calendar::new(x, y) or Calendar::default() functions. And the date can be chosen by double clicking on a cell.

```rust
use fltk::{prelude::*, *};
use fltk_calendar::calendar;
use chrono::prelude::*;

let app = app::App::default().with_scheme(app::Scheme::Gtk);
let mut win = window::Window::new(100, 100, 400, 300, "");
let mut but = button::Button::new(160, 200, 80, 40, "Click");
win.end();
win.show();
but.set_callback(move |_| {
    let cal = calendar::Calendar::default(); // or calendar::Calendar::new(200, 100);
    let date = cal.get_date();
    println!("{:?}", date);
    if let Some(date) = date {
        println!("{:?}", date.year());
        println!("{:?}", date.month());
        println!("{:?}", date.day());
    }
});
app.run().unwrap();
```
*/

/// Defines a calendar widget
pub mod calendar;