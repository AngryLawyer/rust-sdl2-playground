extern mod sdl2;

use std::rand;
use std::rand::RngUtil;
use std::io;
use std::path;
use sdl2::render;
use sdl2::event;
use sdl2::pixels;
use sdl2::rect;
use sdl2::keycode;
use sdl2::surface;

mod ratelimiter;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}


fn main() {
	#[main];
    sdl2::init([sdl2::InitVideo]);
    match render::Renderer::new_with_window(800, 600, []) {
        Ok(renderer) => {
            let mut limiter = ratelimiter::RateLimiter::new(60);
            let asset = @*match GraphicAsset::new(renderer, "item110.bmp") {
                Ok(texture) => texture,
                Err(msg) => {
                    fail!(msg)
                }
            };

            let mut ship = Ship::new(asset, 400, 300, 0);

            'main : loop {
                'event: loop {
                    match event::poll_event() {
                        event::QuitEvent(_) => break 'main,
                        event::KeyDownEvent(_, _, key, _, _) => {
                            match key {
                                keycode::EscapeKey => {
                                    break 'main
                                },
                                keycode::LeftKey => {
                                    ship.left()
                                },
                                keycode::RightKey => {
                                    ship.right()
                                }
                                _ => {
                                }
                            }
                        },
                        event::NoEvent  => {
                            break 'event
                        },
                        _ => {
                        }
                    };
                };
                renderer.clear();
                ship.draw(renderer);
                renderer.present();
                limiter.limit();
            }
        },
        Err(msg) => io::println(msg)
    };
    sdl2::quit();
}

struct Ship {
    asset: @GraphicAsset,
    x: int,
    y: int,
    rot: int
}

impl Ship {
    fn new(asset: @GraphicAsset, x: int, y: int, rot: int) -> ~Ship {
        ~Ship { asset: asset, x: x, y: y, rot: rot}
    }

    fn draw(&self, renderer: &render::Renderer) {
        self.asset.draw(renderer, rect::Point::new(self.x as i32, self.y as i32), (360.0 / 32.0) * self.rot as f64)
    }

    fn left(&mut self) {
        self.rot = (self.rot - 1) % 32;
    }

    fn right(&mut self) {
        self.rot = (self.rot + 1) % 32;
    }
}

struct GraphicAsset {
    texture: ~render::Texture,
    width: int,
    height: int
}

impl GraphicAsset {

    fn new(renderer: &render::Renderer, filename: &str) -> Result<~GraphicAsset, ~str> {
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
                    width: width,
                    height: height
                })
            })
        })
    }

    fn draw(&self, renderer: &render::Renderer, center: rect::Point, rotation: f64) {
        renderer.copy_ex(
            self.texture,
            None,
            Some(rect::Rect {x: center.x - (self.width/2) as i32, y: center.y - (self.height/2) as i32, w: self.width as i32, h: self.height as i32}),
            rotation,
            None,
            render::FlipNone
        );
    }
}
