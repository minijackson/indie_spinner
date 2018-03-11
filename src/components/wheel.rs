use colors;

use conrod::{widget, Positionable, Colorable, Widget};

use std;
use std::time::Instant;

#[derive(Clone, Debug, WidgetCommon)]
pub struct Wheel {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    style: Style,
    labels: Vec<String>,
    init_speed: [f64;2],
    spin_drag: f64,
}

impl Wheel {

    pub fn new(labels: Vec<String>) -> Wheel {
        Wheel {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            labels,
            init_speed: [30f64, 40f64],
            spin_drag: 5f64,
        }
    }

}

widget_ids! {
    struct Ids {
        parts[],
        labels[],
    }
}

pub struct State {
    rotation: f64,
    spinning_speed: f64,
    last_frame: Instant,
    ids: Ids,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "\"default\"")]
    pub colorscheme: Option<&'static str>,
}

impl Widget for Wheel {
    type State = State;
    type Style = Style;
    // Sends the name of the randomly choosed event
    type Event = Option<String>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            rotation: 0f64,
            spinning_speed: 0f64,
            last_frame: Instant::now(),
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, ui, .. } = args;
        let Wheel { labels, init_speed, spin_drag, .. } = self;

        let result;

        if state.ids.parts.len() != labels.len() {
            let mut id_gen = &mut ui.widget_id_generator();
            state.update(|state| {
                state.ids.parts.resize(labels.len(), &mut id_gen);
                state.ids.labels.resize(labels.len(), &mut id_gen);
            });
        }

        if state.spinning_speed != 0f64 {
            let now = Instant::now();
            let dt = state.last_frame.duration_since(now);
            let dt = (dt.as_secs() as f64) + f64::from(dt.subsec_nanos()) / 1_000_000.0;
            // spinning_speed -= spin_drag * dt
            // rotation += spinning_speed * dt
            let new_spinning_speed = state.spinning_speed - (spin_drag * dt);
            let new_rotation = state.rotation + (new_spinning_speed * dt);

            state.update(|state| {
                state.spinning_speed = new_spinning_speed;
                state.rotation = new_rotation;
            });

            if new_spinning_speed == 0f64 {
                result = Some(String::from("thing"));
            } else {
                result = None;
            }
        } else {
            result = None;
        }

        let colorscheme = style.colorscheme.unwrap_or("default");
        let window_size = ui.window_dim();
        let min_window_size = window_size[0].min(window_size[1]);

        const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
        let angle = TWO_PI / (labels.len() as f64);

        for (i, (&part_id, &label_id)) in state
            .ids
            .parts
            .iter()
            .zip(state.ids.labels.iter())
            .enumerate()
        {
            let angle_offset = angle * (i as f64);
            let color = colors::get_additionnal(colorscheme, i);

            widget::Circle::fill_with(min_window_size / 2.3f64, color)
                .section(angle)
                .offset_radians(angle_offset)
                .middle_of(ui.window)
                .graphics_for(id)
                .parent(id)
                .set(part_id, ui);

            widget::Text::new(&labels[i])
                // TODO: placement on the section itself + rotation
                .middle_of(part_id)
                .color(colors::get_foreground(colorscheme))
                .graphics_for(id)
                .parent(id)
                .set(label_id, ui);
        }

        result
    }
}
