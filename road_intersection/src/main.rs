extern crate sdl2;
mod dash;
mod text;
mod map;  // Importer le module map

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::text::TextRenderer; // Importer TextRenderer

fn main() {
    // Initialisation de SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    // Création de la fenêtre
    let window = video_subsystem.window("Fenêtre SDL2", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    // Création du canevas pour dessiner sur la fenêtre
    let mut canvas = window.into_canvas().build().unwrap();

    // Créer l'instance de TextRenderer
    let text_renderer = TextRenderer::new(&ttf_context, "/home/baran/Z01/piscine-Rust/road_intersection/src/font/DejaVuSans.ttf", 24);

    // Appeler la fonction draw_map pour dessiner la carte
    map::draw_map(&mut canvas, &text_renderer);

    // Afficher le résultat
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
