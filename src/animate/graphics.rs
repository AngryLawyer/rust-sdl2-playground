use std::path;
use std::io;
use sdl2::render;
use sdl2::surface;
use sdl2::rect;


struct GraphicAsset {
    texture: ~render::Texture,
    frames: ~[rect::Rect],
    width: int,
    height: int
}


impl GraphicAsset {

    pub fn new(renderer: &render::Renderer, filename: &str) -> Result<~GraphicAsset, ~str> {
        surface::Surface::from_bmp(&path::GenericPath::from_str(filename)).and_then(|surface| {
            renderer.create_texture_from_surface(surface).and_then(|texture| {
                let (width, height) = match texture.query() {
                    Ok(query) => {
                        (query.width, query.height)
                    },
                    Err(msg) => {
                        io::println(fmt!("WARNING: %s", msg));
                        (0, 0)
                    }
                };
                Ok(~GraphicAsset{
                    texture: texture,
                    frames: ~[rect::Rect::new(0, 0, width as i32, height as i32)],
                    width: width,
                    height: height
                })
            })
        })
    }

    pub fn new_with_frames(renderer: &render::Renderer, filename: &str, frames: &[rect::Rect]) -> Result<~GraphicAsset, ~str> {
        surface::Surface::from_bmp(&path::GenericPath::from_str(filename)).and_then(|surface| {
            renderer.create_texture_from_surface(surface).and_then(|texture| {
                let (width, height) = match texture.query() {
                    Ok(query) => {
                        (query.width, query.height)
                    },
                    Err(msg) => {
                        io::println(fmt!("WARNING: %s", msg));
                        (0, 0)
                    }
                };
                Ok(~GraphicAsset{
                    texture: texture,
                    frames: frames.to_owned(), 
                    width: width,
                    height: height
                })
            })
        })

    }

    pub fn draw(&self, renderer: &render::Renderer, center: rect::Point, rotation: f64, frame: uint) {
        if frame >= self.frames.len() {
            return;
        }

        let frame_rect = &self.frames[frame];

        renderer.copy_ex(
            self.texture,
            Some(self.frames[frame]),
            Some(rect::Rect::new(center.x - (frame_rect.w/2) as i32, center.y - (frame_rect.h/2) as i32, frame_rect.w, frame_rect.h)),
            rotation,
            None,
            render::FlipNone
        );
    }
}

pub fn generate_frames(width: uint, height: uint, frames: uint) -> ~[rect::Rect] {
    let mut out: ~[rect::Rect] = ~[];

    for frame in range(0, frames) {
        out.push(rect::Rect::new((frame * width) as i32, 0, width as i32, height as i32));
    }
    out
}
