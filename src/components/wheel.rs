use Ids;
use colors;

use conrod::{self, widget, Positionable, Widget, UiCell};

use std;

pub fn create_wheel(num_of_parts: usize, ui: &mut UiCell, ids: &mut Ids) {

    ids.wheel_parts.resize(num_of_parts, &mut ui.widget_id_generator());
    ids.wheel_labels.resize(num_of_parts, &mut ui.widget_id_generator());

    let window_size = ui.window_dim();
    let min_window_size = window_size[0].min(window_size[1]);

    const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

    let angle = TWO_PI / (num_of_parts as f64);

    let mut i = 0;
    for (&part_id, &label_id)  in ids.wheel_parts.iter().zip(ids.wheel_labels.iter()) {

        let angle_offset = angle * (i as f64);
        let color = colors::get_additionnal("default", i);

        widget::Circle::fill_with(min_window_size / 2.3f64, color)
            .section(angle)
            .offset_radians(angle_offset)
            .middle_of(ui.window)
            .set(part_id, ui);

        widget::Text::new("Hello, World!")
            .down_from(part_id, 0f64)
            .set(label_id, ui);

        i += 1;
    }

}
