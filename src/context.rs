#[cfg(feature = "verbose-log")]
use log::log_i;
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HDC, HGLRC};
#[cfg(target_os = "android")]
use {
    super::egl,
    log::log_f,
    std::{
        ffi::CString,
        mem::transmute_copy,
        ptr::{null, null_mut},
    },
};
use {crate::window::Window, std::sync::Arc};

pub(crate) struct Context {
    window: Arc<Window>,
    #[cfg(target_os = "windows")]
    device: HDC,
    #[cfg(target_os = "windows")]
    render: HGLRC,
    #[cfg(target_os = "android")]
    egl_lib: egl::Egl,
    #[cfg(target_os = "android")]
    display: egl::EGLDisplay,
    #[cfg(target_os = "android")]
    config: egl::EGLConfig,
    #[cfg(target_os = "android")]
    surface: egl::EGLSurface,
    #[cfg(target_os = "android")]
    context: egl::EGLContext,
}

impl Context {
    #[cfg(target_os = "windows")]
    pub(crate) fn new(window: Arc<Window>) -> Option<Self> {
        use std::mem::{size_of, zeroed};
        use std::ptr::null_mut;
        use winapi::shared::minwindef::{FALSE, WORD};
        use winapi::um::wingdi::{
            wglCreateContext, wglMakeCurrent, ChoosePixelFormat, SetPixelFormat, PFD_DOUBLEBUFFER,
            PFD_DRAW_TO_BITMAP, PFD_DRAW_TO_WINDOW, PFD_GENERIC_ACCELERATED, PFD_SUPPORT_OPENGL,
            PFD_SWAP_LAYER_BUFFERS, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
        };
        use winapi::um::winuser::GetDC;
        let mut desc: PIXELFORMATDESCRIPTOR = unsafe { zeroed() };
        desc.nSize = size_of::<PIXELFORMATDESCRIPTOR>() as WORD;
        desc.nVersion = 1;
        desc.dwFlags = PFD_DRAW_TO_WINDOW
            | PFD_DRAW_TO_BITMAP
            | PFD_SUPPORT_OPENGL
            | PFD_GENERIC_ACCELERATED
            | PFD_DOUBLEBUFFER
            | PFD_SWAP_LAYER_BUFFERS;
        desc.iPixelType = PFD_TYPE_RGBA;
        desc.cColorBits = 32;
        desc.cRedBits = 8;
        desc.cGreenBits = 8;
        desc.cBlueBits = 8;
        desc.cAlphaBits = 8;
        desc.cDepthBits = 32;
        desc.cStencilBits = 8;

        let device = unsafe { GetDC(sys_app.get_window()) };
        if device == null_mut() {
            vxlogi!("Device context fetching failed.");
            return None;
        }
        let pixel_format = unsafe { ChoosePixelFormat(device, &desc) };
        if pixel_format == 0 {
            vxlogi!("Pixel format index fetching failed.");
            return None;
        }
        if FALSE == unsafe { SetPixelFormat(device, pixel_format, &desc) } {
            vxlogi!("Pixel format creation failed.");
            return None;
        }
        vxlogi!("Pixel format created.");

        let render = unsafe { wglCreateContext(device) };
        if render == null_mut() {
            vxlogi!("Failed to create rendering context.");
            return None;
        }
        if FALSE == unsafe { wglMakeCurrent(device, render) } {
            vxlogi!("Failed to make rendering context current.");
            return None;
        }

        Some(Self { device, render })
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn swap(&self) {
        use winapi::shared::minwindef::FALSE;
        use winapi::um::wingdi::SwapBuffers;
        if FALSE == unsafe { SwapBuffers(self.device) } {
            vxlogf!("Context swap failed.");
        }
    }

    #[cfg(target_os = "windows")]
    fn get_function<T>(&self, s: &str) -> Option<T> {
        use winapi::um::wingdi::wglGetProcAddress;
        let cs = CString::new(s).unwrap();
        let f = unsafe { wglGetProcAddress(cs.as_ptr()) };
        if f.is_null() {
            return None;
        }
        Some(unsafe { transmute(f) })
    }

    #[cfg(target_os = "linux")]
    pub fn new(window: Arc<Window>) -> Option<Self> {
        Some(Self { window })
    }

    #[cfg(target_os = "linux")]
    pub fn swap(&self) {
        self.window.swap();
    }

    #[cfg(target_os = "linux")]
    pub fn get_function<T>(&self, s: &str) -> Option<T> {
        self.window.get_gl_function(s)
    }

    #[cfg(target_os = "android")]
    pub fn new(window: Arc<Window>) -> Option<Self> {
        use std::{
            mem::transmute_copy,
            ptr::{null, null_mut},
        };

        let egl_lib = if let Some(l) = egl::Egl::new() {
            l
        } else {
            return None;
        };
        let display = (egl_lib.get_display)(egl::DEFAULT_DISPLAY);
        if egl::TRUE != (egl_lib.initialize)(display, null_mut(), null_mut()) {
            return None;
        }
        const EGL_CONFIGS: [[egl::EGLint; 3]; 8] = [
            [egl::OPENGL_ES3_BIT, 32, 4],
            [egl::OPENGL_ES3_BIT, 32, 0],
            [egl::OPENGL_ES3_BIT, 24, 4],
            [egl::OPENGL_ES3_BIT, 24, 0],
            [egl::OPENGL_ES2_BIT, 32, 4],
            [egl::OPENGL_ES2_BIT, 32, 0],
            [egl::OPENGL_ES2_BIT, 24, 4],
            [egl::OPENGL_ES2_BIT, 24, 0],
        ];
        let mut config = null_mut();
        let mut surface = null_mut();
        for c in &EGL_CONFIGS {
            if {
                let attribs = [
                    egl::RENDERABLE_TYPE,
                    c[0],
                    egl::SURFACE_TYPE,
                    egl::WINDOW_BIT,
                    egl::BLUE_SIZE,
                    8,
                    egl::GREEN_SIZE,
                    8,
                    egl::RED_SIZE,
                    8,
                    egl::DEPTH_SIZE,
                    c[1],
                    egl::SAMPLE_BUFFERS,
                    if c[2] == 0 { 0 } else { 1 },
                    egl::SAMPLES,
                    c[2],
                    egl::NONE,
                ];
                let mut num_configs = 0;
                egl::TRUE
                    == (egl_lib.choose_config)(
                        display,
                        attribs.as_ptr(),
                        &mut config,
                        1,
                        &mut num_configs,
                    )
                    && num_configs > 0
                    && !config.is_null()
            } {
                surface = (egl_lib.create_window_surface)(
                    display,
                    config,
                    unsafe { transmute_copy(&window.get_window()) },
                    null(),
                );
                #[cfg(feature = "verbose-log")]
                log_i!(
                    "Surface with OpenGL: {}, depth: {}, samples: {}",
                    c[0],
                    c[1],
                    c[2]
                );
                break;
            }
        }
        if surface.is_null() {
            #[cfg(feature = "verbose-log")]
            log_i!("Can not create EGL Surface.");
            return None;
        }
        const CONTEXT_ATTRIBS: [[egl::EGLint; 5]; 4] = [
            [
                egl::CONTEXT_MAJOR_VERSION,
                3,
                egl::CONTEXT_MINOR_VERSION,
                2,
                egl::NONE,
            ],
            [
                egl::CONTEXT_MAJOR_VERSION,
                3,
                egl::CONTEXT_MINOR_VERSION,
                1,
                egl::NONE,
            ],
            [
                egl::CONTEXT_MAJOR_VERSION,
                3,
                egl::CONTEXT_MINOR_VERSION,
                0,
                egl::NONE,
            ],
            [
                egl::CONTEXT_MAJOR_VERSION,
                2,
                egl::CONTEXT_MINOR_VERSION,
                0,
                egl::NONE,
            ],
        ];

        let mut context = null_mut();

        for attribs in &CONTEXT_ATTRIBS {
            context = (egl_lib.create_context)(display, config, null_mut(), attribs.as_ptr());
            if context.is_null() {
                continue;
            }
            if egl::TRUE == (egl_lib.make_current)(display, surface, surface, context) {
                #[cfg(feature = "verbose-log")]
                log_i!(
                    "EGL context with OpenGL ES {}.{} created",
                    attribs[1],
                    attribs[3]
                );
                break;
            } else {
                context = null_mut();
            }
        }

        if context.is_null() {
            #[cfg(feature = "verbose-log")]
            log_i!("Can not create EGL Context.");
            return None;
        }

        Some(Self {
            window,
            egl_lib,
            display,
            config,
            surface,
            context,
        })
    }

    // void gearoenix::system::GlContext::suspend() noexcept
    // {
    //     if (surface != EGL_NO_SURFACE) {
    //         eglDestroySurface(display, surface);
    //         surface = EGL_NO_SURFACE;
    //     }
    // }
    // void gearoenix::system::GlContext::resume(ANativeWindow* const window) noexcept
    // {
    //     if (!egl_context_initialized) {
    //         init(window);
    //         return;
    //     }
    //     const int original_widhth = screen_width;
    //     const int original_height = screen_height;
    //     this->window = window;
    //     surface = eglCreateWindowSurface(display, config, window, nullptr);
    //     eglQuerySurface(display, surface, EGL_WIDTH, &screen_width);
    //     eglQuerySurface(display, surface, EGL_HEIGHT, &screen_height);
    //     if (screen_width != original_widhth || screen_height != original_height)
    //         GXLOGD("Screen resized");
    //     if (eglMakeCurrent(display, surface, surface, context) == EGL_TRUE)
    //         return;
    //     const EGLint err = eglGetError();
    //     GXLOGD("Unable to eglMakeCurrent " << err);
    //     if (err == EGL_CONTEXT_LOST) {
    //         GXLOGD("Re-creating egl context");
    //         init_egl_context();
    //     } else {
    //         terminate();
    //         init_egl_surface();
    //         init_egl_context();
    //     }
    // }

    #[cfg(target_os = "android")]
    pub fn swap(&self) {
        if egl::TRUE != (self.egl_lib.swap_buffers)(self.display, self.surface) {
            log_f!("EGL context is not valid any more. There is a bug somewhere that does not initialize the context in the correct way.");
            //     if (b == 0) {
            //         const EGLint err = eglGetError();
            //         if (err == EGL_BAD_SURFACE) {
            //             init_egl_surface();
            //             return State::RUNNING;
            //         } else if (err == EGL_CONTEXT_LOST || err == EGL_BAD_CONTEXT) {
            //             terminate();
            //             return State::TERMINATED;
            //         }
            //         GXLOGE("Unhandled error " << err)
        }
    }

    #[cfg(target_os = "android")]
    pub fn get_function<T>(&self, s: &str) -> Option<T> {
        let cs = CString::new(s).unwrap();
        if let Some(f) = (self.egl_lib.get_proc_address)(cs.as_ptr()) {
            Some(unsafe { transmute_copy(&f) })
        } else {
            None
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        {
            use winapi::shared::minwindef::FALSE;
            use winapi::um::wingdi::wglDeleteContext;
            if FALSE == unsafe { wglDeleteContext(self.render) } {
                log_f!("Failed to destroy render context.");
            }
        }
        #[cfg(target_os = "android")]
        {
            if !self.display.is_null() {
                (self.egl_lib.make_current)(self.display, null_mut(), null_mut(), null_mut());
                if !self.context.is_null() {
                    if egl::TRUE != (self.egl_lib.destroy_context)(self.display, self.context) {
                        log_f!("Failed to terminate EGL context.");
                    }
                }
                if !self.surface.is_null() {
                    if egl::TRUE != (self.egl_lib.destroy_surface)(self.display, self.surface) {
                        log_f!("Failed to terminate EGL surface.");
                    }
                }
                if egl::TRUE != (self.egl_lib.terminate)(self.display) {
                    log_f!("Failed to terminate EGL.");
                }
            }
        }
    }
}
