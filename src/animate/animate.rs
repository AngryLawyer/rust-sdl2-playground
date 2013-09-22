extern mod sdl2;
extern mod extra;

use std::rand;
use std::rand::RngUtil;
use std::io;
use std::path;
use extra::treemap;
use sdl2::render;
use sdl2::event;
use sdl2::pixels;
use sdl2::rect;
use sdl2::keycode;
use sdl2::surface;

mod ratelimiter;
mod graphics;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}


fn main() {
	#[main];
    sdl2::init([sdl2::InitVideo]);
    match render::Renderer::new_with_window(800, 600, []) {
        Ok(renderer) => {
            let mut limiter = ratelimiter::RateLimiter::new(10);
            let mut asteroid_graphic = graphics::GraphicAsset::new_with_frames(renderer, "item11.bmp", graphics::generate_frames(43, 43, 18)).unwrap();
            let mut animatable = Animatable::new(@*asteroid_graphic, 18);

            'main : loop {
                'event: loop {
                    match event::poll_event() {
                        event::QuitEvent(_) => break 'main,
                        event::KeyDownEvent(_, _, key, _, _) => {
                            match key {
                                keycode::EscapeKey => {
                                    break 'main
                                },
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
                animatable.think();
                animatable.draw(renderer);
                renderer.present();
                limiter.limit();
            }
        },
        Err(msg) => io::println(msg)
    };
    sdl2::quit();
}

struct Animatable {
    frames: uint,
    frame: uint,
    asset: @graphics::GraphicAsset
}

impl Animatable {
    fn new(asset: @graphics::GraphicAsset, frames: uint) -> ~Animatable {
        ~Animatable{ asset: asset, frames: frames, frame: 0 }
    }

    fn draw(&self, renderer: &render::Renderer) {
        self.asset.draw(renderer, rect::Point::new(400, 300), 0.0, self.frame)
    }

    fn think(&mut self) {
        self.frame += 1;
        if self.frame >= self.frames {
            self.frame = 0;
        }
    }
}
