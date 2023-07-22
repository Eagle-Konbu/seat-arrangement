use itertools::Itertools;
use printpdf::{Line, Mm, PdfDocument, Point, Pt};

const TTF_FILE: &[u8] = include_bytes!("../assets/fonts/ipaexg.ttf");

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn mm2pt(mm: Mm) -> Pt {
    Pt(mm.0 * 2.83465)
}

fn pt2mm(pt: Pt) -> Mm {
    Mm(pt.0 / 2.83465)
}

fn text_width(text: &str, font_size: Pt) -> Pt {
    let mut res = 0.0;
    for c in text.chars() {
        if c.is_ascii() {
            res += font_size.0 * 0.5;
        } else {
            res += font_size.0;
        }
    }

    Pt(res)
}

pub fn gen(seats: Vec<Vec<String>>) -> Result<Vec<u8>, printpdf::Error> {
    let (seat_width, seat_height) = (seats[0].len(), seats.len());
    let (page_width, page_height) = (297.0, 210.0);
    let outline_margin_length = 15.0;
    let seat_margin_length = 5.0;

    let (doc, page1, layer1) =
        PdfDocument::new("Seat Layout", Mm(page_width), Mm(page_height), "main");

    let (rect_width, rect_height) = (
        (page_width - outline_margin_length * 2.0 - seat_margin_length * (seat_width - 1) as f64)
            / seat_width as f64,
        (page_height - outline_margin_length * 2.0 - seat_margin_length * (seat_height - 1) as f64)
            / seat_height as f64,
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let cursor = std::io::Cursor::new(TTF_FILE);
    let font = doc.add_external_font(cursor).unwrap();

    for (j, i) in (0..seat_width).cartesian_product(0..seat_height) {
        let name = &seats[seat_height - i - 1][j];

        let left_upper = Point {
            x: mm2pt(Mm(
                outline_margin_length + (rect_width + seat_margin_length) * j as f64
            )),
            y: mm2pt(Mm(
                outline_margin_length + (rect_height + seat_margin_length) * i as f64
            )),
        };
        let left_lower = Point {
            x: left_upper.x,
            y: left_upper.y + mm2pt(Mm(rect_height)),
        };
        let right_upper = Point {
            x: left_upper.x + mm2pt(Mm(rect_width)),
            y: left_upper.y,
        };
        let right_lower = Point {
            x: left_upper.x + mm2pt(Mm(rect_width)),
            y: left_upper.y + mm2pt(Mm(rect_height)),
        };

        let line = Line {
            points: vec![
                (left_upper, false),
                (left_lower, false),
                (right_lower, false),
                (right_upper, false),
            ],
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        current_layer.add_shape(line);

        let text_x =
            left_upper.x + mm2pt(Mm(rect_width / 2.0)) - text_width(name, Pt(17.0)) / 2.0;
        let text_y = left_upper.y + mm2pt(Mm(rect_height / 2.0)) - Pt(17.0) / 2.0;

        current_layer.use_text(name, 17.0, pt2mm(text_x), pt2mm(text_y), &font);
    }

    doc.save_to_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
