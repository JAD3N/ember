use crate::Scene;
use std::sync::Arc;

pub struct Ember {
    pub window: Arc<winit::window::Window>,
    pub scene: Scene,
}

pub struct Monitor {
    pub handle: winit::monitor::MonitorHandle,
}

impl Monitor {
    pub fn video_modes(&self) -> Vec<VideoMode> {
        let mut video_modes = vec![];

        for video_mode in self.handle.video_modes() {
            video_modes.push(video_mode.into());
        }

        video_modes
    }
}

impl Into<Monitor> for winit::monitor::MonitorHandle {
    fn into(self) -> Monitor {
        Monitor { handle: self }
    }
}

pub struct VideoMode {
    pub handle: winit::monitor::VideoMode,
}

impl Into<VideoMode> for winit::monitor::VideoMode {
    fn into(self) -> VideoMode {
        VideoMode { handle: self }
    }
}

pub enum Fullscreen {
    Borderless(Monitor),
    Exclusive(VideoMode),
}

impl Into<Fullscreen> for winit::window::Fullscreen {
    fn into(self) -> Fullscreen {
        match self {
            winit::window::Fullscreen::Borderless(monitor) => Fullscreen::Borderless(monitor.into()),
            winit::window::Fullscreen::Exclusive(video_mode) => Fullscreen::Exclusive(video_mode.into()),
        }
    }
}

impl Ember {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        Self { window, scene: Scene }
    }

    pub fn monitors(&self) -> Vec<Monitor> {
        let mut monitors = vec![];

        for monitor in self.window.available_monitors() {
            monitors.push(monitor.into())
        }

        monitors
    }

    pub fn primary_monitor(&self) -> Monitor {
        self.window.primary_monitor().into()
    }

    pub fn set_fullscreen<T: Into<Fullscreen>>(&self, fullscreen: Option<T>) {
        match fullscreen {
            Some(fullscreen) => match fullscreen.into() {
                Fullscreen::Borderless(monitor) => self.window.set_fullscreen(
                    Some(winit::window::Fullscreen::Borderless(monitor.handle)),
                ),
                Fullscreen::Exclusive(video_mode) => self.window.set_fullscreen(
                    Some(winit::window::Fullscreen::Exclusive(video_mode.handle)),
                ),
            },
            None => self.window.set_fullscreen(None),
        }
    }

    pub fn fullscreen(&self) -> Option<Fullscreen> {
        match self.window.fullscreen() {
            Some(fullscreen) => Some(fullscreen.into()),
            None => None,
        }
    }

    pub fn set_title<S: Into<String>>(&self, s: S) {
        self.window.set_title(&s.into());
    }
}