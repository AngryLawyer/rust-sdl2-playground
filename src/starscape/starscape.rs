extern mod sdl2;
use std::io;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
	#[main];

    sdl2::init([sdl2::InitVideo]);
    match sdl2::render::Renderer::new_with_window(800, 600, []) {
        Ok(renderer) => {
            renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
            renderer.clear();
            renderer.present();

            'main : loop {
                'event : loop {
                    match sdl2::event::wait_event() {
                        sdl2::event::QuitEvent(_) => break 'main,
                        sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                            io::println("LOL");
                            if key == sdl2::keycode::EscapeKey {
                                break 'main
                            }
                        }
                        _ => {
                        }
                    };
                }
            }
        },
        Err(msg) => fail!(msg)
    };
    sdl2::quit();
}
