use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::text::TextRenderer;
use crate::dash::{draw_dashed_line, draw_vertical_dashed_line};

pub fn draw_map( canvas: &mut Canvas<Window>, text_renderer: &TextRenderer ) {
    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Blanc

    // Dessiner des lignes et des points
    draw_dashed_line(canvas, (11, 400), (290, 400), 5, 5);
    draw_dashed_line(canvas, (510, 400), (789, 400), 5, 5);
    draw_vertical_dashed_line(canvas, (400, 11), (290, 290), 5, 5);
    draw_vertical_dashed_line(canvas, (400, 510), (400, 789), 5, 5);
    
    canvas.draw_line((300, 1), (300, 300)).unwrap();
    canvas.draw_line((300, 300), (1, 300)).unwrap();
    canvas.draw_line((300, 500), (300, 799)).unwrap();
    canvas.draw_line((1, 500), (300, 500)).unwrap();
    canvas.draw_line((500, 1), (500, 300)).unwrap();
    canvas.draw_line((500, 300), (799, 300)).unwrap();
    canvas.draw_line((500, 500), (500, 799)).unwrap();
    canvas.draw_line((500, 500), (799, 500)).unwrap();

    let _ = canvas.draw_point((100, 100));
    let _ = canvas.draw_point((250, 100));

    // Dessiner le texte
    text_renderer.draw_text(canvas, "East", (10, 340), Color::RGB(0, 0, 255));
    text_renderer.draw_text(canvas, "East", (10, 440), Color::RGB(0, 0, 255));
    
    text_renderer.draw_text(canvas, "North", (328, 10), Color::RGB(0, 0, 255));
    text_renderer.draw_text(canvas, "North", (428, 10), Color::RGB(0, 0, 255));

    text_renderer.draw_text(canvas, "West", (740, 340), Color::RGB(0, 0, 255));
    text_renderer.draw_text(canvas, "West", (740, 440), Color::RGB(0, 0, 255));
    
    text_renderer.draw_text(canvas, "South", (328, 770), Color::RGB(0, 0, 255));
    text_renderer.draw_text(canvas, "South", (428, 770), Color::RGB(0, 0, 255));




}
