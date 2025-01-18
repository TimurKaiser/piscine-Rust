use sdl2::ttf::Font;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;

pub struct TextRenderer<'a> {
    font: Font<'a, 'a>,
}

impl<'a> TextRenderer<'a> {
    // Fonction pour créer un TextRenderer avec la police chargée
    pub fn new(ttf_context: &'a Sdl2TtfContext, font_path: &str, font_size: u16) -> Self {
        let font = ttf_context.load_font(font_path, font_size).unwrap();
        TextRenderer { font }
    }

    // Fonction pour dessiner le texte
    pub fn draw_text(
        &self,
        canvas: &mut Canvas<Window>,
        text: &str,
        position: (i32, i32),
        color: Color,
    ) {
        let surface = self.font.render(text)
            .blended(color)
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        // Récupérer la taille du texte
        let (text_width, text_height) = surface.size();

        // Dessiner le texte à la position donnée
        canvas.copy(
            &texture,
            None,
            Some(sdl2::rect::Rect::new(position.0, position.1, text_width, text_height)),
        )
        .unwrap();
    }
}
