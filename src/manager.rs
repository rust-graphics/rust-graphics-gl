#[cfg(feature = "verbose_log")]
use crate::window::log::log_i;
use {
    super::{context::Context, loader::Loader},
    crate::window::Window,
    std::sync::Arc,
};

pub struct Manager {
    window: Arc<Window>,
    context: Arc<Context>,
    loader: Arc<Loader>,
}

impl Manager {
    pub fn new(window: Arc<Window>) -> Option<Self> {
        #[cfg(feature = "verbose_log")]
        log_i!("Start of OpenGL manager.");
        let context = if let Some(context) = Context::new(window.clone()) {
            Arc::new(context)
        } else {
            return None;
        };
        #[cfg(feature = "verbose_log")]
        log_i!("OpenGL context created.");
        let loader = if let Some(loader) = Loader::new(context.clone()) {
            Arc::new(loader)
        } else {
            return None;
        };
        #[cfg(feature = "verbose_log")]
        log_i!("OpenGL library loaded.");
        Some(Self {
            window,
            context,
            loader,
        })
    }

    pub fn get_loader(&self) -> &Loader {
        self.loader.as_ref()
    }

    pub fn swap_buffers(&self) {
        self.context.swap();
    }
}
