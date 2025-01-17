extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    // Initialisation de SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Création de la fenêtre sans couleur spécifiée (fenêtre vide)
    let window = video_subsystem.window("Fenêtre Vide SDL2", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    // Création du canevas pour dessiner sur la fenêtre (mais on ne dessine rien ici)
    let mut canvas = window.into_canvas().build().unwrap();

    // Positionner le point d'origine en bas à gauche
    let x = 0;
    let y = 800 - 1;
    canvas.draw_point((x, y)).unwrap();
 
    // canvas.set_draw_color(Color::RGB(255, 0, 0)); // Rouge
    // canvas.draw_point((400, 400)).unwrap();


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