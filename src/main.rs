#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate simplemap;
extern crate ncurses;
extern crate hex2d;
extern crate hex2d_dpcext as hex2dext;
extern crate rand;
extern crate num;
extern crate schedule_recv;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate fern;

mod ai;
mod curses;
mod game;
mod generate;
mod util;
mod logging;

fn main() {
    logging::init();

    let mut ui = curses::Ui::new().unwrap();

    ui.run();
}
