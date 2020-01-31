#[cfg(target_os = "android")]
use super::egl;
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HDC, HGLRC};
use {crate::window::Window, std::sync::Arc};

pub(crate) struct Context {
    window: Arc<Window>,
    #[cfg(target_os = "windows")]
    device: HDC,
    #[cfg(target_os = "windows")]
    render: HGLRC,
    #[cfg(target_os = "android")]
    egl_lib: egl::Egl,
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
        use std::ptr::null_mut;

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
        for c in EGL_CONFIGS {
            // if (check_surface(c[0], c[1], c[2])) {
            //     surface = eglCreateWindowSurface(display, config, window, nullptr);
            //     eglQuerySurface(display, surface, EGL_WIDTH, &screen_width);
            //     eglQuerySurface(display, surface, EGL_HEIGHT, &screen_height);
            //     GXLOGD("Surface with OpenGL: " << c[0] << ", depth: " << c[1] << ", samples: " << c[2])
            //     return;
            // }
        }
        None
        // GXLOGF("No suitable surface found.")

        //     init_egl_context();
        //     init_gles();
        //     egl_context_initialized = true;
    }

    // void gearoenix::system::GlContext::init_gles() noexcept
    // {
    //     if (gles_initialized)
    //         return;
    //     gles_initialized = true;
    //     if (gl::Loader::load_library(render::engine::Type::OPENGL_ES3)) {
    //         GXLOGD("OpenGL ES3 library loaded.")
    //         es3_supported = true;
    //         return;
    //     }
    //     if (gl::Loader::load_library(render::engine::Type::OPENGL_ES2)) {
    //         GXLOGD("OpenGL ES2 library loaded.")
    //         return;
    //     }
    //     GXLOGF("No suitable OpenGL library found")
    // }
    // void gearoenix::system::GlContext::terminate() noexcept
    // {
    //     if (display != EGL_NO_DISPLAY) {
    //         eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
    //         if (context != EGL_NO_CONTEXT) {
    //             eglDestroyContext(display, context);
    //         }
    //         if (surface != EGL_NO_SURFACE) {
    //             eglDestroySurface(display, surface);
    //         }
    //         eglTerminate(display);
    //     }
    //     display = EGL_NO_DISPLAY;
    //     context = EGL_NO_CONTEXT;
    //     surface = EGL_NO_SURFACE;
    //     context_valid = false;
    // }
    // bool gearoenix::system::GlContext::check_surface(const EGLint opengl_version, const EGLint depth_size, const EGLint samples_size) noexcept
    // {
    //     const EGLint attribs[] = {
    //         EGL_RENDERABLE_TYPE, opengl_version,
    //         EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
    //         EGL_BLUE_SIZE, 8,
    //         EGL_GREEN_SIZE, 8,
    //         EGL_RED_SIZE, 8,
    //         EGL_DEPTH_SIZE, depth_size,
    //         EGL_SAMPLE_BUFFERS, samples_size == 0 ? 0 : 1,
    //         EGL_SAMPLES, samples_size,
    //         EGL_NONE
    //     };
    //     this->depth_size = static_cast<int>(depth_size);
    //     this->samples_size = static_cast<int>(samples_size);
    //     EGLint num_configs;
    //     return 0 != eglChooseConfig(display, attribs, &config, 1, &num_configs);
    // }
    // void gearoenix::system::GlContext::init_egl_context() noexcept
    // {
    //     context_valid = true;
    //     {
    //         GXLOGD("Trying to create OpenGL context 3")
    //         const EGLint context_attribs[] = { EGL_CONTEXT_CLIENT_VERSION, 3, EGL_NONE };
    //         context = eglCreateContext(display, config, nullptr, context_attribs);
    //         if (eglMakeCurrent(display, surface, surface, context) != EGL_FALSE)
    //             return;
    //     }
    //     {
    //         GXLOGD("Trying to create OpenGL context 2")
    //         const EGLint context_attribs[] = { EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE };
    //         context = eglCreateContext(display, config, nullptr, context_attribs);
    //         if (eglMakeCurrent(display, surface, surface, context) != EGL_FALSE)
    //             return;
    //     }
    //     GXLOGF("Can not create the required context")
    // }
    // gearoenix::system::GlContext::~GlContext() noexcept
    // {
    //     terminate();
    // }
    // gearoenix::system::GlContext::State gearoenix::system::GlContext::swap() noexcept
    // {
    //     const EGLBoolean b = eglSwapBuffers(display, surface);
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
    //     }
    //     return State::RUNNING;
    // }
    // void gearoenix::system::GlContext::invalidate() noexcept
    // {
    //     terminate();
    //     egl_context_initialized = false;
    // }
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
}

impl Drop for Context {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        {
            use winapi::shared::minwindef::FALSE;
            use winapi::um::wingdi::wglDeleteContext;
            if FALSE == unsafe { wglDeleteContext(self.render) } {
                vxlogf!("Failed to destroy render context.");
            }
        }
    }
}
