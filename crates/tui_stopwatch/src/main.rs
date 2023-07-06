mod drawing;
mod user_data;

pub const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

use core_stopwatch::data_source;
use core_stopwatch::file_access;
use core_stopwatch::prelude::*;
use core_stopwatch::Timers;
use cursive::event::Event;
use cursive::{Cursive, CursiveExt};

fn main() {
    let mut tui = Cursive::default();

    {
        let data = setup().unwrap();
        user_data::set_data(data.clone());
    }

    tui.add_global_callback('q', |s| s.quit());

    drawing::draw_stopwatch_list(&mut tui);

    tui.set_autorefresh(true);
    tui.add_global_callback(Event::Refresh, |tui| fps_update(tui));

    tui.run();
}

fn fps_update(tui: &mut Cursive) {
    let mut layout = tui.find_name(drawing::STOPWATCH_LIST_NAME).unwrap();
    drawing::update_tui_stopwatch_list(&mut layout);
}

fn setup() -> AppResult<Timers> {
    file_access::handle_env_file(PROJECT_ROOT);

    env_logger::init();
    let path = file_access::get_path_to_data(PROJECT_ROOT)?;

    data_source::load_timers_from(&path).map_err(AppError::from)
}
