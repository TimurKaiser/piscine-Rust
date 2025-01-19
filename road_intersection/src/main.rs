extern crate sdl2;
mod dash;
mod text;
mod map;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;  // Importation de Rect pour dessiner la voiture
use crate::text::TextRenderer;
use sdl2::ttf;  // Ajout de l'importation pour ttf

struct Car {
    x:          i32,
    y:          i32,
    velocity:   i32,
}

impl Car {
    // Déplacer la voiture vers la gauche
    fn moove(&mut self) {
        self.x -= self.velocity;
    }

    // Créer et dessiner la voiture sur le canevas
    fn crate_car(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Couleur de la voiture (rouge)
        let car_dimention = Rect::new(self.x, self.y, 50, 30); // Dimensions de la voiture
        canvas.fill_rect(car_dimention).unwrap(); // Dessiner la voiture
    }

    // Réinitialiser la position de la voiture si elle dépasse l'écran
    fn car_despawn(&mut self, window_size: i32) {
        if self.x < 0 {
            self.x = window_size; // Remettre la voiture à droite de l'écran
        }
    }
}

fn main() {
    // Initialisation de SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Initialisation de SDL_ttf
    let ttf_context = sdl2::ttf::init().unwrap(); // Décommente et initialise ttf_context

    // Création de la fenêtre
    let window = video_subsystem
        .window("Fenêtre SDL2", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    // Création du canevas pour dessiner sur la fenêtre
    let mut canvas = window.into_canvas().build().unwrap();

    // Créer l'instance de TextRenderer
    let text_renderer = TextRenderer::new(&ttf_context, "/home/baran/Z01/piscine-Rust/road_intersection/src/font/DejaVuSans.ttf", 18);

    // Gestionnaire d'événements pour interagir avec la fenêtre
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Créer l'instance de la voiture
    let mut car = Car {
        x:          800,  // Position initiale (à droite)
        y:          300,  // Position verticale de la voiture
        velocity:   5,    // Vitesse de la voiture
    };

    let window_size = 800;  // Taille de la fenêtre

    // Appeler la fonction draw_map pour dessiner la carte
    map::draw_map(&mut canvas, &text_renderer);

    // Afficher le résultat initial
    canvas.present();

    // Boucle principale pour gérer les événements et le rendu
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return; // Quitter l'application si l'utilisateur ferme la fenêtre ou appuie sur Échap
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    car.moove();  // Déplacer la voiture à gauche lorsqu'on appuie sur la flèche gauche
                },
                _ => {}
            }
        }

        // Nettoyer le canevas avec un fond noir
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Dessiner la carte (en supposant que la fonction `map::draw_map` dessine la carte)
        map::draw_map(&mut canvas, &text_renderer);

        // Dessiner la voiture
        car.crate_car(&mut canvas);

        // Réinitialiser la position de la voiture si elle sort de l'écran
        car.car_despawn(window_size);

        // Afficher le résultat à l'écran
        canvas.present();

        // Limiter la vitesse de la boucle (environ 60 FPS)
        std::thread::sleep(std::time::Duration::from_millis(16)); // 60 FPS
    }
}


//extern crate sdl2;
//mod dash;
//mod text;
//mod map;
//
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
//use sdl2::pixels::Color;
//use crate::text::TextRenderer;
//
//
//
//struct Car {
//    x:          i32,
//    y:          i32,
//    velocity:   i32,
//}
//
//
//impl Car {
//    fn moove(&mut self) {
//        self.x -= self.velocity;
//    }
//
//    fn crate_car(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
//        canvas.set_draw_color(Color::RGB(255, 0, 0));
//        let car_dimention = Rect::new(self.x, self.y, 50, 30);
//        canvas.fill_rect(car_dimention).unwrap();
//    }
//
//    fn car_despawn(&mut self, window_size: i32) {
//        if self.x < 0 {
//            self.x = window_size;
//        }
//    }
//}
//
//
//
//
//fn main() {
//    // Initialisation de SDL2
//    let sdl_context = sdl2::init().unwrap();
//    let video_subsystem = sdl_context.video().unwrap();
//    // let ttf_context = sdl2::ttf::init().unwrap();
//
//    // Création de la fenêtre
//    let window = video_subsystem
//        .window("Fenêtre SDL2", 800, 800)
//        .position_centered()
//        .build()
//        .unwrap();
//
//    // Création du canevas pour dessiner sur la fenêtre
//    let mut canvas = window.into_canvas().build().unwrap();
//
//    // Créer l'instance de TextRenderer
//    let text_renderer = TextRenderer::new(&ttf_context, "/home/baran/Z01/piscine-Rust/road_intersection/src/font/DejaVuSans.ttf", 18);
//
//    // askip pour le key
//    let mut event_pump = sdl_context.event_pump().unwrap();
//
//    let mut car = Car {
//        x:          800,
//        y:          300,
//        velocity:   5,
//    };
//
//    let window_size = 800;
//
//    // Appeler la fonction draw_map pour dessiner la carte
//    map::draw_map(&mut canvas, &text_renderer);
//
//    // Boucle d'événements pour gérer la fermeture de la fenêtre
//    loop {
//        for event in event_pump.poll_iter() {
//            match event {
//                Event::Quit { .. } |
//                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                    return; // Quitter l'application si l'utilisateur ferme la fenêtre ou appuie sur Échap
//                },
//                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
//                    car.moove();  // Déplacer la voiture à gauche lorsqu'on appuie sur la flèche gauche
//                },
//                _ => {}
//            }
//        }
//    }
//
//    car.car_despawn(window_size);
//
//    canvas.present();
//
//}
//