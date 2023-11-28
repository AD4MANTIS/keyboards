use plotters::{
    backend::BitMapBackend,
    style::{Color, FontDesc, RGBColor, TextStyle},
};
use plotters_backend::{text_anchor, BackendColor, DrawingBackend, FontFamily, FontStyle};

use crate::models::layout_map::KeyboardKey;

// KEYBOARD FUNCTIONS
pub(crate) fn draw_keyboard<const N: usize>(
    my_genome: &[char; N],
    id: &str,
    layout_map: &[KeyboardKey; N],
) {
    let file_name = format!("results/{}.png", id);

    const IMG_HEIGHT: i32 = 600;
    let mut plt = BitMapBackend::new(&file_name, (1400, IMG_HEIGHT as u32));
    // plt.into_drawing_area().fill(&plotters::style::RGBColor(255, 255, 255)).unwrap();

    for i in 0..N {
        let layout = &layout_map[i];

        let x = layout.x;
        let y = IMG_HEIGHT - layout.y;

        let letter = my_genome[i];
        let mut my_color = RGBColor(176, 176, 176);

        if letter == 'E' {
            my_color = RGBColor(0, 255, 255); // cyan
        } else if ["T", "A", "O", "I", "N", "S", "R", "H", "L"]
            .contains(&letter.to_string().as_str())
        {
            my_color = RGBColor(0, 238, 118); // springgreen2
        } else if ["[", "]", "~", "+", "7", "4", "6", "3", "8", "5"]
            .contains(&letter.to_string().as_str())
        {
            my_color = RGBColor(255, 99, 71); // tomato
        }

        let my_color = my_color.to_rgba();

        if layout.home {
            plt.draw_rect(
                (x, y),
                (x, y),
                &plotters::style::ShapeStyle {
                    color: my_color.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
                true,
            )
            .unwrap();
        }

        plt.draw_rect(
            (x - 45, y - 45),
            (x + 45, y + 45),
            &plotters::style::ShapeStyle {
                color: my_color,
                filled: true,
                stroke_width: 1,
            },
            true,
        )
        .unwrap();

        plt.draw_text(
            &letter.to_string(),
            &TextStyle {
                font: FontDesc::new(FontFamily::SansSerif, 60., FontStyle::Normal),
                pos: text_anchor::Pos::default(),
                color: BackendColor {
                    alpha: 1.,
                    rgb: (255, 255, 255),
                },
            },
            (x - 10, y - 10),
        )
        .unwrap();
    }

    plt.present().unwrap();
}
