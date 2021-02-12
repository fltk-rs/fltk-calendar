// Example of a command-line app which just shows a calendar dialog

use fltk_calendar::calendar;

fn main() {
    println!("Getting date");
    let cal = calendar::Calendar::default();
    let date = cal.get_date();
    println!("{:?}", date);
}