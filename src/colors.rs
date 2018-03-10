use conrod::Color;

use std::collections::HashMap;

pub struct ColorScheme {
    foreground: Color,
    background: Color,
    additionnal: Vec<Color>,
}

lazy_static! {
    static ref COLOR_SCHEMES: HashMap<&'static str, ColorScheme> = {
        let mut m = HashMap::new();

        m.insert("default", ColorScheme {
            foreground: Color::Rgba(1.0, 1.0, 1.0, 1.0),
            background: Color::Rgba(0.172549, 0.243137, 0.313725, 1.0),
            additionnal: vec![
                Color::Rgba(0.752941, 0.223529, 0.168627, 1.0),
                Color::Rgba(0.827451, 0.329412, 0.0, 1.0),
                Color::Rgba(0.952941, 0.611765, 0.070588, 1.0),
                Color::Rgba(0.160784, 0.501961, 0.72549, 1.0),
                Color::Rgba(0.152941, 0.682353, 0.376471, 1.0),
                Color::Rgba(0.086275, 0.627451, 0.521569, 1.0),
                Color::Rgba(0.556863, 0.266667, 0.678431, 1.0),
            ],
        });

        m
    };
}

pub fn get_background(colorscheme: &'static str) -> Color {
    COLOR_SCHEMES[colorscheme].background
}

pub fn get_foreground(colorscheme: &'static str) -> Color {
    COLOR_SCHEMES[colorscheme].foreground
}

pub fn get_additionnal(colorscheme: &'static str, index: usize) -> Color {
    let colorscheme = &COLOR_SCHEMES[colorscheme];
    let max_index = colorscheme.additionnal.len();

    colorscheme.additionnal[index % max_index]
}
