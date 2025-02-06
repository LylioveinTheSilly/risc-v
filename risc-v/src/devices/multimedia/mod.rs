pub mod display;
pub use display::Display;

pub use sdl2::{Sdl, VideoSubsystem};

#[allow(unused)]
pub struct SdlDevice {
    context: Sdl,
    video_subsystem: VideoSubsystem,
}
impl SdlDevice {
    pub fn new() -> Result<Self, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;

        Ok(Self { context, video_subsystem })
    }
    pub fn display(&self) -> Result<Display, String> {
        let window = self.video_subsystem.window("Fantasy Console", 800, 600)
            .borderless()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Display::new(canvas))
    }
}

