use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;


pub fn draw_text(
    canvas: &mut Canvas<Window>,
    ttf_context: &Sdl2TtfContext,
    text: &str,
    font_path: &str,
    font_size: u16,
    position: (i32, i32),
    color: Color,
) {
    // Charger la police depuis le fichier
    let font = ttf_context.load_font(font_path, font_size).unwrap();

    // Créer la texture du texte à partir de la police
    let surface = font.render(text)
        .blended(color)
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    // Récupérer la taille du texte
    let (text_width, text_height) = surface.size();

    // Dessiner le texte à la position donnée
    canvas.copy(&texture, None, Some(sdl2::rect::Rect::new(position.0, position.1, text_width, text_height)))
        .unwrap();
}
