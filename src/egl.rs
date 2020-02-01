use {
    library_loader::Linker,
    log::unwrap_f,
    std::os::raw::{c_char, c_uint, c_ulong, c_void},
};

pub type EGLint = i32;
pub type EGLBoolean = c_uint;

pub type EGLNativeDisplayType = *mut c_void;
pub type EGLNativeWindowType = c_ulong;
pub type EGLDisplay = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLSurface = *mut c_void;
pub type EGLContext = *mut c_void;

pub const TRUE: EGLBoolean = 1;
pub const FALSE: EGLBoolean = 0;
pub const DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as EGLNativeDisplayType;
pub const OPENGL_ES2_BIT: EGLint = 4;
pub const OPENGL_ES3_BIT: EGLint = 64;
pub const RENDERABLE_TYPE: EGLint = 12352;
pub const SURFACE_TYPE: EGLint = 12339;
pub const WINDOW_BIT: EGLint = 4;
pub const RED_SIZE: EGLint = 12324;
pub const GREEN_SIZE: EGLint = 12323;
pub const BLUE_SIZE: EGLint = 12322;
pub const ALPHA_SIZE: EGLint = 12321;
pub const DEPTH_SIZE: EGLint = 12325;
pub const STENCIL_SIZE: EGLint = 12326;
pub const SAMPLE_BUFFERS: EGLint = 12338;
pub const SAMPLES: EGLint = 12337;
pub const CONTEXT_MAJOR_VERSION: EGLint = 12440;
pub const CONTEXT_MINOR_VERSION: EGLint = 12539;
pub const NONE: EGLint = 12344;

pub struct Egl {
    pub get_display: extern "C" fn(display_id: EGLNativeDisplayType) -> EGLDisplay,
    pub initialize:
        extern "C" fn(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean,
    pub choose_config: extern "C" fn(
        dpy: EGLDisplay,
        attrib_list: *const EGLint,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EGLBoolean,
    pub create_window_surface: extern "C" fn(
        dpy: EGLDisplay,
        config: EGLConfig,
        win: EGLNativeWindowType,
        attrib_list: *const EGLint,
    ) -> EGLSurface,
    pub create_context: extern "C" fn(
        dpy: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext,
    pub make_current: extern "C" fn(
        dpy: EGLDisplay,
        draw: EGLSurface,
        read: EGLSurface,
        ctx: EGLContext,
    ) -> EGLBoolean,
    pub destroy_context: extern "C" fn(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean,
    pub destroy_surface: extern "C" fn(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean,
    pub terminate: extern "C" fn(dpy: EGLDisplay) -> EGLBoolean,
    pub get_proc_address: extern "C" fn(procname: *const c_char) -> Option<extern "C" fn()>,
    pub swap_buffers: extern "C" fn(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean,
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
        Some(Self {
            get_display: fun!(GetDisplay),
            initialize: fun!(Initialize),
            choose_config: fun!(ChooseConfig),
            create_window_surface: fun!(CreateWindowSurface),
            create_context: fun!(CreateContext),
            make_current: fun!(MakeCurrent),
            destroy_context: fun!(DestroyContext),
            destroy_surface: fun!(DestroySurface),
            terminate: fun!(Terminate),
            get_proc_address: fun!(GetProcAddress),
            swap_buffers: fun!(SwapBuffers),
            _lib,
        })
    }
}
