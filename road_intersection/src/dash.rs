use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn draw_dashed_line(
    canvas: &mut Canvas<Window>,
    start: (i32, i32),
    end: (i32, i32),
    dash_length: i32,
    space_length: i32,
) {
    let (mut current_x, y) = start;
    let end_x = end.0;

    while current_x < end_x {
        let next_x = (current_x + dash_length).min(end_x); // Limite à la fin de la ligne
        canvas
            .draw_line(Point::new(current_x, y), Point::new(next_x, y))
            .unwrap();
        current_x = next_x + space_length; // Ajouter l'espace entre les tirets
    }
}
