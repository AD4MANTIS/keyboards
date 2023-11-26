use plotters::{
    backend::BitMapBackend,
    style::{Color, FontDesc, RGBColor, TextStyle},
};
use plotters_backend::{text_anchor, BackendColor, DrawingBackend, FontFamily, FontStyle};

use crate::models::layout_map::Layout;

pub(crate) fn draw_keyboard<const N: usize>(
    my_genome: &[char; N],
    id: &str,
    layout_map: &[Layout; N],
) {
    let file_name = format!("results/{}.png", id);
    let mut plt = BitMapBackend::new(&file_name, (1400, 600));
    // plt.into_drawing_area().fill(&plotters::style::RGBColor(255, 255, 255)).unwrap();

    // const NAMED_COLOURS: [&str; 8] = [
    //     "yellow", "blue", "green", "orange", "pink", "green", "blue", "yellow",
    // ]; // https://www.farb-tabelle.de/en/rgb2hex.htm

    for i in 0..N {
        let layout = &layout_map[i];
        let letter = my_genome[i];
        let mut my_colour = RGBColor(176, 176, 176);
        // myColour = NAMED_COLOURS[finger]

        if letter == 'E' {
            my_colour = RGBColor(0, 255, 255); // cyan
        } else if ["T", "A", "O", "I", "N", "S", "R", "H", "L"]
            .contains(&letter.to_string().as_str())
        {
            my_colour = RGBColor(0, 238, 118); // springgreen2
        } else if ["[", "]", "~", "+", "7", "4", "6", "3", "8", "5"]
            .contains(&letter.to_string().as_str())
        {
            my_colour = RGBColor(255, 99, 71); // tomato
        }

        let my_colour = my_colour.to_rgba();

        if layout.home {
            plt.draw_rect(
                ((layout.x * 100.) as i32, (layout.y * 100.) as i32),
                ((layout.x * 100.) as i32, (layout.y * 100.) as i32),
                &plotters::style::ShapeStyle {
                    color: my_colour.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
                true,
            )
            .unwrap();
        }

        plt.draw_rect(
            (
                ((layout.x - 0.45) * 100.) as i32,
                ((layout.y - 0.45) * 100.) as i32,
            ),
            (
                ((layout.x + 0.45) * 100.) as i32,
                ((layout.y + 0.45) * 100.) as i32,
            ),
            &plotters::style::ShapeStyle {
                color: my_colour,
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
            ((layout.x * 100.) as i32, (layout.y * 100.) as i32),
        )
        .unwrap();
    }

    plt.present().unwrap();
}
