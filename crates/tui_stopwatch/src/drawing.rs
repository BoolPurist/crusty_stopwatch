pub const STOPWATCH_LIST_NAME: &str = "list";

use crate::user_data;
use core_stopwatch::{formating, Stopwatch, Timer};
use cursive::{
    view::Nameable,
    views::{LinearLayout, Panel, TextView},
    Cursive, View,
};

pub fn draw_stopwatch_list(tui: &mut Cursive) {
    let laytout = {
        let mut laytout = LinearLayout::vertical();

        update_tui_stopwatch_list(&mut laytout);
        laytout.with_name(STOPWATCH_LIST_NAME)
    };
    tui.add_layer(laytout);
}

pub fn update_tui_stopwatch_list(layout: &mut LinearLayout) {
    layout.clear();

    user_data::get_data(|data| {
        let tui_watches = data
            .iter_stopwatches()
            .map(|stop_watch| create_stopwatch(stop_watch));

        for next in tui_watches {
            layout.add_child(next);
        }
    });
}

fn create_stopwatch(data: &Stopwatch) -> impl View {
    let mut layout = LinearLayout::vertical();
    if let Some(title) = data.title() {
        layout.add_child(TextView::new(title));
    }

    {
        let created = format!("Created: {}", formating::utc_date_to_row(data.started()));
        let duration = format!("Elapsed: {}", formating::duration_to_row(data.elapsed()));
        layout.add_child(TextView::new(duration));
        layout.add_child(TextView::new(created));
    }

    Panel::new(layout)
}
