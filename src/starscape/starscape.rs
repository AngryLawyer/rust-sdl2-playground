extern mod sdl2;
use std::rand;
use std::rand::RngUtil;
use sdl2::render;
use sdl2::event;
use sdl2::pixels;
use sdl2::rect;
use sdl2::keycode;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
	#[main];
    let mut stars = generate_star_layer();
    sdl2::init([sdl2::InitVideo]);
    match render::Renderer::new_with_window(800, 600, []) {
        Ok(renderer) => {
            let mut limiter = RateLimiter::new(60);

            'main : loop {
                'event: loop {
                    match event::poll_event() {
                        event::QuitEvent(_) => break 'main,
                        event::KeyDownEvent(_, _, key, _, _) => {
                            if key == keycode::EscapeKey {
                                break 'main
                            }
                        },
                        event::NoEvent  => {
                            break 'event
                        },
                        _ => {
                        }
                    };
                };
                do_think(stars);
                renderer.set_draw_color(pixels::RGB(0, 0, 0));
                renderer.clear();
                render_stars(renderer, stars);
                renderer.present();
                limiter.limit();
            }
        },
        Err(msg) => fail!(msg)
    };
    sdl2::quit();
}

fn render_stars(renderer: &render::Renderer, stars: &[Star]) {
    for star in stars.iter() {
        let base_hue = (255 / 5) / star.layer;
        let color = if star.frame > 4 {
            base_hue - (star.frame * 50)
        } else {
            base_hue - ((8 - star.frame) * 50)
        };
        renderer.set_draw_color(pixels::RGB(color, color, color));
        renderer.draw_point(star.pos);
    }
}

fn do_think(stars: &mut [Star]) {
    for star in stars.mut_iter() {
        star.think()
    }
}

fn generate_star_layer() -> ~[Star] {
    let mut rng = rand::rng();
    let mut stars: ~[Star] = ~[];

    for _i in range(0, 100) {
        let x: int = rng.gen_uint_range(0, 800) as int;
        let y: int = rng.gen_uint_range(0, 600) as int;
        let frame: u8 = rng.gen_uint_range(0, 7) as u8;
        let layer: u8 = rng.gen_uint_range(1, 5) as u8;
        stars.push(Star::new(x, y, frame, layer));
    }
    stars
}

struct Star {
    pos: rect::Point,
    frame: u8,
    layer: u8
}

impl Star {

    fn new(x: int, y: int, initial_frame: u8, layer: u8) -> Star {
        Star {
            pos: rect::Point {
                x: x as i32,
                y: y as i32
            },
            frame: initial_frame,
            layer: layer
        }
    }

    fn think(&mut self) {
        if self.frame == 7 {
            self.frame = 0
        } else {
            self.frame += 1
        };
        self.pos.y += self.layer as i32;
        if self.pos.y > 600 {
            self.pos.y -= 600
        }
    }
}

struct RateLimiter {
    fps: uint,
    last_ticks: uint
}

impl RateLimiter {

    fn new(fps: uint) -> RateLimiter {
        RateLimiter {
            fps: fps,
            last_ticks: 0
        }
    }

    fn limit(&mut self) {
        let ticks = sdl2::timer::get_ticks();
        let adjusted_ticks = ticks - self.last_ticks;
        if adjusted_ticks < 1000 / self.fps {
            sdl2::timer::delay((1000 / self.fps) - adjusted_ticks);
        }
        self.last_ticks = ticks;
    }
}
