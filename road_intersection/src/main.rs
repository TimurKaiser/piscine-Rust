extern crate sdl2;
mod dash;
mod text;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use dash::draw_dashed_line;
use dash::draw_vertical_dashed_line;
use text::draw_text;

fn main() {
    // Initialisation de SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    // Création de la fenêtre sans couleur spécifiée (fenêtre vide)
    let window = video_subsystem.window("Fenêtre Vide SDL2", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    // Création du canevas pour dessiner sur la fenêtre
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Blanc

    draw_dashed_line(&mut canvas, (11, 400), (290, 400), 5, 5);
    draw_dashed_line(&mut canvas, (510, 400), (789, 400), 5, 5);

    draw_vertical_dashed_line(&mut canvas, (400, 11), (290, 290), 5, 5);
    draw_vertical_dashed_line(&mut canvas, (400, 510), (400, 789), 5, 5);

    let _ = canvas.draw_line((300, 1), (300, 300));
    let _ = canvas.draw_line((300, 300), (1, 300));

    let _ = canvas.draw_line((300, 500), (300, 799));
    let _ = canvas.draw_line((1, 500), (300, 500));

    let _ = canvas.draw_line((500, 1), (500, 300));
    let _ = canvas.draw_line((500, 300), (799, 300));

    let _ = canvas.draw_line((500, 500), (500, 799));
    let _ = canvas.draw_line((500, 500), (799, 500));

    // Utilisation de la fonction draw_text avec un chemin relatif
    draw_text(
        &mut canvas,
        &ttf_context,
        "TEST",
        "/home/baran/Z01/piscine-Rust/road_intersection/src/font/DejaVuSans.ttf", // Chemin relatif vers la police
        24,
        (400, 400), // Position x, y
        Color::RGB(0, 0, 255), // Couleur du texte
    );
    
    canvas.present();

    // Boucle d'événements pour gérer la fermeture de la fenêtre
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return;
                },
                _ => {}
            }
        }
    }
}
