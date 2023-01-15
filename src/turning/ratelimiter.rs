use sdl2;

struct RateLimiter {
    fps: usize,
    last_ticks: usize
}

impl RateLimiter {

    pub fn new(fps: usize) -> RateLimiter {
        RateLimiter {
            fps: fps,
            last_ticks: 0
        }
    }

    pub fn limit(&mut self) {
        let ticks = sdl2::timer::get_ticks();
        let adjusted_ticks = ticks - self.last_ticks;
        if adjusted_ticks < 1000 / self.fps {
            sdl2::timer::delay((1000 / self.fps) - adjusted_ticks);
        }
        self.last_ticks = ticks;
    }
}
