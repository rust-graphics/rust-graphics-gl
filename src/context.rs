#[cfg(target_os = "windows")]
use winapi::shared::windef::{HDC, HGLRC};
use {crate::window::Window, std::sync::Arc};

pub(crate) struct Context {
    window: Arc<Window>,
    #[cfg(target_os = "windows")]
    device: HDC,
    #[cfg(target_os = "windows")]
    render: HGLRC,
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
