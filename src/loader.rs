use {
    super::{
        context::Context,
        types::{BitField, Boolean, Enumerated, SInt, SizeI, SizeIPtr, UInt},
    },
    crate::window::{library_loader::Linker, log::log_i},
    std::{
        os::raw::{c_char, c_void},
        sync::Arc,
    },
};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Loader {
    pub active_texture: extern "C" fn(texture: Enumerated),
    pub attach_shader: extern "C" fn(program: UInt, shader: UInt),
    pub bind_attrib_location: extern "C" fn(program: UInt, index: UInt, name: *const c_char),
    pub bind_buffer: extern "C" fn(target: Enumerated, buffer: UInt),
    pub bind_framebuffer: extern "C" fn(target: Enumerated, framebuffer: UInt),
    pub bind_renderbuffer: extern "C" fn(target: Enumerated, renderbuffer: UInt),
    pub bind_texture: extern "C" fn(target: Enumerated, texture: UInt),
    pub bind_vertex_array: extern "C" fn(arr: UInt),
    pub blend_func: extern "C" fn(s_factor: Enumerated, d_factor: Enumerated),
    pub buffer_data: extern "C" fn(
        target: Enumerated,
        data_size: SizeIPtr,
        data: *const c_void,
        usage: Enumerated,
    ),
    pub check_framebuffer_status: extern "C" fn(target: Enumerated) -> Enumerated,
    pub clear_color: extern "C" fn(red: f32, green: f32, blue: f32, alpha: f32),
    pub clear: extern "C" fn(mask: BitField),
    pub compile_shader: extern "C" fn(shader: UInt),
    pub create_program: extern "C" fn() -> UInt,
    pub create_shader: extern "C" fn(shader: Enumerated) -> UInt,
    pub cull_face: extern "C" fn(mode: Enumerated),
    pub delete_buffers: extern "C" fn(number: SizeI, shader: *const UInt),
    pub delete_framebuffers: extern "C" fn(number: SizeI, framebuffers: *const UInt),
    pub delete_program: extern "C" fn(program: UInt),
    pub delete_renderbuffers: extern "C" fn(number: SizeI, renderbuffers: *const UInt),
    pub delete_shader: extern "C" fn(shader: UInt),
    pub delete_textures: extern "C" fn(number: SizeI, textures: *const UInt),
    pub delete_vertex_arrays: extern "C" fn(number: SizeI, arrays: *const UInt),
    pub depth_mask: extern "C" fn(flag: Boolean),
    pub disable: extern "C" fn(cap: Enumerated),
    pub draw_elements: extern "C" fn(
        mode: Enumerated,
        count: SizeI,
        element_type: Enumerated,
        indices: *const c_void,
    ),
    pub enable: extern "C" fn(cap: Enumerated),
    pub enable_vertex_attrib_array: extern "C" fn(index: UInt),
    pub framebuffer_renderbuffer: extern "C" fn(
        target: Enumerated,
        attachment: Enumerated,
        renderbuffertarget: Enumerated,
        renderbuffer: UInt,
    ),
    pub framebuffer_texture2d: extern "C" fn(
        target: Enumerated,
        attachment: Enumerated,
        textarget: Enumerated,
        texture: UInt,
        level: SInt,
    ),
    pub gen_buffers: extern "C" fn(number: SizeI, buffers: *mut UInt),
    pub gen_framebuffers: extern "C" fn(number: SizeI, framebuffers: *mut UInt),
    pub gen_renderbuffers: extern "C" fn(number: SizeI, renderbuffers: *mut UInt),
    pub gen_textures: extern "C" fn(number: SizeI, textures: *mut UInt),
    pub generate_mipmap: extern "C" fn(target: Enumerated),
    pub get_attrib_location: extern "C" fn(program: UInt, name: *const c_char) -> SInt,
    pub get_error: extern "C" fn() -> Enumerated,
    pub get_integer_v: extern "C" fn(pname: Enumerated, data: *mut SInt),
    pub gen_vertex_arrays: extern "C" fn(number: SizeI, arrays: *mut UInt),
    pub get_program_iv: extern "C" fn(program: UInt, pnamne: Enumerated, params: *mut SInt),
    pub get_program_info_log:
        extern "C" fn(program: UInt, buf_size: SizeI, length: *mut SizeI, info: *mut c_char),
    pub get_shader_iv: extern "C" fn(shader: UInt, pname: Enumerated, params: *mut SInt),
    pub get_shader_info_log:
        extern "C" fn(shader: UInt, buf_size: SizeI, length: *mut SizeI, info: *mut c_char),
    pub get_uniform_location: extern "C" fn(program: UInt, name: *const c_char) -> SInt,
    pub link_program: extern "C" fn(program: UInt),
    pub read_buffer: extern "C" fn(src: Enumerated),
    pub renderbuffer_storage: extern "C" fn(
        target: Enumerated,
        internal_format: Enumerated,
        img_width: SizeI,
        img_height: SizeI,
    ),
    pub tex_image_2d: extern "C" fn(
        target: Enumerated,
        level: SInt,
        internal_format: SInt,
        image_width: SizeI,
        image_height: SizeI,
        border: SInt,
        format: Enumerated,
        data_type: Enumerated,
        pixels: *const c_void,
    ),
    pub tex_parameter_f: extern "C" fn(Enumerated, Enumerated, f32),
    pub tex_parameter_fv: extern "C" fn(Enumerated, Enumerated, *const f32),
    pub tex_parameter_i: extern "C" fn(Enumerated, Enumerated, SInt),
    pub tex_parameter_iv: extern "C" fn(Enumerated, Enumerated, *const SInt),
    pub scissor: extern "C" fn(SInt, SInt, SizeI, SizeI),
    pub shader_source:
        extern "C" fn(shader: UInt, count: SizeI, code: *const *const c_char, length: *const SInt),
    pub uniform_1f: extern "C" fn(location: SInt, v0: f32),
    pub uniform_1fv: extern "C" fn(location: SInt, count: SizeI, data: *const c_void),
    pub uniform_1i: extern "C" fn(location: SInt, v0: SInt),
    pub uniform_1iv: extern "C" fn(location: SInt, count: SizeI, data: *const c_void),
    pub uniform_2fv: extern "C" fn(location: SInt, count: SizeI, data: *const c_void),
    pub uniform_3fv: extern "C" fn(location: SInt, count: SizeI, data: *const c_void),
    pub uniform_4fv: extern "C" fn(location: SInt, count: SizeI, data: *const c_void),
    pub uniform_matrix_2fv:
        extern "C" fn(location: SInt, count: SizeI, transpose: Boolean, value: *const f32),
    pub uniform_matrix_3fv:
        extern "C" fn(location: SInt, count: SizeI, transpose: Boolean, value: *const f32),
    pub uniform_matrix_4fv:
        extern "C" fn(location: SInt, count: SizeI, transpose: Boolean, value: *const f32),
    pub use_program: extern "C" fn(program: UInt),
    pub validate_program: extern "C" fn(program: UInt),
    pub vertex_attrib_pointer: extern "C" fn(
        index: UInt,
        size: SInt,
        t: Enumerated,
        normalized: Boolean,
        stride: SizeI,
        pointer: *const c_void,
    ),
    pub viewport: extern "C" fn(SInt, SInt, SizeI, SizeI),
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    _library: Linker,
    context: Arc<Context>,
}

