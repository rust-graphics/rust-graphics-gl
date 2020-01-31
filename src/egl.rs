use {
    library_loader::Linker,
    log::unwrap_f,
    std::os::raw::{c_uint, c_void},
};

pub type EGLint = i32;
pub type EGLBoolean = c_uint;
pub type EGLNativeDisplayType = *mut c_void;
pub type EGLDisplay = *mut c_void;

pub const TRUE: EGLBoolean = 1;
pub const FALSE: EGLBoolean = 0;
pub const DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as EGLNativeDisplayType;
pub const OPENGL_ES2_BIT: EGLint = 4;
pub const OPENGL_ES3_BIT: EGLint = 64;

pub struct Egl {
    pub get_display: extern "C" fn(display_id: EGLNativeDisplayType) -> EGLDisplay,
    pub initialize:
        extern "C" fn(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean,
    _lib: Linker,
}

impl Egl {
    pub fn new() -> Option<Self> {
        let _lib = if let Some(l) = Linker::new("libEGL.so") {
            l
        } else {
            return None;
        };
        macro_rules! fun {
            ($f:ident) => {
                if let Some(f) = _lib.get_function(concat!("egl", stringify!($f))) {
                    f
                } else {
                    return None;
                }
            };
        }
        Self {
            get_display: fun!(GetDisplay),
            initialize: fun!(eglInitialize),
            _lib,
        }
    }
}