impl Loader {
    pub(crate) fn new(context: Arc<Context>) -> Option<Self> {
        #[cfg(feature = "verbose-log")]
        log_i!("Going to load OpenGL library.");
        #[cfg(target_os = "windows")]
        let library_name = "opengl32.dll";
        #[cfg(target_os = "linux")]
        let library_name = "libGL.so";
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        let _library = if let Some(l) = Linker::new(library_name) {
            l
        } else {
            log_i!("Can not load OpenGL library, {} not found.", library_name);
            return None;
        };

        #[cfg(any(target_os = "windows", target_os = "linux"))]
        macro_rules! fun {
            ($n:expr) => {
                if let Some(f) = context.get_function(concat!("gl", $n)) {
                    f
                } else if let Some(f) = _library.get_function(concat!("gl", $n)) {
                    f
                } else {
                    log_i!("Can not load function 'gl{}'", $n);
                    return None;
                }
            };
        }

        #[cfg(target_os = "android")]
        macro_rules! fun {
            ($n:expr) => {
                if let Some(f) = context.get_function(concat!("gl", $n)) {
                    f
                } else {
                    log_i!("Can not load function 'gl{}'", $n);
                    return None;
                }
            };
        }

        Some(Self {
            active_texture: fun!("ActiveTexture"),
            attach_shader: fun!("AttachShader"),
            bind_attrib_location: fun!("BindAttribLocation"),
            bind_buffer: fun!("BindBuffer"),
            bind_framebuffer: fun!("BindFramebuffer"),
            bind_renderbuffer: fun!("BindRenderbuffer"),
            bind_texture: fun!("BindTexture"),
            bind_vertex_array: fun!("BindVertexArray"),
            blend_func: fun!("BlendFunc"),
            buffer_data: fun!("BufferData"),
            check_framebuffer_status: fun!("CheckFramebufferStatus"),
            clear_color: fun!("ClearColor"),
            clear: fun!("Clear"),
            compile_shader: fun!("CompileShader"),
            create_program: fun!("CreateProgram"),
            create_shader: fun!("CreateShader"),
            cull_face: fun!("CullFace"),
            delete_buffers: fun!("DeleteBuffers"),
            delete_framebuffers: fun!("DeleteFramebuffers"),
            delete_program: fun!("DeleteProgram"),
            delete_renderbuffers: fun!("DeleteRenderbuffers"),
            delete_shader: fun!("DeleteShader"),
            delete_textures: fun!("DeleteTextures"),
            delete_vertex_arrays: fun!("DeleteVertexArrays"),
            depth_mask: fun!("DepthMask"),
            disable: fun!("Disable"),
            draw_elements: fun!("DrawElements"),
            enable: fun!("Enable"),
            enable_vertex_attrib_array: fun!("EnableVertexAttribArray"),
            framebuffer_renderbuffer: fun!("FramebufferRenderbuffer"),
            framebuffer_texture2d: fun!("FramebufferTexture2D"),
            gen_buffers: fun!("GenBuffers"),
            gen_framebuffers: fun!("GenFramebuffers"),
            gen_renderbuffers: fun!("GenRenderbuffers"),
            gen_textures: fun!("GenTextures"),
            generate_mipmap: fun!("GenerateMipmap"),
            get_attrib_location: fun!("GetAttribLocation"),
            get_error: fun!("GetError"),
            get_integer_v: fun!("GetIntegerv"),
            gen_vertex_arrays: fun!("GenVertexArrays"),
            get_program_iv: fun!("GetProgramiv"),
            get_program_info_log: fun!("GetProgramInfoLog"),
            get_shader_iv: fun!("GetShaderiv"),
            get_shader_info_log: fun!("GetShaderInfoLog"),
            get_uniform_location: fun!("GetUniformLocation"),
            link_program: fun!("LinkProgram"),
            read_buffer: fun!("ReadBuffer"),
            renderbuffer_storage: fun!("RenderbufferStorage"),
            tex_image_2d: fun!("TexImage2D"),
            tex_parameter_f: fun!("TexParameterf"),
            tex_parameter_fv: fun!("TexParameterfv"),
            tex_parameter_i: fun!("TexParameteri"),
            tex_parameter_iv: fun!("TexParameteriv"),
            scissor: fun!("Scissor"),
            shader_source: fun!("ShaderSource"),
            uniform_1f: fun!("Uniform1f"),
            uniform_1fv: fun!("Uniform1fv"),
            uniform_1i: fun!("Uniform1i"),
            uniform_1iv: fun!("Uniform1iv"),
            uniform_2fv: fun!("Uniform2fv"),
            uniform_3fv: fun!("Uniform3fv"),
            uniform_4fv: fun!("Uniform4fv"),
            uniform_matrix_2fv: fun!("UniformMatrix2fv"),
            uniform_matrix_3fv: fun!("UniformMatrix3fv"),
            uniform_matrix_4fv: fun!("UniformMatrix4fv"),
            use_program: fun!("UseProgram"),
            validate_program: fun!("ValidateProgram"),
            vertex_attrib_pointer: fun!("VertexAttribPointer"),
            viewport: fun!("Viewport"),
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            _library,
            context,
        })
    }
}
