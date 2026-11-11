use std::{
    ffi::{CStr, c_char, c_void},
    mem,
};

// 1.0
pub type GLvoid = c_void;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLsizei = i32;
pub type GLbitfield = u32;
pub type GLdouble = f64;
pub type GLuint = u32;
pub type GLboolean = u8;
pub type GLubyte = u8;
pub const DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
pub const STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;
pub const COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
pub const FALSE: GLboolean = 0;
pub const TRUE: GLboolean = 1;
pub const POINTS: GLenum = 0x0000;
pub const LINES: GLenum = 0x0001;
pub const LINE_LOOP: GLenum = 0x0002;
pub const LINE_STRIP: GLenum = 0x0003;
pub const TRIANGLES: GLenum = 0x0004;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const TRIANGLE_FAN: GLenum = 0x0006;
pub const QUADS: GLenum = 0x0007;
pub const NEVER: GLenum = 0x0200;
pub const LESS: GLenum = 0x0201;
pub const EQUAL: GLenum = 0x0202;
pub const LEQUAL: GLenum = 0x0203;
pub const GREATER: GLenum = 0x0204;
pub const NOTEQUAL: GLenum = 0x0205;
pub const GEQUAL: GLenum = 0x0206;
pub const ALWAYS: GLenum = 0x0207;
pub const ZERO: GLenum = 0;
pub const ONE: GLenum = 1;
pub const SRC_COLOR: GLenum = 0x0300;
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const DST_ALPHA: GLenum = 0x0304;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const DST_COLOR: GLenum = 0x0306;
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const NONE: GLenum = 0;
pub const FRONT_LEFT: GLenum = 0x0400;
pub const FRONT_RIGHT: GLenum = 0x0401;
pub const BACK_LEFT: GLenum = 0x0402;
pub const BACK_RIGHT: GLenum = 0x0403;
pub const FRONT: GLenum = 0x0404;
pub const BACK: GLenum = 0x0405;
pub const LEFT: GLenum = 0x0406;
pub const RIGHT: GLenum = 0x0407;
pub const FRONT_AND_BACK: GLenum = 0x0408;
pub const NO_ERROR: GLenum = 0;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
pub const CW: GLenum = 0x0900;
pub const CCW: GLenum = 0x0901;
pub const POINT_SIZE: GLenum = 0x0B11;
pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const LINE_SMOOTH: GLenum = 0x0B20;
pub const LINE_WIDTH: GLenum = 0x0B21;
pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const POLYGON_MODE: GLenum = 0x0B40;
pub const POLYGON_SMOOTH: GLenum = 0x0B41;
pub const CULL_FACE: GLenum = 0x0B44;
pub const CULL_FACE_MODE: GLenum = 0x0B45;
pub const FRONT_FACE: GLenum = 0x0B46;
pub const DEPTH_RANGE: GLenum = 0x0B70;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const DEPTH_FUNC: GLenum = 0x0B74;
pub const STENCIL_TEST: GLenum = 0x0B90;
pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const STENCIL_FUNC: GLenum = 0x0B92;
pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const STENCIL_FAIL: GLenum = 0x0B94;
pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const STENCIL_REF: GLenum = 0x0B97;
pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const VIEWPORT: GLenum = 0x0BA2;
pub const DITHER: GLenum = 0x0BD0;
pub const BLEND_DST: GLenum = 0x0BE0;
pub const BLEND_SRC: GLenum = 0x0BE1;
pub const BLEND: GLenum = 0x0BE2;
pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
pub const DRAW_BUFFER: GLenum = 0x0C01;
pub const READ_BUFFER: GLenum = 0x0C02;
pub const SCISSOR_BOX: GLenum = 0x0C10;
pub const SCISSOR_TEST: GLenum = 0x0C11;
pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const COLOR_WRITEMASK: GLenum = 0x0C23;
pub const DOUBLEBUFFER: GLenum = 0x0C32;
pub const STEREO: GLenum = 0x0C33;
pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
pub const PACK_LSB_FIRST: GLenum = 0x0D01;
pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const SUBPIXEL_BITS: GLenum = 0x0D50;
pub const TEXTURE_1D: GLenum = 0x0DE0;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE_WIDTH: GLenum = 0x1000;
pub const TEXTURE_HEIGHT: GLenum = 0x1001;
pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const DONT_CARE: GLenum = 0x1100;
pub const FASTEST: GLenum = 0x1101;
pub const NICEST: GLenum = 0x1102;
pub const BYTE: GLenum = 0x1400;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const SHORT: GLenum = 0x1402;
pub const UNSIGNED_SHORT: GLenum = 0x1403;
pub const INT: GLenum = 0x1404;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const FLOAT: GLenum = 0x1406;
pub const STACK_OVERFLOW: GLenum = 0x0503;
pub const STACK_UNDERFLOW: GLenum = 0x0504;
pub const CLEAR: GLenum = 0x1500;
pub const AND: GLenum = 0x1501;
pub const AND_REVERSE: GLenum = 0x1502;
pub const COPY: GLenum = 0x1503;
pub const AND_INVERTED: GLenum = 0x1504;
pub const NOOP: GLenum = 0x1505;
pub const XOR: GLenum = 0x1506;
pub const OR: GLenum = 0x1507;
pub const NOR: GLenum = 0x1508;
pub const EQUIV: GLenum = 0x1509;
pub const INVERT: GLenum = 0x150A;
pub const OR_REVERSE: GLenum = 0x150B;
pub const COPY_INVERTED: GLenum = 0x150C;
pub const OR_INVERTED: GLenum = 0x150D;
pub const NAND: GLenum = 0x150E;
pub const SET: GLenum = 0x150F;
pub const TEXTURE: GLenum = 0x1702;
pub const COLOR: GLenum = 0x1800;
pub const DEPTH: GLenum = 0x1801;
pub const STENCIL: GLenum = 0x1802;
pub const STENCIL_INDEX: GLenum = 0x1901;
pub const DEPTH_COMPONENT: GLenum = 0x1902;
pub const RED: GLenum = 0x1903;
pub const GREEN: GLenum = 0x1904;
pub const BLUE: GLenum = 0x1905;
pub const ALPHA: GLenum = 0x1906;
pub const RGB: GLenum = 0x1907;
pub const RGBA: GLenum = 0x1908;
pub const POINT: GLenum = 0x1B00;
pub const LINE: GLenum = 0x1B01;
pub const FILL: GLenum = 0x1B02;
pub const KEEP: GLenum = 0x1E00;
pub const REPLACE: GLenum = 0x1E01;
pub const INCR: GLenum = 0x1E02;
pub const DECR: GLenum = 0x1E03;
pub const VENDOR: GLenum = 0x1F00;
pub const RENDERER: GLenum = 0x1F01;
pub const VERSION: GLenum = 0x1F02;
pub const EXTENSIONS: GLenum = 0x1F03;
pub const NEAREST: GLenum = 0x2600;
pub const LINEAR: GLenum = 0x2601;
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const REPEAT: GLenum = 0x2901;
pub type PFNGLCULLFACEPROC = unsafe extern "system" fn(mode: GLenum);
pub type PFNGLFRONTFACEPROC = unsafe extern "system" fn(mode: GLenum);
pub type PFNGLHINTPROC = unsafe extern "system" fn(target: GLenum, mode: GLenum);
pub type PFNGLLINEWIDTHPROC = unsafe extern "system" fn(width: GLfloat);
pub type PFNGLPOINTSIZEPROC = unsafe extern "system" fn(size: GLfloat);
pub type PFNGLPOLYGONMODEPROC = unsafe extern "system" fn(face: GLenum, mode: GLenum);
pub type PFNGLSCISSORPROC =
    unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLTEXPARAMETERFPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);
pub type PFNGLTEXPARAMETERFVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);
pub type PFNGLTEXPARAMETERIPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
pub type PFNGLTEXPARAMETERIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
pub type PFNGLTEXIMAGE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
);
pub type PFNGLTEXIMAGE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
);
pub type PFNGLDRAWBUFFERPROC = unsafe extern "system" fn(buf: GLenum);
pub type PFNGLCLEARPROC = unsafe extern "system" fn(mask: GLbitfield);
pub type PFNGLCLEARCOLORPROC =
    unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
pub type PFNGLCLEARSTENCILPROC = unsafe extern "system" fn(s: GLint);
pub type PFNGLCLEARDEPTHPROC = unsafe extern "system" fn(depth: GLdouble);
pub type PFNGLSTENCILMASKPROC = unsafe extern "system" fn(mask: GLuint);
pub type PFNGLCOLORMASKPROC =
    unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
pub type PFNGLDEPTHMASKPROC = unsafe extern "system" fn(flag: GLboolean);
pub type PFNGLDISABLEPROC = unsafe extern "system" fn(cap: GLenum);
pub type PFNGLENABLEPROC = unsafe extern "system" fn(cap: GLenum);
pub type PFNGLFINISHPROC = unsafe extern "system" fn();
pub type PFNGLFLUSHPROC = unsafe extern "system" fn();
pub type PFNGLBLENDFUNCPROC = unsafe extern "system" fn(sfactor: GLenum, dfactor: GLenum);
pub type PFNGLLOGICOPPROC = unsafe extern "system" fn(opcode: GLenum);
pub type PFNGLSTENCILFUNCPROC = unsafe extern "system" fn(func: GLenum, ref_: GLint, mask: GLuint);
pub type PFNGLSTENCILOPPROC = unsafe extern "system" fn(fail: GLenum, zfail: GLenum, zpass: GLenum);
pub type PFNGLDEPTHFUNCPROC = unsafe extern "system" fn(func: GLenum);
pub type PFNGLPIXELSTOREFPROC = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
pub type PFNGLPIXELSTOREIPROC = unsafe extern "system" fn(pname: GLenum, param: GLint);
pub type PFNGLREADBUFFERPROC = unsafe extern "system" fn(src: GLenum);
pub type PFNGLREADPIXELSPROC = unsafe extern "system" fn(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
);
pub type PFNGLGETBOOLEANVPROC = unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean);
pub type PFNGLGETDOUBLEVPROC = unsafe extern "system" fn(pname: GLenum, data: *mut GLdouble);
pub type PFNGLGETERRORPROC = unsafe extern "system" fn() -> GLenum;
pub type PFNGLGETFLOATVPROC = unsafe extern "system" fn(pname: GLenum, data: *mut GLfloat);
pub type PFNGLGETINTEGERVPROC = unsafe extern "system" fn(pname: GLenum, data: *mut GLint);
pub type PFNGLGETSTRINGPROC = unsafe extern "system" fn(name: GLenum) -> *const GLubyte;
pub type PFNGLGETTEXIMAGEPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
);
pub type PFNGLGETTEXPARAMETERFVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETTEXPARAMETERIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXLEVELPARAMETERFVPROC =
    unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETTEXLEVELPARAMETERIVPROC =
    unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
pub type PFNGLISENABLEDPROC = unsafe extern "system" fn(cap: GLenum) -> GLboolean;
pub type PFNGLDEPTHRANGEPROC = unsafe extern "system" fn(n: GLdouble, f: GLdouble);
pub type PFNGLVIEWPORTPROC =
    unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

// 1.1
pub type GLclampf = f32;
pub type GLclampd = f64;
pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub const DOUBLE: GLenum = 0x140A;
pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
pub const R3_G3_B2: GLenum = 0x2A10;
pub const RGB4: GLenum = 0x804F;
pub const RGB5: GLenum = 0x8050;
pub const RGB8: GLenum = 0x8051;
pub const RGB10: GLenum = 0x8052;
pub const RGB12: GLenum = 0x8053;
pub const RGB16: GLenum = 0x8054;
pub const RGBA2: GLenum = 0x8055;
pub const RGBA4: GLenum = 0x8056;
pub const RGB5_A1: GLenum = 0x8057;
pub const RGBA8: GLenum = 0x8058;
pub const RGB10_A2: GLenum = 0x8059;
pub const RGBA12: GLenum = 0x805A;
pub const RGBA16: GLenum = 0x805B;
pub const VERTEX_ARRAY: GLenum = 0x8074;
pub type PFNGLDRAWARRAYSPROC =
    unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);
pub type PFNGLDRAWELEMENTSPROC =
    unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);
pub type PFNGLGETPOINTERVPROC = unsafe extern "system" fn(pname: GLenum, params: *mut *mut c_void);
pub type PFNGLPOLYGONOFFSETPROC = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);
pub type PFNGLCOPYTEXIMAGE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    border: GLint,
);
pub type PFNGLCOPYTEXIMAGE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
);
pub type PFNGLCOPYTEXSUBIMAGE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
);
pub type PFNGLCOPYTEXSUBIMAGE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
);
pub type PFNGLTEXSUBIMAGE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
);
pub type PFNGLTEXSUBIMAGE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
);
pub type PFNGLBINDTEXTUREPROC = unsafe extern "system" fn(target: GLenum, texture: GLuint);
pub type PFNGLDELETETEXTURESPROC = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);
pub type PFNGLGENTEXTURESPROC = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);
pub type PFNGLISTEXTUREPROC = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

// 1.2
pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub const TEXTURE_3D: GLenum = 0x806F;
pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
pub const TEXTURE_DEPTH: GLenum = 0x8071;
pub const TEXTURE_WRAP_R: GLenum = 0x8072;
pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub const BGR: GLenum = 0x80E0;
pub const BGRA: GLenum = 0x80E1;
pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub type PFNGLDRAWRANGEELEMENTSPROC = unsafe extern "system" fn(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
);
pub type PFNGLTEXIMAGE3DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
);
pub type PFNGLTEXSUBIMAGE3DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
);
pub type PFNGLCOPYTEXSUBIMAGE3DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
);

// 1.3
pub const TEXTURE0: GLenum = 0x84C0;
pub const TEXTURE1: GLenum = 0x84C1;
pub const TEXTURE2: GLenum = 0x84C2;
pub const TEXTURE3: GLenum = 0x84C3;
pub const TEXTURE4: GLenum = 0x84C4;
pub const TEXTURE5: GLenum = 0x84C5;
pub const TEXTURE6: GLenum = 0x84C6;
pub const TEXTURE7: GLenum = 0x84C7;
pub const TEXTURE8: GLenum = 0x84C8;
pub const TEXTURE9: GLenum = 0x84C9;
pub const TEXTURE10: GLenum = 0x84CA;
pub const TEXTURE11: GLenum = 0x84CB;
pub const TEXTURE12: GLenum = 0x84CC;
pub const TEXTURE13: GLenum = 0x84CD;
pub const TEXTURE14: GLenum = 0x84CE;
pub const TEXTURE15: GLenum = 0x84CF;
pub const TEXTURE16: GLenum = 0x84D0;
pub const TEXTURE17: GLenum = 0x84D1;
pub const TEXTURE18: GLenum = 0x84D2;
pub const TEXTURE19: GLenum = 0x84D3;
pub const TEXTURE20: GLenum = 0x84D4;
pub const TEXTURE21: GLenum = 0x84D5;
pub const TEXTURE22: GLenum = 0x84D6;
pub const TEXTURE23: GLenum = 0x84D7;
pub const TEXTURE24: GLenum = 0x84D8;
pub const TEXTURE25: GLenum = 0x84D9;
pub const TEXTURE26: GLenum = 0x84DA;
pub const TEXTURE27: GLenum = 0x84DB;
pub const TEXTURE28: GLenum = 0x84DC;
pub const TEXTURE29: GLenum = 0x84DD;
pub const TEXTURE30: GLenum = 0x84DE;
pub const TEXTURE31: GLenum = 0x84DF;
pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const MULTISAMPLE: GLenum = 0x809D;
pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const SAMPLES: GLenum = 0x80A9;
pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const COMPRESSED_RGB: GLenum = 0x84ED;
pub const COMPRESSED_RGBA: GLenum = 0x84EE;
pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const CLAMP_TO_BORDER: GLenum = 0x812D;
pub type PFNGLACTIVETEXTUREPROC = unsafe extern "system" fn(texture: GLenum);
pub type PFNGLSAMPLECOVERAGEPROC = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);
pub type PFNGLCOMPRESSEDTEXIMAGE3DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    image_size: GLsizei,
    data: *const c_void,
);
pub type PFNGLCOMPRESSEDTEXIMAGE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    image_size: GLsizei,
    data: *const c_void,
);
pub type PFNGLCOMPRESSEDTEXIMAGE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    image_size: GLsizei,
    data: *const c_void,
);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    image_size: GLsizei,
    data: *const c_void,
);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    image_size: GLsizei,
    data: *const c_void,
);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    image_size: GLsizei,
    data: *const c_void,
);
pub type PFNGLGETCOMPRESSEDTEXIMAGEPROC =
    unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut c_void);

// 1.4
pub const BLEND_DST_RGB: GLenum = 0x80C8;
pub const BLEND_SRC_RGB: GLenum = 0x80C9;
pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
pub const MIRRORED_REPEAT: GLenum = 0x8370;
pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub const INCR_WRAP: GLenum = 0x8507;
pub const DECR_WRAP: GLenum = 0x8508;
pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub const BLEND_COLOR: GLenum = 0x8005;
pub const BLEND_EQUATION: GLenum = 0x8009;
pub const CONSTANT_COLOR: GLenum = 0x8001;
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const CONSTANT_ALPHA: GLenum = 0x8003;
pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const FUNC_ADD: GLenum = 0x8006;
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const FUNC_SUBTRACT: GLenum = 0x800A;
pub const MIN: GLenum = 0x8007;
pub const MAX: GLenum = 0x8008;
pub type PFNGLBLENDFUNCSEPARATEPROC = unsafe extern "system" fn(
    sfactor_rgb: GLenum,
    dfactor_rgb: GLenum,
    sfactor_alpha: GLenum,
    dfactor_alpha: GLenum,
);
pub type PFNGLMULTIDRAWARRAYSPROC = unsafe extern "system" fn(
    mode: GLenum,
    first: *const GLint,
    count: *const GLsizei,
    drawcount: GLsizei,
);
pub type PFNGLMULTIDRAWELEMENTSPROC = unsafe extern "system" fn(
    mode: GLenum,
    count: *const GLsizei,
    type_: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
);
pub type PFNGLPOINTPARAMETERFPROC = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
pub type PFNGLPOINTPARAMETERFVPROC =
    unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);
pub type PFNGLPOINTPARAMETERIPROC = unsafe extern "system" fn(pname: GLenum, param: GLint);
pub type PFNGLPOINTPARAMETERIVPROC = unsafe extern "system" fn(pname: GLenum, params: *const GLint);
pub type PFNGLBLENDCOLORPROC =
    unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
pub type PFNGLBLENDEQUATIONPROC = unsafe extern "system" fn(mode: GLenum);

// 1.5
pub type GLsizeiptr = isize;
pub type GLintptr = isize;
pub const BUFFER_SIZE: GLenum = 0x8764;
pub const BUFFER_USAGE: GLenum = 0x8765;
pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const CURRENT_QUERY: GLenum = 0x8865;
pub const QUERY_RESULT: GLenum = 0x8866;
pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const READ_ONLY: GLenum = 0x88B8;
pub const WRITE_ONLY: GLenum = 0x88B9;
pub const READ_WRITE: GLenum = 0x88BA;
pub const BUFFER_ACCESS: GLenum = 0x88BB;
pub const BUFFER_MAPPED: GLenum = 0x88BC;
pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub const STREAM_DRAW: GLenum = 0x88E0;
pub const STREAM_READ: GLenum = 0x88E1;
pub const STREAM_COPY: GLenum = 0x88E2;
pub const STATIC_DRAW: GLenum = 0x88E4;
pub const STATIC_READ: GLenum = 0x88E5;
pub const STATIC_COPY: GLenum = 0x88E6;
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
pub const DYNAMIC_READ: GLenum = 0x88E9;
pub const DYNAMIC_COPY: GLenum = 0x88EA;
pub const SAMPLES_PASSED: GLenum = 0x8914;
pub const SRC1_ALPHA: GLenum = 0x8589;
pub type PFNGLGENQUERIESPROC = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
pub type PFNGLDELETEQUERIESPROC = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
pub type PFNGLISQUERYPROC = unsafe extern "system" fn(id: GLuint) -> GLboolean;
pub type PFNGLBEGINQUERYPROC = unsafe extern "system" fn(target: GLenum, id: GLuint);
pub type PFNGLENDQUERYPROC = unsafe extern "system" fn(target: GLenum);
pub type PFNGLGETQUERYIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYOBJECTIVPROC =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYOBJECTUIVPROC =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLBINDBUFFERPROC = unsafe extern "system" fn(target: GLenum, buffer: GLuint);
pub type PFNGLDELETEBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);
pub type PFNGLGENBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
pub type PFNGLISBUFFERPROC = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;
pub type PFNGLBUFFERDATAPROC =
    unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type PFNGLBUFFERSUBDATAPROC = unsafe extern "system" fn(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
);
pub type PFNGLGETBUFFERSUBDATAPROC = unsafe extern "system" fn(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut c_void,
);
pub type PFNGLMAPBUFFERPROC =
    unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut c_void;
pub type PFNGLUNMAPBUFFERPROC = unsafe extern "system" fn(target: GLenum) -> GLboolean;
pub type PFNGLGETBUFFERPARAMETERIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETBUFFERPOINTERVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut c_void);

// 2.0
pub type GLchar = c_char;
pub type GLshort = i16;
pub type GLbyte = i8;
pub type GLushort = u16;
pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub const DRAW_BUFFER0: GLenum = 0x8825;
pub const DRAW_BUFFER1: GLenum = 0x8826;
pub const DRAW_BUFFER2: GLenum = 0x8827;
pub const DRAW_BUFFER3: GLenum = 0x8828;
pub const DRAW_BUFFER4: GLenum = 0x8829;
pub const DRAW_BUFFER5: GLenum = 0x882A;
pub const DRAW_BUFFER6: GLenum = 0x882B;
pub const DRAW_BUFFER7: GLenum = 0x882C;
pub const DRAW_BUFFER8: GLenum = 0x882D;
pub const DRAW_BUFFER9: GLenum = 0x882E;
pub const DRAW_BUFFER10: GLenum = 0x882F;
pub const DRAW_BUFFER11: GLenum = 0x8830;
pub const DRAW_BUFFER12: GLenum = 0x8831;
pub const DRAW_BUFFER13: GLenum = 0x8832;
pub const DRAW_BUFFER14: GLenum = 0x8833;
pub const DRAW_BUFFER15: GLenum = 0x8834;
pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const SHADER_TYPE: GLenum = 0x8B4F;
pub const FLOAT_VEC2: GLenum = 0x8B50;
pub const FLOAT_VEC3: GLenum = 0x8B51;
pub const FLOAT_VEC4: GLenum = 0x8B52;
pub const INT_VEC2: GLenum = 0x8B53;
pub const INT_VEC3: GLenum = 0x8B54;
pub const INT_VEC4: GLenum = 0x8B55;
pub const BOOL: GLenum = 0x8B56;
pub const BOOL_VEC2: GLenum = 0x8B57;
pub const BOOL_VEC3: GLenum = 0x8B58;
pub const BOOL_VEC4: GLenum = 0x8B59;
pub const FLOAT_MAT2: GLenum = 0x8B5A;
pub const FLOAT_MAT3: GLenum = 0x8B5B;
pub const FLOAT_MAT4: GLenum = 0x8B5C;
pub const SAMPLER_1D: GLenum = 0x8B5D;
pub const SAMPLER_2D: GLenum = 0x8B5E;
pub const SAMPLER_3D: GLenum = 0x8B5F;
pub const SAMPLER_CUBE: GLenum = 0x8B60;
pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub const DELETE_STATUS: GLenum = 0x8B80;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const VALIDATE_STATUS: GLenum = 0x8B83;
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const ATTACHED_SHADERS: GLenum = 0x8B85;
pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub const LOWER_LEFT: GLenum = 0x8CA1;
pub const UPPER_LEFT: GLenum = 0x8CA2;
pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub type PFNGLBLENDEQUATIONSEPARATEPROC =
    unsafe extern "system" fn(mode_rgb: GLenum, mode_alpha: GLenum);
pub type PFNGLDRAWBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);
pub type PFNGLSTENCILOPSEPARATEPROC =
    unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
pub type PFNGLSTENCILFUNCSEPARATEPROC =
    unsafe extern "system" fn(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
pub type PFNGLSTENCILMASKSEPARATEPROC = unsafe extern "system" fn(face: GLenum, mask: GLuint);
pub type PFNGLATTACHSHADERPROC = unsafe extern "system" fn(program: GLuint, shader: GLuint);
pub type PFNGLBINDATTRIBLOCATIONPROC =
    unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);
pub type PFNGLCOMPILESHADERPROC = unsafe extern "system" fn(shader: GLuint);
pub type PFNGLCREATEPROGRAMPROC = unsafe extern "system" fn() -> GLuint;
pub type PFNGLCREATESHADERPROC = unsafe extern "system" fn(type_: GLenum) -> GLuint;
pub type PFNGLDELETEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLDELETESHADERPROC = unsafe extern "system" fn(shader: GLuint);
pub type PFNGLDETACHSHADERPROC = unsafe extern "system" fn(program: GLuint, shader: GLuint);
pub type PFNGLDISABLEVERTEXATTRIBARRAYPROC = unsafe extern "system" fn(index: GLuint);
pub type PFNGLENABLEVERTEXATTRIBARRAYPROC = unsafe extern "system" fn(index: GLuint);
pub type PFNGLGETACTIVEATTRIBPROC = unsafe extern "system" fn(
    program: GLuint,
    index: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
);
pub type PFNGLGETACTIVEUNIFORMPROC = unsafe extern "system" fn(
    program: GLuint,
    index: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
);
pub type PFNGLGETATTACHEDSHADERSPROC = unsafe extern "system" fn(
    program: GLuint,
    max_count: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
);
pub type PFNGLGETATTRIBLOCATIONPROC =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETPROGRAMIVPROC =
    unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETPROGRAMINFOLOGPROC = unsafe extern "system" fn(
    program: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    info_log: *mut GLchar,
);
pub type PFNGLGETSHADERIVPROC =
    unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSHADERINFOLOGPROC = unsafe extern "system" fn(
    shader: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    info_log: *mut GLchar,
);
pub type PFNGLGETSHADERSOURCEPROC = unsafe extern "system" fn(
    shader: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
);
pub type PFNGLGETUNIFORMLOCATIONPROC =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETUNIFORMFVPROC =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);
pub type PFNGLGETUNIFORMIVPROC =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);
pub type PFNGLGETVERTEXATTRIBDVPROC =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
pub type PFNGLGETVERTEXATTRIBFVPROC =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETVERTEXATTRIBIVPROC =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETVERTEXATTRIBPOINTERVPROC =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
pub type PFNGLISPROGRAMPROC = unsafe extern "system" fn(program: GLuint) -> GLboolean;
pub type PFNGLISSHADERPROC = unsafe extern "system" fn(shader: GLuint) -> GLboolean;
pub type PFNGLLINKPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLSHADERSOURCEPROC = unsafe extern "system" fn(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
);
pub type PFNGLUSEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLUNIFORM1FPROC = unsafe extern "system" fn(location: GLint, v0: GLfloat);
pub type PFNGLUNIFORM2FPROC = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);
pub type PFNGLUNIFORM3FPROC =
    unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
pub type PFNGLUNIFORM4FPROC =
    unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
pub type PFNGLUNIFORM1IPROC = unsafe extern "system" fn(location: GLint, v0: GLint);
pub type PFNGLUNIFORM2IPROC = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);
pub type PFNGLUNIFORM3IPROC =
    unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);
pub type PFNGLUNIFORM4IPROC =
    unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
pub type PFNGLUNIFORM1FVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM2FVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM3FVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM4FVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM1IVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM2IVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM3IVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM4IVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORMMATRIX2FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX3FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX4FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLVALIDATEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLVERTEXATTRIB1DPROC = unsafe extern "system" fn(index: GLuint, x: GLdouble);
pub type PFNGLVERTEXATTRIB1DVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB1FPROC = unsafe extern "system" fn(index: GLuint, x: GLfloat);
pub type PFNGLVERTEXATTRIB1FVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB1SPROC = unsafe extern "system" fn(index: GLuint, x: GLshort);
pub type PFNGLVERTEXATTRIB1SVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB2DPROC =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
pub type PFNGLVERTEXATTRIB2DVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB2FPROC = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);
pub type PFNGLVERTEXATTRIB2FVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB2SPROC = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);
pub type PFNGLVERTEXATTRIB2SVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB3DPROC =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type PFNGLVERTEXATTRIB3DVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB3FPROC =
    unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
pub type PFNGLVERTEXATTRIB3FVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB3SPROC =
    unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
pub type PFNGLVERTEXATTRIB3SVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4NBVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIB4NIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIB4NSVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4NUBPROC =
    unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
pub type PFNGLVERTEXATTRIB4NUBVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIB4NUIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIB4NUSVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIB4BVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIB4DPROC =
    unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLVERTEXATTRIB4DVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB4FPROC =
    unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
pub type PFNGLVERTEXATTRIB4FVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB4IVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIB4SPROC =
    unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
pub type PFNGLVERTEXATTRIB4SVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4UBVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIB4UIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIB4USVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIBPOINTERPROC = unsafe extern "system" fn(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
);

// 2.1
pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub const FLOAT_MAT2X3: GLenum = 0x8B65;
pub const FLOAT_MAT2X4: GLenum = 0x8B66;
pub const FLOAT_MAT3X2: GLenum = 0x8B67;
pub const FLOAT_MAT3X4: GLenum = 0x8B68;
pub const FLOAT_MAT4X2: GLenum = 0x8B69;
pub const FLOAT_MAT4X3: GLenum = 0x8B6A;
pub const SRGB: GLenum = 0x8C40;
pub const SRGB8: GLenum = 0x8C41;
pub const SRGB_ALPHA: GLenum = 0x8C42;
pub const SRGB8_ALPHA8: GLenum = 0x8C43;
pub const COMPRESSED_SRGB: GLenum = 0x8C48;
pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub type PFNGLUNIFORMMATRIX2X3FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX3X2FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX2X4FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX4X2FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX3X4FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);
pub type PFNGLUNIFORMMATRIX4X3FVPROC = unsafe extern "system" fn(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
);

// 3.0
pub type GLhalf = u16;
pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub const CLIP_DISTANCE0: GLenum = 0x3000;
pub const CLIP_DISTANCE1: GLenum = 0x3001;
pub const CLIP_DISTANCE2: GLenum = 0x3002;
pub const CLIP_DISTANCE3: GLenum = 0x3003;
pub const CLIP_DISTANCE4: GLenum = 0x3004;
pub const CLIP_DISTANCE5: GLenum = 0x3005;
pub const CLIP_DISTANCE6: GLenum = 0x3006;
pub const CLIP_DISTANCE7: GLenum = 0x3007;
pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub const MAJOR_VERSION: GLenum = 0x821B;
pub const MINOR_VERSION: GLenum = 0x821C;
pub const NUM_EXTENSIONS: GLenum = 0x821D;
pub const CONTEXT_FLAGS: GLenum = 0x821E;
pub const COMPRESSED_RED: GLenum = 0x8225;
pub const COMPRESSED_RG: GLenum = 0x8226;
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
pub const RGBA32F: GLenum = 0x8814;
pub const RGB32F: GLenum = 0x8815;
pub const RGBA16F: GLenum = 0x881A;
pub const RGB16F: GLenum = 0x881B;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub const CLAMP_READ_COLOR: GLenum = 0x891C;
pub const FIXED_ONLY: GLenum = 0x891D;
pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub const R11F_G11F_B10F: GLenum = 0x8C3A;
pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub const RGB9_E5: GLenum = 0x8C3D;
pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub const RGBA32UI: GLenum = 0x8D70;
pub const RGB32UI: GLenum = 0x8D71;
pub const RGBA16UI: GLenum = 0x8D76;
pub const RGB16UI: GLenum = 0x8D77;
pub const RGBA8UI: GLenum = 0x8D7C;
pub const RGB8UI: GLenum = 0x8D7D;
pub const RGBA32I: GLenum = 0x8D82;
pub const RGB32I: GLenum = 0x8D83;
pub const RGBA16I: GLenum = 0x8D88;
pub const RGB16I: GLenum = 0x8D89;
pub const RGBA8I: GLenum = 0x8D8E;
pub const RGB8I: GLenum = 0x8D8F;
pub const RED_INTEGER: GLenum = 0x8D94;
pub const GREEN_INTEGER: GLenum = 0x8D95;
pub const BLUE_INTEGER: GLenum = 0x8D96;
pub const RGB_INTEGER: GLenum = 0x8D98;
pub const RGBA_INTEGER: GLenum = 0x8D99;
pub const BGR_INTEGER: GLenum = 0x8D9A;
pub const BGRA_INTEGER: GLenum = 0x8D9B;
pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub const QUERY_WAIT: GLenum = 0x8E13;
pub const QUERY_NO_WAIT: GLenum = 0x8E14;
pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const DEPTH_STENCIL: GLenum = 0x84F9;
pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const RENDERBUFFER: GLenum = 0x8D41;
pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const STENCIL_INDEX1: GLenum = 0x8D46;
pub const STENCIL_INDEX4: GLenum = 0x8D47;
pub const STENCIL_INDEX8: GLenum = 0x8D48;
pub const STENCIL_INDEX16: GLenum = 0x8D49;
pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub const MAX_SAMPLES: GLenum = 0x8D57;
pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub const HALF_FLOAT: GLenum = 0x140B;
pub const MAP_READ_BIT: GLbitfield = 0x0001;
pub const MAP_WRITE_BIT: GLbitfield = 0x0002;
pub const MAP_INVALIDATE_RANGE_BIT: GLbitfield = 0x0004;
pub const MAP_INVALIDATE_BUFFER_BIT: GLbitfield = 0x0008;
pub const MAP_FLUSH_EXPLICIT_BIT: GLbitfield = 0x0010;
pub const MAP_UNSYNCHRONIZED_BIT: GLbitfield = 0x0020;
pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub const RG: GLenum = 0x8227;
pub const RG_INTEGER: GLenum = 0x8228;
pub const R8: GLenum = 0x8229;
pub const R16: GLenum = 0x822A;
pub const RG8: GLenum = 0x822B;
pub const RG16: GLenum = 0x822C;
pub const R16F: GLenum = 0x822D;
pub const R32F: GLenum = 0x822E;
pub const RG16F: GLenum = 0x822F;
pub const RG32F: GLenum = 0x8230;
pub const R8I: GLenum = 0x8231;
pub const R8UI: GLenum = 0x8232;
pub const R16I: GLenum = 0x8233;
pub const R16UI: GLenum = 0x8234;
pub const R32I: GLenum = 0x8235;
pub const R32UI: GLenum = 0x8236;
pub const RG8I: GLenum = 0x8237;
pub const RG8UI: GLenum = 0x8238;
pub const RG16I: GLenum = 0x8239;
pub const RG16UI: GLenum = 0x823A;
pub const RG32I: GLenum = 0x823B;
pub const RG32UI: GLenum = 0x823C;
pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub type PFNGLCOLORMASKIPROC = unsafe extern "system" fn(
    index: GLuint,
    r: GLboolean,
    g: GLboolean,
    b: GLboolean,
    a: GLboolean,
);
#[allow(non_camel_case_types)]
pub type PFNGLGETBOOLEANI_VPROC =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean);
#[allow(non_camel_case_types)]
pub type PFNGLGETINTEGERI_VPROC =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);
pub type PFNGLENABLEIPROC = unsafe extern "system" fn(target: GLenum, index: GLuint);
pub type PFNGLDISABLEIPROC = unsafe extern "system" fn(target: GLenum, index: GLuint);
pub type PFNGLISENABLEDIPROC =
    unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;
pub type PFNGLBEGINTRANSFORMFEEDBACKPROC = unsafe extern "system" fn(primitive_mode: GLenum);
pub type PFNGLENDTRANSFORMFEEDBACKPROC = unsafe extern "system" fn();
pub type PFNGLBINDBUFFERRANGEPROC = unsafe extern "system" fn(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
);
pub type PFNGLBINDBUFFERBASEPROC =
    unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);
pub type PFNGLTRANSFORMFEEDBACKVARYINGSPROC = unsafe extern "system" fn(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const GLchar,
    buffer_mode: GLenum,
);
pub type PFNGLGETTRANSFORMFEEDBACKVARYINGPROC = unsafe extern "system" fn(
    program: GLuint,
    index: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    type_: *mut GLenum,
    name: *mut GLchar,
);
pub type PFNGLCLAMPCOLORPROC = unsafe extern "system" fn(target: GLenum, clamp: GLenum);
pub type PFNGLBEGINCONDITIONALRENDERPROC = unsafe extern "system" fn(id: GLuint, mode: GLenum);
pub type PFNGLENDCONDITIONALRENDERPROC = unsafe extern "system" fn();
pub type PFNGLVERTEXATTRIBIPOINTERPROC = unsafe extern "system" fn(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
);
pub type PFNGLGETVERTEXATTRIBIIVPROC =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETVERTEXATTRIBIUIVPROC =
    unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLVERTEXATTRIBI1IPROC = unsafe extern "system" fn(index: GLuint, x: GLint);
pub type PFNGLVERTEXATTRIBI2IPROC = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);
pub type PFNGLVERTEXATTRIBI3IPROC =
    unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);
pub type PFNGLVERTEXATTRIBI4IPROC =
    unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
pub type PFNGLVERTEXATTRIBI1UIPROC = unsafe extern "system" fn(index: GLuint, x: GLuint);
pub type PFNGLVERTEXATTRIBI2UIPROC = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);
pub type PFNGLVERTEXATTRIBI3UIPROC =
    unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
pub type PFNGLVERTEXATTRIBI4UIPROC =
    unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
pub type PFNGLVERTEXATTRIBI1IVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI2IVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI3IVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI4IVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI1UIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI2UIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI3UIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI4UIVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI4BVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIBI4SVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIBI4UBVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIBI4USVPROC = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
pub type PFNGLGETUNIFORMUIVPROC =
    unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);
pub type PFNGLBINDFRAGDATALOCATIONPROC =
    unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);
pub type PFNGLGETFRAGDATALOCATIONPROC =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLUNIFORM1UIPROC = unsafe extern "system" fn(location: GLint, v0: GLuint);
pub type PFNGLUNIFORM2UIPROC = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);
pub type PFNGLUNIFORM3UIPROC =
    unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
pub type PFNGLUNIFORM4UIPROC =
    unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
pub type PFNGLUNIFORM1UIVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORM2UIVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORM3UIVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORM4UIVPROC =
    unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLTEXPARAMETERIIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
pub type PFNGLTEXPARAMETERIUIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);
pub type PFNGLGETTEXPARAMETERIIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXPARAMETERIUIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);
pub type PFNGLCLEARBUFFERIVPROC =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
pub type PFNGLCLEARBUFFERUIVPROC =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
pub type PFNGLCLEARBUFFERFVPROC =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
pub type PFNGLCLEARBUFFERFIPROC =
    unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
pub type PFNGLGETSTRINGIPROC =
    unsafe extern "system" fn(name: GLenum, index: GLuint) -> *const GLubyte;
pub type PFNGLISRENDERBUFFERPROC = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;
pub type PFNGLBINDRENDERBUFFERPROC =
    unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);
pub type PFNGLDELETERENDERBUFFERSPROC =
    unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);
pub type PFNGLGENRENDERBUFFERSPROC =
    unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
pub type PFNGLRENDERBUFFERSTORAGEPROC = unsafe extern "system" fn(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
);
pub type PFNGLGETRENDERBUFFERPARAMETERIVPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLISFRAMEBUFFERPROC = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;
pub type PFNGLBINDFRAMEBUFFERPROC = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);
pub type PFNGLDELETEFRAMEBUFFERSPROC =
    unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);
pub type PFNGLGENFRAMEBUFFERSPROC =
    unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
pub type PFNGLCHECKFRAMEBUFFERSTATUSPROC = unsafe extern "system" fn(target: GLenum) -> GLenum;
pub type PFNGLFRAMEBUFFERTEXTURE1DPROC = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
);
pub type PFNGLFRAMEBUFFERTEXTURE2DPROC = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
);
pub type PFNGLFRAMEBUFFERTEXTURE3DPROC = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
    zoffset: GLint,
);
pub type PFNGLFRAMEBUFFERRENDERBUFFERPROC = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
);
pub type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
);
pub type PFNGLGENERATEMIPMAPPROC = unsafe extern "system" fn(target: GLenum);
pub type PFNGLBLITFRAMEBUFFERPROC = unsafe extern "system" fn(
    src_x0: GLint,
    src_y0: GLint,
    src_x1: GLint,
    src_y1: GLint,
    dst_x0: GLint,
    dst_y0: GLint,
    dst_x1: GLint,
    dst_y1: GLint,
    mask: GLbitfield,
    filter: GLenum,
);
pub type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
);
pub type PFNGLFRAMEBUFFERTEXTURELAYERPROC = unsafe extern "system" fn(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
);
pub type PFNGLMAPBUFFERRANGEPROC = unsafe extern "system" fn(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
) -> *mut c_void;
pub type PFNGLFLUSHMAPPEDBUFFERRANGEPROC =
    unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);
pub type PFNGLBINDVERTEXARRAYPROC = unsafe extern "system" fn(array: GLuint);
pub type PFNGLDELETEVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);
pub type PFNGLGENVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
pub type PFNGLISVERTEXARRAYPROC = unsafe extern "system" fn(array: GLuint) -> GLboolean;

// 3.1
pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub const R8_SNORM: GLenum = 0x8F94;
pub const RG8_SNORM: GLenum = 0x8F95;
pub const RGB8_SNORM: GLenum = 0x8F96;
pub const RGBA8_SNORM: GLenum = 0x8F97;
pub const R16_SNORM: GLenum = 0x8F98;
pub const RG16_SNORM: GLenum = 0x8F99;
pub const RGB16_SNORM: GLenum = 0x8F9A;
pub const RGBA16_SNORM: GLenum = 0x8F9B;
pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub const COPY_READ_BUFFER: GLenum = 0x8F36;
pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub const UNIFORM_BUFFER: GLenum = 0x8A11;
pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const UNIFORM_TYPE: GLenum = 0x8A37;
pub const UNIFORM_SIZE: GLenum = 0x8A38;
pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub const INVALID_INDEX: GLenum = 0xFFFFFFFF;
pub type PFNGLDRAWARRAYSINSTANCEDPROC =
    unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
pub type PFNGLDRAWELEMENTSINSTANCEDPROC = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
);
pub type PFNGLTEXBUFFERPROC =
    unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);
pub type PFNGLPRIMITIVERESTARTINDEXPROC = unsafe extern "system" fn(index: GLuint);
pub type PFNGLCOPYBUFFERSUBDATAPROC = unsafe extern "system" fn(
    read_target: GLenum,
    write_target: GLenum,
    read_offset: GLintptr,
    write_offset: GLintptr,
    size: GLsizeiptr,
);
pub type PFNGLGETUNIFORMINDICESPROC = unsafe extern "system" fn(
    program: GLuint,
    uniform_count: GLsizei,
    uniform_names: *const *const GLchar,
    uniform_indices: *mut GLuint,
);
pub type PFNGLGETACTIVEUNIFORMSIVPROC = unsafe extern "system" fn(
    program: GLuint,
    uniform_count: GLsizei,
    uniform_indices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
);
pub type PFNGLGETACTIVEUNIFORMNAMEPROC = unsafe extern "system" fn(
    program: GLuint,
    uniform_index: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    uniform_name: *mut GLchar,
);
pub type PFNGLGETUNIFORMBLOCKINDEXPROC =
    unsafe extern "system" fn(program: GLuint, uniform_block_name: *const GLchar) -> GLuint;
pub type PFNGLGETACTIVEUNIFORMBLOCKIVPROC = unsafe extern "system" fn(
    program: GLuint,
    uniform_block_index: GLuint,
    pname: GLenum,
    params: *mut GLint,
);
pub type PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC = unsafe extern "system" fn(
    program: GLuint,
    uniform_block_index: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    uniform_block_name: *mut GLchar,
);
pub type PFNGLUNIFORMBLOCKBINDINGPROC = unsafe extern "system" fn(
    program: GLuint,
    uniform_block_index: GLuint,
    uniform_block_binding: GLuint,
);

// 3.2
#[repr(C)]
pub struct __GLsync {
    _unused: [u8; 0],
}
pub type GLsync = *mut __GLsync;
pub type GLuint64 = u64;
pub type GLint64 = i64;
pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub const LINES_ADJACENCY: GLenum = 0x000A;
pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
pub const DEPTH_CLAMP: GLenum = 0x864F;
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub const OBJECT_TYPE: GLenum = 0x9112;
pub const SYNC_CONDITION: GLenum = 0x9113;
pub const SYNC_STATUS: GLenum = 0x9114;
pub const SYNC_FLAGS: GLenum = 0x9115;
pub const SYNC_FENCE: GLenum = 0x9116;
pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub const UNSIGNALED: GLenum = 0x9118;
pub const SIGNALED: GLenum = 0x9119;
pub const ALREADY_SIGNALED: GLenum = 0x911A;
pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
pub const CONDITION_SATISFIED: GLenum = 0x911C;
pub const WAIT_FAILED: GLenum = 0x911D;
pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
pub const SYNC_FLUSH_COMMANDS_BIT: GLbitfield = 0x00000001;
pub const SAMPLE_POSITION: GLenum = 0x8E50;
pub const SAMPLE_MASK: GLenum = 0x8E51;
pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub const TEXTURE_SAMPLES: GLenum = 0x9106;
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
pub type PFNGLDRAWELEMENTSBASEVERTEXPROC = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    basevertex: GLint,
);
pub type PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC = unsafe extern "system" fn(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    basevertex: GLint,
);
pub type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC = unsafe extern "system" fn(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    basevertex: GLint,
);
pub type PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC = unsafe extern "system" fn(
    mode: GLenum,
    count: *const GLsizei,
    type_: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
    basevertex: *const GLint,
);
pub type PFNGLPROVOKINGVERTEXPROC = unsafe extern "system" fn(mode: GLenum);
pub type PFNGLFENCESYNCPROC =
    unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync;
pub type PFNGLISSYNCPROC = unsafe extern "system" fn(sync: GLsync) -> GLboolean;
pub type PFNGLDELETESYNCPROC = unsafe extern "system" fn(sync: GLsync);
pub type PFNGLCLIENTWAITSYNCPROC =
    unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
pub type PFNGLWAITSYNCPROC =
    unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
pub type PFNGLGETINTEGER64VPROC = unsafe extern "system" fn(pname: GLenum, data: *mut GLint64);
pub type PFNGLGETSYNCIVPROC = unsafe extern "system" fn(
    sync: GLsync,
    pname: GLenum,
    count: GLsizei,
    length: *mut GLsizei,
    values: *mut GLint,
);
#[allow(non_camel_case_types)]
pub type PFNGLGETINTEGER64I_VPROC =
    unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64);
pub type PFNGLGETBUFFERPARAMETERI64VPROC =
    unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64);
pub type PFNGLFRAMEBUFFERTEXTUREPROC =
    unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
pub type PFNGLTEXIMAGE2DMULTISAMPLEPROC = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
);
pub type PFNGLTEXIMAGE3DMULTISAMPLEPROC = unsafe extern "system" fn(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
);
pub type PFNGLGETMULTISAMPLEFVPROC =
    unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat);
pub type PFNGLSAMPLEMASKIPROC = unsafe extern "system" fn(mask_number: GLuint, mask: GLbitfield);

// 3.3
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
pub const SRC1_COLOR: GLenum = 0x88F9;
pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
pub const SAMPLER_BINDING: GLenum = 0x8919;
pub const RGB10_A2UI: GLenum = 0x906F;
pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
pub const TIME_ELAPSED: GLenum = 0x88BF;
pub const TIMESTAMP: GLenum = 0x8E28;
pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
pub type PFNGLBINDFRAGDATALOCATIONINDEXEDPROC = unsafe extern "system" fn(
    program: GLuint,
    color_number: GLuint,
    index: GLuint,
    name: *const GLchar,
);
pub type PFNGLGETFRAGDATAINDEXPROC =
    unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGENSAMPLERSPROC = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);
pub type PFNGLDELETESAMPLERSPROC =
    unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);
pub type PFNGLISSAMPLERPROC = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;
pub type PFNGLBINDSAMPLERPROC = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);
pub type PFNGLSAMPLERPARAMETERIPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint);
pub type PFNGLSAMPLERPARAMETERIVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
pub type PFNGLSAMPLERPARAMETERFPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat);
pub type PFNGLSAMPLERPARAMETERFVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat);
pub type PFNGLSAMPLERPARAMETERIIVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
pub type PFNGLSAMPLERPARAMETERIUIVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);
pub type PFNGLGETSAMPLERPARAMETERIVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSAMPLERPARAMETERIIVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSAMPLERPARAMETERFVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETSAMPLERPARAMETERIUIVPROC =
    unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLQUERYCOUNTERPROC = unsafe extern "system" fn(id: GLuint, target: GLenum);
pub type PFNGLGETQUERYOBJECTI64VPROC =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64);
pub type PFNGLGETQUERYOBJECTUI64VPROC =
    unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);
pub type PFNGLVERTEXATTRIBDIVISORPROC = unsafe extern "system" fn(index: GLuint, divisor: GLuint);
pub type PFNGLVERTEXATTRIBP1UIPROC =
    unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP1UIVPROC = unsafe extern "system" fn(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
);
pub type PFNGLVERTEXATTRIBP2UIPROC =
    unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP2UIVPROC = unsafe extern "system" fn(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
);
pub type PFNGLVERTEXATTRIBP3UIPROC =
    unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP3UIVPROC = unsafe extern "system" fn(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
);
pub type PFNGLVERTEXATTRIBP4UIPROC =
    unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP4UIVPROC = unsafe extern "system" fn(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
);

pub struct Gl {
    // 1.0
    cull_face: PFNGLCULLFACEPROC,
    front_face: PFNGLFRONTFACEPROC,
    hint: PFNGLHINTPROC,
    line_width: PFNGLLINEWIDTHPROC,
    point_size: PFNGLPOINTSIZEPROC,
    polygon_mode: PFNGLPOLYGONMODEPROC,
    scissor: PFNGLSCISSORPROC,
    tex_parameterf: PFNGLTEXPARAMETERFPROC,
    tex_parameterfv: PFNGLTEXPARAMETERFVPROC,
    tex_parameteri: PFNGLTEXPARAMETERIPROC,
    tex_parameteriv: PFNGLTEXPARAMETERIVPROC,
    tex_image_1d: PFNGLTEXIMAGE1DPROC,
    tex_image_2d: PFNGLTEXIMAGE2DPROC,
    draw_buffer: PFNGLDRAWBUFFERPROC,
    clear: PFNGLCLEARPROC,
    clear_color: PFNGLCLEARCOLORPROC,
    clear_stencil: PFNGLCLEARSTENCILPROC,
    clear_depth: PFNGLCLEARDEPTHPROC,
    stencil_mask: PFNGLSTENCILMASKPROC,
    color_mask: PFNGLCOLORMASKPROC,
    depth_mask: PFNGLDEPTHMASKPROC,
    disable: PFNGLDISABLEPROC,
    enable: PFNGLENABLEPROC,
    finish: PFNGLFINISHPROC,
    flush: PFNGLFLUSHPROC,
    blend_func: PFNGLBLENDFUNCPROC,
    logic_op: PFNGLLOGICOPPROC,
    stencil_func: PFNGLSTENCILFUNCPROC,
    stencil_op: PFNGLSTENCILOPPROC,
    depth_func: PFNGLDEPTHFUNCPROC,
    pixel_storef: PFNGLPIXELSTOREFPROC,
    pixel_storei: PFNGLPIXELSTOREIPROC,
    read_buffer: PFNGLREADBUFFERPROC,
    read_pixels: PFNGLREADPIXELSPROC,
    get_booleanv: PFNGLGETBOOLEANVPROC,
    get_doublev: PFNGLGETDOUBLEVPROC,
    get_error: PFNGLGETERRORPROC,
    get_floatv: PFNGLGETFLOATVPROC,
    get_integerv: PFNGLGETINTEGERVPROC,
    get_string: PFNGLGETSTRINGPROC,
    get_tex_image: PFNGLGETTEXIMAGEPROC,
    get_tex_parameterfv: PFNGLGETTEXPARAMETERFVPROC,
    get_tex_parameteriv: PFNGLGETTEXPARAMETERIVPROC,
    get_tex_level_parameterfv: PFNGLGETTEXLEVELPARAMETERFVPROC,
    get_tex_level_parameteriv: PFNGLGETTEXLEVELPARAMETERIVPROC,
    is_enabled: PFNGLISENABLEDPROC,
    depth_range: PFNGLDEPTHRANGEPROC,
    viewport: PFNGLVIEWPORTPROC,

    // 1.1
    draw_arrays: PFNGLDRAWARRAYSPROC,
    draw_elements: PFNGLDRAWELEMENTSPROC,
    get_pointerv: PFNGLGETPOINTERVPROC,
    polygon_offset: PFNGLPOLYGONOFFSETPROC,
    copy_tex_image_1d: PFNGLCOPYTEXIMAGE1DPROC,
    copy_tex_image_2d: PFNGLCOPYTEXIMAGE2DPROC,
    copy_tex_sub_image_1d: PFNGLCOPYTEXSUBIMAGE1DPROC,
    copy_tex_sub_image_2d: PFNGLCOPYTEXSUBIMAGE2DPROC,
    tex_sub_image_1d: PFNGLTEXSUBIMAGE1DPROC,
    tex_sub_image_2d: PFNGLTEXSUBIMAGE2DPROC,
    bind_texture: PFNGLBINDTEXTUREPROC,
    delete_textures: PFNGLDELETETEXTURESPROC,
    gen_textures: PFNGLGENTEXTURESPROC,
    is_texture: PFNGLISTEXTUREPROC,

    // 1.2
    draw_range_elements: PFNGLDRAWRANGEELEMENTSPROC,
    tex_image_3d: PFNGLTEXIMAGE3DPROC,
    tex_sub_image_3d: PFNGLTEXSUBIMAGE3DPROC,
    copy_tex_sub_image_3d: PFNGLCOPYTEXSUBIMAGE3DPROC,

    // 1.3
    active_texture: PFNGLACTIVETEXTUREPROC,
    sample_coverage: PFNGLSAMPLECOVERAGEPROC,
    compressed_tex_image_3d: PFNGLCOMPRESSEDTEXIMAGE3DPROC,
    compressed_tex_image_2d: PFNGLCOMPRESSEDTEXIMAGE2DPROC,
    compressed_tex_image_1d: PFNGLCOMPRESSEDTEXIMAGE1DPROC,
    compressed_tex_sub_image_3d: PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC,
    compressed_tex_sub_image_2d: PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC,
    compressed_tex_sub_image_1d: PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC,
    get_compressed_tex_image: PFNGLGETCOMPRESSEDTEXIMAGEPROC,

    // 1.4
    blend_func_separate: PFNGLBLENDFUNCSEPARATEPROC,
    multi_draw_arrays: PFNGLMULTIDRAWARRAYSPROC,
    multi_draw_elements: PFNGLMULTIDRAWELEMENTSPROC,
    point_parameterf: PFNGLPOINTPARAMETERFPROC,
    point_parameterfv: PFNGLPOINTPARAMETERFVPROC,
    point_parameteri: PFNGLPOINTPARAMETERIPROC,
    point_parameteriv: PFNGLPOINTPARAMETERIVPROC,
    blend_color: PFNGLBLENDCOLORPROC,
    blend_equation: PFNGLBLENDEQUATIONPROC,

    // 1.5
    gen_queries: PFNGLGENQUERIESPROC,
    delete_queries: PFNGLDELETEQUERIESPROC,
    is_query: PFNGLISQUERYPROC,
    begin_query: PFNGLBEGINQUERYPROC,
    end_query: PFNGLENDQUERYPROC,
    get_queryiv: PFNGLGETQUERYIVPROC,
    get_query_objectiv: PFNGLGETQUERYOBJECTIVPROC,
    get_query_objectuiv: PFNGLGETQUERYOBJECTUIVPROC,
    bind_buffer: PFNGLBINDBUFFERPROC,
    delete_buffers: PFNGLDELETEBUFFERSPROC,
    gen_buffers: PFNGLGENBUFFERSPROC,
    is_buffer: PFNGLISBUFFERPROC,
    buffer_data: PFNGLBUFFERDATAPROC,
    buffer_sub_data: PFNGLBUFFERSUBDATAPROC,
    get_buffer_sub_data: PFNGLGETBUFFERSUBDATAPROC,
    map_buffer: PFNGLMAPBUFFERPROC,
    unmap_buffer: PFNGLUNMAPBUFFERPROC,
    get_buffer_parameteriv: PFNGLGETBUFFERPARAMETERIVPROC,
    get_buffer_pointerv: PFNGLGETBUFFERPOINTERVPROC,

    // 2.0
    blend_equation_separate: PFNGLBLENDEQUATIONSEPARATEPROC,
    draw_buffers: PFNGLDRAWBUFFERSPROC,
    stencil_op_separate: PFNGLSTENCILOPSEPARATEPROC,
    stencil_func_separate: PFNGLSTENCILFUNCSEPARATEPROC,
    stencil_mask_separate: PFNGLSTENCILMASKSEPARATEPROC,
    attach_shader: PFNGLATTACHSHADERPROC,
    bind_attrib_location: PFNGLBINDATTRIBLOCATIONPROC,
    compile_shader: PFNGLCOMPILESHADERPROC,
    create_program: PFNGLCREATEPROGRAMPROC,
    create_shader: PFNGLCREATESHADERPROC,
    delete_program: PFNGLDELETEPROGRAMPROC,
    delete_shader: PFNGLDELETESHADERPROC,
    detach_shader: PFNGLDETACHSHADERPROC,
    disable_vertex_attrib_array: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
    enable_vertex_attrib_array: PFNGLENABLEVERTEXATTRIBARRAYPROC,
    get_active_attrib: PFNGLGETACTIVEATTRIBPROC,
    get_active_uniform: PFNGLGETACTIVEUNIFORMPROC,
    get_attached_shaders: PFNGLGETATTACHEDSHADERSPROC,
    get_attrib_location: PFNGLGETATTRIBLOCATIONPROC,
    get_programiv: PFNGLGETPROGRAMIVPROC,
    get_program_info_log: PFNGLGETPROGRAMINFOLOGPROC,
    get_shaderiv: PFNGLGETSHADERIVPROC,
    get_shader_info_log: PFNGLGETSHADERINFOLOGPROC,
    get_shader_source: PFNGLGETSHADERSOURCEPROC,
    get_uniform_location: PFNGLGETUNIFORMLOCATIONPROC,
    get_uniformfv: PFNGLGETUNIFORMFVPROC,
    get_uniformiv: PFNGLGETUNIFORMIVPROC,
    get_vertex_attribdv: PFNGLGETVERTEXATTRIBDVPROC,
    get_vertex_attribfv: PFNGLGETVERTEXATTRIBFVPROC,
    get_vertex_attribiv: PFNGLGETVERTEXATTRIBIVPROC,
    get_vertex_attrib_pointerv: PFNGLGETVERTEXATTRIBPOINTERVPROC,
    is_program: PFNGLISPROGRAMPROC,
    is_shader: PFNGLISSHADERPROC,
    link_program: PFNGLLINKPROGRAMPROC,
    shader_source: PFNGLSHADERSOURCEPROC,
    use_program: PFNGLUSEPROGRAMPROC,
    uniform1f: PFNGLUNIFORM1FPROC,
    uniform2f: PFNGLUNIFORM2FPROC,
    uniform3f: PFNGLUNIFORM3FPROC,
    uniform4f: PFNGLUNIFORM4FPROC,
    uniform1i: PFNGLUNIFORM1IPROC,
    uniform2i: PFNGLUNIFORM2IPROC,
    uniform3i: PFNGLUNIFORM3IPROC,
    uniform4i: PFNGLUNIFORM4IPROC,
    uniform1fv: PFNGLUNIFORM1FVPROC,
    uniform2fv: PFNGLUNIFORM2FVPROC,
    uniform3fv: PFNGLUNIFORM3FVPROC,
    uniform4fv: PFNGLUNIFORM4FVPROC,
    uniform1iv: PFNGLUNIFORM1IVPROC,
    uniform2iv: PFNGLUNIFORM2IVPROC,
    uniform3iv: PFNGLUNIFORM3IVPROC,
    uniform4iv: PFNGLUNIFORM4IVPROC,
    uniform_matrix2fv: PFNGLUNIFORMMATRIX2FVPROC,
    uniform_matrix3fv: PFNGLUNIFORMMATRIX3FVPROC,
    uniform_matrix4fv: PFNGLUNIFORMMATRIX4FVPROC,
    validate_program: PFNGLVALIDATEPROGRAMPROC,
    vertex_attrib1d: PFNGLVERTEXATTRIB1DPROC,
    vertex_attrib1dv: PFNGLVERTEXATTRIB1DVPROC,
    vertex_attrib1f: PFNGLVERTEXATTRIB1FPROC,
    vertex_attrib1fv: PFNGLVERTEXATTRIB1FVPROC,
    vertex_attrib1s: PFNGLVERTEXATTRIB1SPROC,
    vertex_attrib1sv: PFNGLVERTEXATTRIB1SVPROC,
    vertex_attrib2d: PFNGLVERTEXATTRIB2DPROC,
    vertex_attrib2dv: PFNGLVERTEXATTRIB2DVPROC,
    vertex_attrib2f: PFNGLVERTEXATTRIB2FPROC,
    vertex_attrib2fv: PFNGLVERTEXATTRIB2FVPROC,
    vertex_attrib2s: PFNGLVERTEXATTRIB2SPROC,
    vertex_attrib2sv: PFNGLVERTEXATTRIB2SVPROC,
    vertex_attrib3d: PFNGLVERTEXATTRIB3DPROC,
    vertex_attrib3dv: PFNGLVERTEXATTRIB3DVPROC,
    vertex_attrib3f: PFNGLVERTEXATTRIB3FPROC,
    vertex_attrib3fv: PFNGLVERTEXATTRIB3FVPROC,
    vertex_attrib3s: PFNGLVERTEXATTRIB3SPROC,
    vertex_attrib3sv: PFNGLVERTEXATTRIB3SVPROC,
    vertex_attrib_4nbv: PFNGLVERTEXATTRIB4NBVPROC,
    vertex_attrib_4niv: PFNGLVERTEXATTRIB4NIVPROC,
    vertex_attrib_4nsv: PFNGLVERTEXATTRIB4NSVPROC,
    vertex_attrib_4nub: PFNGLVERTEXATTRIB4NUBPROC,
    vertex_attrib_4nubv: PFNGLVERTEXATTRIB4NUBVPROC,
    vertex_attrib_4nuiv: PFNGLVERTEXATTRIB4NUIVPROC,
    vertex_attrib_4nusv: PFNGLVERTEXATTRIB4NUSVPROC,
    vertex_attrib4bv: PFNGLVERTEXATTRIB4BVPROC,
    vertex_attrib4d: PFNGLVERTEXATTRIB4DPROC,
    vertex_attrib4dv: PFNGLVERTEXATTRIB4DVPROC,
    vertex_attrib4f: PFNGLVERTEXATTRIB4FPROC,
    vertex_attrib4fv: PFNGLVERTEXATTRIB4FVPROC,
    vertex_attrib4iv: PFNGLVERTEXATTRIB4IVPROC,
    vertex_attrib4s: PFNGLVERTEXATTRIB4SPROC,
    vertex_attrib4sv: PFNGLVERTEXATTRIB4SVPROC,
    vertex_attrib4ubv: PFNGLVERTEXATTRIB4UBVPROC,
    vertex_attrib4uiv: PFNGLVERTEXATTRIB4UIVPROC,
    vertex_attrib4usv: PFNGLVERTEXATTRIB4USVPROC,
    vertex_attrib_pointer: PFNGLVERTEXATTRIBPOINTERPROC,

    // 2.1
    uniform_matrix2x3fv: PFNGLUNIFORMMATRIX2X3FVPROC,
    uniform_matrix3x2fv: PFNGLUNIFORMMATRIX3X2FVPROC,
    uniform_matrix2x4fv: PFNGLUNIFORMMATRIX2X4FVPROC,
    uniform_matrix4x2fv: PFNGLUNIFORMMATRIX4X2FVPROC,
    uniform_matrix3x4fv: PFNGLUNIFORMMATRIX3X4FVPROC,
    uniform_matrix4x3fv: PFNGLUNIFORMMATRIX4X3FVPROC,

    // 3.0
    color_maski: PFNGLCOLORMASKIPROC,
    get_booleani_v: PFNGLGETBOOLEANI_VPROC,
    get_integeri_v: PFNGLGETINTEGERI_VPROC,
    enablei: PFNGLENABLEIPROC,
    disablei: PFNGLDISABLEIPROC,
    is_enabledi: PFNGLISENABLEDIPROC,
    begin_transform_feedback: PFNGLBEGINTRANSFORMFEEDBACKPROC,
    end_transform_feedback: PFNGLENDTRANSFORMFEEDBACKPROC,
    bind_buffer_range: PFNGLBINDBUFFERRANGEPROC,
    bind_buffer_base: PFNGLBINDBUFFERBASEPROC,
    transform_feedback_varyings: PFNGLTRANSFORMFEEDBACKVARYINGSPROC,
    get_transform_feedback_varying: PFNGLGETTRANSFORMFEEDBACKVARYINGPROC,
    clamp_color: PFNGLCLAMPCOLORPROC,
    begin_conditional_render: PFNGLBEGINCONDITIONALRENDERPROC,
    end_conditional_render: PFNGLENDCONDITIONALRENDERPROC,
    vertex_attrib_ipointer: PFNGLVERTEXATTRIBIPOINTERPROC,
    get_vertex_attrib_iiv: PFNGLGETVERTEXATTRIBIIVPROC,
    get_vertex_attrib_iuiv: PFNGLGETVERTEXATTRIBIUIVPROC,
    vertex_attrib_i1i: PFNGLVERTEXATTRIBI1IPROC,
    vertex_attrib_i2i: PFNGLVERTEXATTRIBI2IPROC,
    vertex_attrib_i3i: PFNGLVERTEXATTRIBI3IPROC,
    vertex_attrib_i4i: PFNGLVERTEXATTRIBI4IPROC,
    vertex_attrib_i1ui: PFNGLVERTEXATTRIBI1UIPROC,
    vertex_attrib_i2ui: PFNGLVERTEXATTRIBI2UIPROC,
    vertex_attrib_i3ui: PFNGLVERTEXATTRIBI3UIPROC,
    vertex_attrib_i4ui: PFNGLVERTEXATTRIBI4UIPROC,
    vertex_attrib_i1iv: PFNGLVERTEXATTRIBI1IVPROC,
    vertex_attrib_i2iv: PFNGLVERTEXATTRIBI2IVPROC,
    vertex_attrib_i3iv: PFNGLVERTEXATTRIBI3IVPROC,
    vertex_attrib_i4iv: PFNGLVERTEXATTRIBI4IVPROC,
    vertex_attrib_i1uiv: PFNGLVERTEXATTRIBI1UIVPROC,
    vertex_attrib_i2uiv: PFNGLVERTEXATTRIBI2UIVPROC,
    vertex_attrib_i3uiv: PFNGLVERTEXATTRIBI3UIVPROC,
    vertex_attrib_i4uiv: PFNGLVERTEXATTRIBI4UIVPROC,
    vertex_attrib_i4bv: PFNGLVERTEXATTRIBI4BVPROC,
    vertex_attrib_i4sv: PFNGLVERTEXATTRIBI4SVPROC,
    vertex_attrib_i4ubv: PFNGLVERTEXATTRIBI4UBVPROC,
    vertex_attrib_i4usv: PFNGLVERTEXATTRIBI4USVPROC,
    get_uniformuiv: PFNGLGETUNIFORMUIVPROC,
    bind_frag_data_location: PFNGLBINDFRAGDATALOCATIONPROC,
    get_frag_data_location: PFNGLGETFRAGDATALOCATIONPROC,
    uniform1ui: PFNGLUNIFORM1UIPROC,
    uniform2ui: PFNGLUNIFORM2UIPROC,
    uniform3ui: PFNGLUNIFORM3UIPROC,
    uniform4ui: PFNGLUNIFORM4UIPROC,
    uniform1uiv: PFNGLUNIFORM1UIVPROC,
    uniform2uiv: PFNGLUNIFORM2UIVPROC,
    uniform3uiv: PFNGLUNIFORM3UIVPROC,
    uniform4uiv: PFNGLUNIFORM4UIVPROC,
    tex_parameter_iiv: PFNGLTEXPARAMETERIIVPROC,
    tex_parameter_iuiv: PFNGLTEXPARAMETERIUIVPROC,
    get_tex_parameter_iiv: PFNGLGETTEXPARAMETERIIVPROC,
    get_tex_parameter_iuiv: PFNGLGETTEXPARAMETERIUIVPROC,
    clear_bufferiv: PFNGLCLEARBUFFERIVPROC,
    clear_bufferuiv: PFNGLCLEARBUFFERUIVPROC,
    clear_bufferfv: PFNGLCLEARBUFFERFVPROC,
    clear_bufferfi: PFNGLCLEARBUFFERFIPROC,
    get_stringi: PFNGLGETSTRINGIPROC,
    is_renderbuffer: PFNGLISRENDERBUFFERPROC,
    bind_renderbuffer: PFNGLBINDRENDERBUFFERPROC,
    delete_renderbuffers: PFNGLDELETERENDERBUFFERSPROC,
    gen_renderbuffers: PFNGLGENRENDERBUFFERSPROC,
    renderbuffer_storage: PFNGLRENDERBUFFERSTORAGEPROC,
    get_renderbuffer_parameteriv: PFNGLGETRENDERBUFFERPARAMETERIVPROC,
    is_framebuffer: PFNGLISFRAMEBUFFERPROC,
    bind_framebuffer: PFNGLBINDFRAMEBUFFERPROC,
    delete_framebuffers: PFNGLDELETEFRAMEBUFFERSPROC,
    gen_framebuffers: PFNGLGENFRAMEBUFFERSPROC,
    check_framebuffer_status: PFNGLCHECKFRAMEBUFFERSTATUSPROC,
    framebuffer_texture_1d: PFNGLFRAMEBUFFERTEXTURE1DPROC,
    framebuffer_texture_2d: PFNGLFRAMEBUFFERTEXTURE2DPROC,
    framebuffer_texture_3d: PFNGLFRAMEBUFFERTEXTURE3DPROC,
    framebuffer_renderbuffer: PFNGLFRAMEBUFFERRENDERBUFFERPROC,
    get_framebuffer_attachment_parameteriv: PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC,
    generate_mipmap: PFNGLGENERATEMIPMAPPROC,
    blit_framebuffer: PFNGLBLITFRAMEBUFFERPROC,
    renderbuffer_storage_multisample: PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC,
    framebuffer_texture_layer: PFNGLFRAMEBUFFERTEXTURELAYERPROC,
    map_buffer_range: PFNGLMAPBUFFERRANGEPROC,
    flush_mapped_buffer_range: PFNGLFLUSHMAPPEDBUFFERRANGEPROC,
    bind_vertex_array: PFNGLBINDVERTEXARRAYPROC,
    delete_vertex_arrays: PFNGLDELETEVERTEXARRAYSPROC,
    gen_vertex_arrays: PFNGLGENVERTEXARRAYSPROC,
    is_vertex_array: PFNGLISVERTEXARRAYPROC,

    // 3.1
    draw_arrays_instanced: PFNGLDRAWARRAYSINSTANCEDPROC,
    draw_elements_instanced: PFNGLDRAWELEMENTSINSTANCEDPROC,
    tex_buffer: PFNGLTEXBUFFERPROC,
    primitive_restart_index: PFNGLPRIMITIVERESTARTINDEXPROC,
    copy_buffer_sub_data: PFNGLCOPYBUFFERSUBDATAPROC,
    get_uniform_indices: PFNGLGETUNIFORMINDICESPROC,
    get_active_uniformsiv: PFNGLGETACTIVEUNIFORMSIVPROC,
    get_active_uniform_name: PFNGLGETACTIVEUNIFORMNAMEPROC,
    get_uniform_block_index: PFNGLGETUNIFORMBLOCKINDEXPROC,
    get_active_uniform_blockiv: PFNGLGETACTIVEUNIFORMBLOCKIVPROC,
    get_active_uniform_block_name: PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC,
    uniform_block_binding: PFNGLUNIFORMBLOCKBINDINGPROC,

    // 3.2
    draw_elements_base_vertex: PFNGLDRAWELEMENTSBASEVERTEXPROC,
    draw_range_elements_base_vertex: PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC,
    draw_elements_instanced_base_vertex: PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC,
    multi_draw_elements_base_vertex: PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC,
    provoking_vertex: PFNGLPROVOKINGVERTEXPROC,
    fence_sync: PFNGLFENCESYNCPROC,
    is_sync: PFNGLISSYNCPROC,
    delete_sync: PFNGLDELETESYNCPROC,
    client_wait_sync: PFNGLCLIENTWAITSYNCPROC,
    wait_sync: PFNGLWAITSYNCPROC,
    get_integer64v: PFNGLGETINTEGER64VPROC,
    get_synciv: PFNGLGETSYNCIVPROC,
    get_integer64i_v: PFNGLGETINTEGER64I_VPROC,
    get_buffer_parameteri64v: PFNGLGETBUFFERPARAMETERI64VPROC,
    framebuffer_texture: PFNGLFRAMEBUFFERTEXTUREPROC,
    tex_image_2dmultisample: PFNGLTEXIMAGE2DMULTISAMPLEPROC,
    tex_image_3dmultisample: PFNGLTEXIMAGE3DMULTISAMPLEPROC,
    get_multisamplefv: PFNGLGETMULTISAMPLEFVPROC,
    sample_maski: PFNGLSAMPLEMASKIPROC,

    // 3.3
    bind_frag_data_location_indexed: PFNGLBINDFRAGDATALOCATIONINDEXEDPROC,
    get_frag_data_index: PFNGLGETFRAGDATAINDEXPROC,
    gen_samplers: PFNGLGENSAMPLERSPROC,
    delete_samplers: PFNGLDELETESAMPLERSPROC,
    is_sampler: PFNGLISSAMPLERPROC,
    bind_sampler: PFNGLBINDSAMPLERPROC,
    sampler_parameteri: PFNGLSAMPLERPARAMETERIPROC,
    sampler_parameteriv: PFNGLSAMPLERPARAMETERIVPROC,
    sampler_parameterf: PFNGLSAMPLERPARAMETERFPROC,
    sampler_parameterfv: PFNGLSAMPLERPARAMETERFVPROC,
    sampler_parameter_iiv: PFNGLSAMPLERPARAMETERIIVPROC,
    sampler_parameter_iuiv: PFNGLSAMPLERPARAMETERIUIVPROC,
    get_sampler_parameteriv: PFNGLGETSAMPLERPARAMETERIVPROC,
    get_sampler_parameter_iiv: PFNGLGETSAMPLERPARAMETERIIVPROC,
    get_sampler_parameterfv: PFNGLGETSAMPLERPARAMETERFVPROC,
    get_sampler_parameter_iuiv: PFNGLGETSAMPLERPARAMETERIUIVPROC,
    query_counter: PFNGLQUERYCOUNTERPROC,
    get_query_objecti64v: PFNGLGETQUERYOBJECTI64VPROC,
    get_query_objectui64v: PFNGLGETQUERYOBJECTUI64VPROC,
    vertex_attrib_divisor: PFNGLVERTEXATTRIBDIVISORPROC,
    vertex_attrib_p1ui: PFNGLVERTEXATTRIBP1UIPROC,
    vertex_attrib_p1uiv: PFNGLVERTEXATTRIBP1UIVPROC,
    vertex_attrib_p2ui: PFNGLVERTEXATTRIBP2UIPROC,
    vertex_attrib_p2uiv: PFNGLVERTEXATTRIBP2UIVPROC,
    vertex_attrib_p3ui: PFNGLVERTEXATTRIBP3UIPROC,
    vertex_attrib_p3uiv: PFNGLVERTEXATTRIBP3UIVPROC,
    vertex_attrib_p4ui: PFNGLVERTEXATTRIBP4UIPROC,
    vertex_attrib_p4uiv: PFNGLVERTEXATTRIBP4UIVPROC,
}

impl Gl {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            cull_face: unsafe {
                unsafe extern "system" fn cull_face(_mode: GLenum) {
                    panic!("Unable to load cull_face")
                }
                let val = f(c"glCullFace");
                if val.is_null() {
                    cull_face
                } else {
                    mem::transmute(val)
                }
            },
            front_face: unsafe {
                unsafe extern "system" fn front_face(_mode: GLenum) {
                    panic!("Unable to load front_face")
                }
                let val = f(c"glFrontFace");
                if val.is_null() {
                    front_face
                } else {
                    mem::transmute(val)
                }
            },
            hint: unsafe {
                unsafe extern "system" fn hint(_target: GLenum, _mode: GLenum) {
                    panic!("Unable to load hint")
                }
                let val = f(c"glHint");
                if val.is_null() {
                    hint
                } else {
                    mem::transmute(val)
                }
            },
            line_width: unsafe {
                unsafe extern "system" fn line_width(_width: GLfloat) {
                    panic!("Unable to load line_width")
                }
                let val = f(c"glLineWidth");
                if val.is_null() {
                    line_width
                } else {
                    mem::transmute(val)
                }
            },
            point_size: unsafe {
                unsafe extern "system" fn point_size(_size: GLfloat) {
                    panic!("Unable to load point_size")
                }
                let val = f(c"glPointSize");
                if val.is_null() {
                    point_size
                } else {
                    mem::transmute(val)
                }
            },
            polygon_mode: unsafe {
                unsafe extern "system" fn polygon_mode(_face: GLenum, _mode: GLenum) {
                    panic!("Unable to load polygon_mode")
                }
                let val = f(c"glPolygonMode");
                if val.is_null() {
                    polygon_mode
                } else {
                    mem::transmute(val)
                }
            },
            scissor: unsafe {
                unsafe extern "system" fn scissor(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load scissor")
                }
                let val = f(c"glScissor");
                if val.is_null() {
                    scissor
                } else {
                    mem::transmute(val)
                }
            },
            tex_parameterf: unsafe {
                unsafe extern "system" fn tex_parameterf(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load tex_parameterf")
                }
                let val = f(c"glTexParameterf");
                if val.is_null() {
                    tex_parameterf
                } else {
                    mem::transmute(val)
                }
            },
            tex_parameterfv: unsafe {
                unsafe extern "system" fn tex_parameterfv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load tex_parameterfv")
                }
                let val = f(c"glTexParameterfv");
                if val.is_null() {
                    tex_parameterfv
                } else {
                    mem::transmute(val)
                }
            },
            tex_parameteri: unsafe {
                unsafe extern "system" fn tex_parameteri(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load tex_parameteri")
                }
                let val = f(c"glTexParameteri");
                if val.is_null() {
                    tex_parameteri
                } else {
                    mem::transmute(val)
                }
            },
            tex_parameteriv: unsafe {
                unsafe extern "system" fn tex_parameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLint,
                ) {
                    panic!("Unable to load tex_parameteriv")
                }
                let val = f(c"glTexParameteriv");
                if val.is_null() {
                    tex_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            tex_image_1d: unsafe {
                unsafe extern "system" fn tex_image_1d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLint,
                    _width: GLsizei,
                    _border: GLint,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const c_void,
                ) {
                    panic!("Unable to load tex_image_1d")
                }
                let val = f(c"glTexImage1D");
                if val.is_null() {
                    tex_image_1d
                } else {
                    mem::transmute(val)
                }
            },
            tex_image_2d: unsafe {
                unsafe extern "system" fn tex_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const c_void,
                ) {
                    panic!("Unable to load tex_image_2d")
                }
                let val = f(c"glTexImage2D");
                if val.is_null() {
                    tex_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            draw_buffer: unsafe {
                unsafe extern "system" fn draw_buffer(_buf: GLenum) {
                    panic!("Unable to load draw_buffer")
                }
                let val = f(c"glDrawBuffer");
                if val.is_null() {
                    draw_buffer
                } else {
                    mem::transmute(val)
                }
            },
            clear: unsafe {
                unsafe extern "system" fn clear(_mask: GLbitfield) {
                    panic!("Unable to load clear")
                }
                let val = f(c"glClear");
                if val.is_null() {
                    clear
                } else {
                    mem::transmute(val)
                }
            },
            clear_color: unsafe {
                unsafe extern "system" fn clear_color(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load clear_color")
                }
                let val = f(c"glClearColor");
                if val.is_null() {
                    clear_color
                } else {
                    mem::transmute(val)
                }
            },
            clear_stencil: unsafe {
                unsafe extern "system" fn clear_stencil(_s: GLint) {
                    panic!("Unable to load clear_stencil")
                }
                let val = f(c"glClearStencil");
                if val.is_null() {
                    clear_stencil
                } else {
                    mem::transmute(val)
                }
            },
            clear_depth: unsafe {
                unsafe extern "system" fn clear_depth(_depth: GLdouble) {
                    panic!("Unable to load clear_depth")
                }
                let val = f(c"glClearDepth");
                if val.is_null() {
                    clear_depth
                } else {
                    mem::transmute(val)
                }
            },
            stencil_mask: unsafe {
                unsafe extern "system" fn stencil_mask(_mask: GLuint) {
                    panic!("Unable to load stencil_mask")
                }
                let val = f(c"glStencilMask");
                if val.is_null() {
                    stencil_mask
                } else {
                    mem::transmute(val)
                }
            },
            color_mask: unsafe {
                unsafe extern "system" fn color_mask(
                    _red: GLboolean,
                    _green: GLboolean,
                    _blue: GLboolean,
                    _alpha: GLboolean,
                ) {
                    panic!("Unable to load color_mask")
                }
                let val = f(c"glColorMask");
                if val.is_null() {
                    color_mask
                } else {
                    mem::transmute(val)
                }
            },
            depth_mask: unsafe {
                unsafe extern "system" fn depth_mask(_flag: GLboolean) {
                    panic!("Unable to load depth_mask")
                }
                let val = f(c"glDepthMask");
                if val.is_null() {
                    depth_mask
                } else {
                    mem::transmute(val)
                }
            },
            disable: unsafe {
                unsafe extern "system" fn disable(_cap: GLenum) {
                    panic!("Unable to load disable")
                }
                let val = f(c"glDisable");
                if val.is_null() {
                    disable
                } else {
                    mem::transmute(val)
                }
            },
            enable: unsafe {
                unsafe extern "system" fn enable(_cap: GLenum) {
                    panic!("Unable to load enable")
                }
                let val = f(c"glEnable");
                if val.is_null() {
                    enable
                } else {
                    mem::transmute(val)
                }
            },
            finish: unsafe {
                unsafe extern "system" fn finish() {
                    panic!("Unable to load finish")
                }
                let val = f(c"glFinish");
                if val.is_null() {
                    finish
                } else {
                    mem::transmute(val)
                }
            },
            flush: unsafe {
                unsafe extern "system" fn flush() {
                    panic!("Unable to load flush")
                }
                let val = f(c"glFlush");
                if val.is_null() {
                    flush
                } else {
                    mem::transmute(val)
                }
            },
            blend_func: unsafe {
                unsafe extern "system" fn blend_func(_sfactor: GLenum, _dfactor: GLenum) {
                    panic!("Unable to load blend_func")
                }
                let val = f(c"glBlendFunc");
                if val.is_null() {
                    blend_func
                } else {
                    mem::transmute(val)
                }
            },
            logic_op: unsafe {
                unsafe extern "system" fn logic_op(_opcode: GLenum) {
                    panic!("Unable to load logic_op")
                }
                let val = f(c"glLogicOp");
                if val.is_null() {
                    logic_op
                } else {
                    mem::transmute(val)
                }
            },
            stencil_func: unsafe {
                unsafe extern "system" fn stencil_func(_func: GLenum, _ref: GLint, _mask: GLuint) {
                    panic!("Unable to load stencil_func")
                }
                let val = f(c"glStencilFunc");
                if val.is_null() {
                    stencil_func
                } else {
                    mem::transmute(val)
                }
            },
            stencil_op: unsafe {
                unsafe extern "system" fn stencil_op(
                    _fail: GLenum,
                    _zfail: GLenum,
                    _zpass: GLenum,
                ) {
                    panic!("Unable to load stencil_op")
                }
                let val = f(c"glStencilOp");
                if val.is_null() {
                    stencil_op
                } else {
                    mem::transmute(val)
                }
            },
            depth_func: unsafe {
                unsafe extern "system" fn depth_func(_func: GLenum) {
                    panic!("Unable to load depth_func")
                }
                let val = f(c"glDepthFunc");
                if val.is_null() {
                    depth_func
                } else {
                    mem::transmute(val)
                }
            },
            pixel_storef: unsafe {
                unsafe extern "system" fn pixel_storef(_pname: GLenum, _param: GLfloat) {
                    panic!("Unable to load pixel_storef")
                }
                let val = f(c"glPixelStoref");
                if val.is_null() {
                    pixel_storef
                } else {
                    mem::transmute(val)
                }
            },
            pixel_storei: unsafe {
                unsafe extern "system" fn pixel_storei(_pname: GLenum, _param: GLint) {
                    panic!("Unable to load pixel_storei")
                }
                let val = f(c"glPixelStorei");
                if val.is_null() {
                    pixel_storei
                } else {
                    mem::transmute(val)
                }
            },
            read_buffer: unsafe {
                unsafe extern "system" fn read_buffer(_src: GLenum) {
                    panic!("Unable to load read_buffer")
                }
                let val = f(c"glReadBuffer");
                if val.is_null() {
                    read_buffer
                } else {
                    mem::transmute(val)
                }
            },
            read_pixels: unsafe {
                unsafe extern "system" fn read_pixels(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *mut c_void,
                ) {
                    panic!("Unable to load read_pixels")
                }
                let val = f(c"glReadPixels");
                if val.is_null() {
                    read_pixels
                } else {
                    mem::transmute(val)
                }
            },
            get_booleanv: unsafe {
                unsafe extern "system" fn get_booleanv(_pname: GLenum, _data: *mut GLboolean) {
                    panic!("Unable to load get_booleanv")
                }
                let val = f(c"glGetBooleanv");
                if val.is_null() {
                    get_booleanv
                } else {
                    mem::transmute(val)
                }
            },
            get_doublev: unsafe {
                unsafe extern "system" fn get_doublev(_pname: GLenum, _data: *mut GLdouble) {
                    panic!("Unable to load get_doublev")
                }
                let val = f(c"glGetDoublev");
                if val.is_null() {
                    get_doublev
                } else {
                    mem::transmute(val)
                }
            },
            get_error: unsafe {
                unsafe extern "system" fn get_error() -> GLenum {
                    panic!("Unable to load get_error")
                }
                let val = f(c"glGetError");
                if val.is_null() {
                    get_error
                } else {
                    mem::transmute(val)
                }
            },
            get_floatv: unsafe {
                unsafe extern "system" fn get_floatv(_pname: GLenum, _data: *mut GLfloat) {
                    panic!("Unable to load get_floatv")
                }
                let val = f(c"glGetFloatv");
                if val.is_null() {
                    get_floatv
                } else {
                    mem::transmute(val)
                }
            },
            get_integerv: unsafe {
                unsafe extern "system" fn get_integerv(_pname: GLenum, _data: *mut GLint) {
                    panic!("Unable to load get_integerv")
                }
                let val = f(c"glGetIntegerv");
                if val.is_null() {
                    get_integerv
                } else {
                    mem::transmute(val)
                }
            },
            get_string: unsafe {
                unsafe extern "system" fn get_string(_name: GLenum) -> *const GLubyte {
                    panic!("Unable to load get_string")
                }
                let val = f(c"glGetString");
                if val.is_null() {
                    get_string
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_image: unsafe {
                unsafe extern "system" fn get_tex_image(
                    _target: GLenum,
                    _level: GLint,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *mut c_void,
                ) {
                    panic!("Unable to load get_tex_image")
                }
                let val = f(c"glGetTexImage");
                if val.is_null() {
                    get_tex_image
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_parameterfv: unsafe {
                unsafe extern "system" fn get_tex_parameterfv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load get_tex_parameterfv")
                }
                let val = f(c"glGetTexParameterfv");
                if val.is_null() {
                    get_tex_parameterfv
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_parameteriv: unsafe {
                unsafe extern "system" fn get_tex_parameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_tex_parameteriv")
                }
                let val = f(c"glGetTexParameteriv");
                if val.is_null() {
                    get_tex_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_level_parameterfv: unsafe {
                unsafe extern "system" fn get_tex_level_parameterfv(
                    _target: GLenum,
                    _level: GLint,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load get_tex_level_parameterfv")
                }
                let val = f(c"glGetTexLevelParameterfv");
                if val.is_null() {
                    get_tex_level_parameterfv
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_level_parameteriv: unsafe {
                unsafe extern "system" fn get_tex_level_parameteriv(
                    _target: GLenum,
                    _level: GLint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_tex_level_parameteriv")
                }
                let val = f(c"glGetTexLevelParameteriv");
                if val.is_null() {
                    get_tex_level_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            is_enabled: unsafe {
                unsafe extern "system" fn is_enabled(_cap: GLenum) -> GLboolean {
                    panic!("Unable to load is_enabled")
                }
                let val = f(c"glIsEnabled");
                if val.is_null() {
                    is_enabled
                } else {
                    mem::transmute(val)
                }
            },
            depth_range: unsafe {
                unsafe extern "system" fn depth_range(_n: GLdouble, _f: GLdouble) {
                    panic!("Unable to load depth_range")
                }
                let val = f(c"glDepthRange");
                if val.is_null() {
                    depth_range
                } else {
                    mem::transmute(val)
                }
            },
            viewport: unsafe {
                unsafe extern "system" fn viewport(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load viewport")
                }
                let val = f(c"glViewport");
                if val.is_null() {
                    viewport
                } else {
                    mem::transmute(val)
                }
            },
            draw_arrays: unsafe {
                unsafe extern "system" fn draw_arrays(
                    _mode: GLenum,
                    _first: GLint,
                    _count: GLsizei,
                ) {
                    panic!("Unable to load draw_arrays")
                }
                let val = f(c"glDrawArrays");
                if val.is_null() {
                    draw_arrays
                } else {
                    mem::transmute(val)
                }
            },
            draw_elements: unsafe {
                unsafe extern "system" fn draw_elements(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const c_void,
                ) {
                    panic!("Unable to load draw_elements")
                }
                let val = f(c"glDrawElements");
                if val.is_null() {
                    draw_elements
                } else {
                    mem::transmute(val)
                }
            },
            get_pointerv: unsafe {
                unsafe extern "system" fn get_pointerv(_pname: GLenum, _params: *mut *mut c_void) {
                    panic!("Unable to load get_pointerv")
                }
                let val = f(c"glGetPointerv");
                if val.is_null() {
                    get_pointerv
                } else {
                    mem::transmute(val)
                }
            },
            polygon_offset: unsafe {
                unsafe extern "system" fn polygon_offset(_factor: GLfloat, _units: GLfloat) {
                    panic!("Unable to load polygon_offset")
                }
                let val = f(c"glPolygonOffset");
                if val.is_null() {
                    polygon_offset
                } else {
                    mem::transmute(val)
                }
            },
            copy_tex_image_1d: unsafe {
                unsafe extern "system" fn copy_tex_image_1d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _border: GLint,
                ) {
                    panic!("Unable to load copy_tex_image_1d")
                }
                let val = f(c"glCopyTexImage1D");
                if val.is_null() {
                    copy_tex_image_1d
                } else {
                    mem::transmute(val)
                }
            },
            copy_tex_image_2d: unsafe {
                unsafe extern "system" fn copy_tex_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                ) {
                    panic!("Unable to load copy_tex_image_2d")
                }
                let val = f(c"glCopyTexImage2D");
                if val.is_null() {
                    copy_tex_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            copy_tex_sub_image_1d: unsafe {
                unsafe extern "system" fn copy_tex_sub_image_1d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                ) {
                    panic!("Unable to load copy_tex_sub_image_1d")
                }
                let val = f(c"glCopyTexSubImage1D");
                if val.is_null() {
                    copy_tex_sub_image_1d
                } else {
                    mem::transmute(val)
                }
            },
            copy_tex_sub_image_2d: unsafe {
                unsafe extern "system" fn copy_tex_sub_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load copy_tex_sub_image_2d")
                }
                let val = f(c"glCopyTexSubImage2D");
                if val.is_null() {
                    copy_tex_sub_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            tex_sub_image_1d: unsafe {
                unsafe extern "system" fn tex_sub_image_1d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _width: GLsizei,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const c_void,
                ) {
                    panic!("Unable to load tex_sub_image_1d")
                }
                let val = f(c"glTexSubImage1D");
                if val.is_null() {
                    tex_sub_image_1d
                } else {
                    mem::transmute(val)
                }
            },
            tex_sub_image_2d: unsafe {
                unsafe extern "system" fn tex_sub_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const c_void,
                ) {
                    panic!("Unable to load tex_sub_image_2d")
                }
                let val = f(c"glTexSubImage2D");
                if val.is_null() {
                    tex_sub_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            bind_texture: unsafe {
                unsafe extern "system" fn bind_texture(_target: GLenum, _texture: GLuint) {
                    panic!("Unable to load bind_texture")
                }
                let val = f(c"glBindTexture");
                if val.is_null() {
                    bind_texture
                } else {
                    mem::transmute(val)
                }
            },
            delete_textures: unsafe {
                unsafe extern "system" fn delete_textures(_n: GLsizei, _textures: *const GLuint) {
                    panic!("Unable to load delete_textures")
                }
                let val = f(c"glDeleteTextures");
                if val.is_null() {
                    delete_textures
                } else {
                    mem::transmute(val)
                }
            },
            gen_textures: unsafe {
                unsafe extern "system" fn gen_textures(_n: GLsizei, _textures: *mut GLuint) {
                    panic!("Unable to load gen_textures")
                }
                let val = f(c"glGenTextures");
                if val.is_null() {
                    gen_textures
                } else {
                    mem::transmute(val)
                }
            },
            is_texture: unsafe {
                unsafe extern "system" fn is_texture(_texture: GLuint) -> GLboolean {
                    panic!("Unable to load is_texture")
                }
                let val = f(c"glIsTexture");
                if val.is_null() {
                    is_texture
                } else {
                    mem::transmute(val)
                }
            },
            draw_range_elements: unsafe {
                unsafe extern "system" fn draw_range_elements(
                    _mode: GLenum,
                    _start: GLuint,
                    _end: GLuint,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const c_void,
                ) {
                    panic!("Unable to load draw_range_elements")
                }
                let val = f(c"glDrawRangeElements");
                if val.is_null() {
                    draw_range_elements
                } else {
                    mem::transmute(val)
                }
            },
            tex_image_3d: unsafe {
                unsafe extern "system" fn tex_image_3d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _border: GLint,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const c_void,
                ) {
                    panic!("Unable to load tex_image_3d")
                }
                let val = f(c"glTexImage3D");
                if val.is_null() {
                    tex_image_3d
                } else {
                    mem::transmute(val)
                }
            },
            tex_sub_image_3d: unsafe {
                unsafe extern "system" fn tex_sub_image_3d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const c_void,
                ) {
                    panic!("Unable to load tex_sub_image_3d")
                }
                let val = f(c"glTexSubImage3D");
                if val.is_null() {
                    tex_sub_image_3d
                } else {
                    mem::transmute(val)
                }
            },
            copy_tex_sub_image_3d: unsafe {
                unsafe extern "system" fn copy_tex_sub_image_3d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load copy_tex_sub_image_3d")
                }
                let val = f(c"glCopyTexSubImage3D");
                if val.is_null() {
                    copy_tex_sub_image_3d
                } else {
                    mem::transmute(val)
                }
            },
            active_texture: unsafe {
                unsafe extern "system" fn active_texture(_texture: GLenum) {
                    panic!("Unable to load active_texture")
                }
                let val = f(c"glActiveTexture");
                if val.is_null() {
                    active_texture
                } else {
                    mem::transmute(val)
                }
            },
            sample_coverage: unsafe {
                unsafe extern "system" fn sample_coverage(_value: GLfloat, _invert: GLboolean) {
                    panic!("Unable to load sample_coverage")
                }
                let val = f(c"glSampleCoverage");
                if val.is_null() {
                    sample_coverage
                } else {
                    mem::transmute(val)
                }
            },
            compressed_tex_image_3d: unsafe {
                unsafe extern "system" fn compressed_tex_image_3d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _border: GLint,
                    _image_size: GLsizei,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load compressed_tex_image_3d")
                }
                let val = f(c"glCompressedTexImage3D");
                if val.is_null() {
                    compressed_tex_image_3d
                } else {
                    mem::transmute(val)
                }
            },
            compressed_tex_image_2d: unsafe {
                unsafe extern "system" fn compressed_tex_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                    _image_size: GLsizei,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load compressed_tex_image_2d")
                }
                let val = f(c"glCompressedTexImage2D");
                if val.is_null() {
                    compressed_tex_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            compressed_tex_image_1d: unsafe {
                unsafe extern "system" fn compressed_tex_image_1d(
                    _target: GLenum,
                    _level: GLint,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _border: GLint,
                    _image_size: GLsizei,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load compressed_tex_image_1d")
                }
                let val = f(c"glCompressedTexImage1D");
                if val.is_null() {
                    compressed_tex_image_1d
                } else {
                    mem::transmute(val)
                }
            },
            compressed_tex_sub_image_3d: unsafe {
                unsafe extern "system" fn compressed_tex_sub_image_3d(
                    _target: GLenum,
                    _level: GLint,
                    __xoffset: GLint,
                    _yoffset: GLint,
                    _zoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _format: GLenum,
                    _image_size: GLsizei,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load compressed_tex_sub_image_3d")
                }
                let val = f(c"glCompressedTexSubImage3D");
                if val.is_null() {
                    compressed_tex_sub_image_3d
                } else {
                    mem::transmute(val)
                }
            },
            compressed_tex_sub_image_2d: unsafe {
                unsafe extern "system" fn compressed_tex_sub_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _yoffset: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _format: GLenum,
                    _image_size: GLsizei,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load compressed_tex_sub_image_2d")
                }
                let val = f(c"glCompressedTexSubImage2D");
                if val.is_null() {
                    compressed_tex_sub_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            compressed_tex_sub_image_1d: unsafe {
                unsafe extern "system" fn compressed_tex_sub_image_1d(
                    _target: GLenum,
                    _level: GLint,
                    _xoffset: GLint,
                    _width: GLsizei,
                    _format: GLenum,
                    _image_size: GLsizei,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load compressed_tex_sub_image_1d")
                }
                let val = f(c"glCompressedTexSubImage1D");
                if val.is_null() {
                    compressed_tex_sub_image_1d
                } else {
                    mem::transmute(val)
                }
            },
            get_compressed_tex_image: unsafe {
                unsafe extern "system" fn get_compressed_tex_image(
                    _target: GLenum,
                    _level: GLint,
                    _img: *mut c_void,
                ) {
                    panic!("Unable to load get_compressed_tex_image")
                }
                let val = f(c"glGetCompressedTexImage");
                if val.is_null() {
                    get_compressed_tex_image
                } else {
                    mem::transmute(val)
                }
            },
            blend_func_separate: unsafe {
                unsafe extern "system" fn blend_func_separate(
                    _sfactor_rgb: GLenum,
                    _dfactor_rgb: GLenum,
                    _sfactor_alpha: GLenum,
                    _dfactor_alpha: GLenum,
                ) {
                    panic!("Unable to load blend_func_separate")
                }
                let val = f(c"glBlendFuncSeparate");
                if val.is_null() {
                    blend_func_separate
                } else {
                    mem::transmute(val)
                }
            },
            multi_draw_arrays: unsafe {
                unsafe extern "system" fn multi_draw_arrays(
                    _mode: GLenum,
                    _first: *const GLint,
                    _count: *const GLsizei,
                    _drawcount: GLsizei,
                ) {
                    panic!("Unable to load multi_draw_arrays")
                }
                let val = f(c"glMultiDrawArrays");
                if val.is_null() {
                    multi_draw_arrays
                } else {
                    mem::transmute(val)
                }
            },
            multi_draw_elements: unsafe {
                unsafe extern "system" fn multi_draw_elements(
                    _mode: GLenum,
                    _count: *const GLsizei,
                    _type: GLenum,
                    _indices: *const *const c_void,
                    _drawcount: GLsizei,
                ) {
                    panic!("Unable to load multi_draw_elements")
                }
                let val = f(c"glMultiDrawElements");
                if val.is_null() {
                    multi_draw_elements
                } else {
                    mem::transmute(val)
                }
            },
            point_parameterf: unsafe {
                unsafe extern "system" fn point_parameterf(_pname: GLenum, _param: GLfloat) {
                    panic!("Unable to load point_parameterf")
                }
                let val = f(c"glPointParameterf");
                if val.is_null() {
                    point_parameterf
                } else {
                    mem::transmute(val)
                }
            },
            point_parameterfv: unsafe {
                unsafe extern "system" fn point_parameterfv(
                    _pname: GLenum,
                    _params: *const GLfloat,
                ) {
                    panic!("Unable to load point_parameterfv")
                }
                let val = f(c"glPointParameterfv");
                if val.is_null() {
                    point_parameterfv
                } else {
                    mem::transmute(val)
                }
            },
            point_parameteri: unsafe {
                unsafe extern "system" fn point_parameteri(_pname: GLenum, _param: GLint) {
                    panic!("Unable to load point_parameteri")
                }
                let val = f(c"glPointParameteri");
                if val.is_null() {
                    point_parameteri
                } else {
                    mem::transmute(val)
                }
            },
            point_parameteriv: unsafe {
                unsafe extern "system" fn point_parameteriv(_pname: GLenum, _params: *const GLint) {
                    panic!("Unable to load point_parameteriv")
                }
                let val = f(c"glPointParameteriv");
                if val.is_null() {
                    point_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            blend_color: unsafe {
                unsafe extern "system" fn blend_color(
                    _red: GLfloat,
                    _green: GLfloat,
                    _blue: GLfloat,
                    _alpha: GLfloat,
                ) {
                    panic!("Unable to load blend_color")
                }
                let val = f(c"glBlendColor");
                if val.is_null() {
                    blend_color
                } else {
                    mem::transmute(val)
                }
            },
            blend_equation: unsafe {
                unsafe extern "system" fn blend_equation(_mode: GLenum) {
                    panic!("Unable to load blend_equation")
                }
                let val = f(c"glBlendEquation");
                if val.is_null() {
                    blend_equation
                } else {
                    mem::transmute(val)
                }
            },
            gen_queries: unsafe {
                unsafe extern "system" fn gen_queries(_n: GLsizei, _ids: *mut GLuint) {
                    panic!("Unable to load gen_queries")
                }
                let val = f(c"glGenQueries");
                if val.is_null() {
                    gen_queries
                } else {
                    mem::transmute(val)
                }
            },
            delete_queries: unsafe {
                unsafe extern "system" fn delete_queries(_n: GLsizei, _ids: *const GLuint) {
                    panic!("Unable to load delete_queries")
                }
                let val = f(c"glDeleteQueries");
                if val.is_null() {
                    delete_queries
                } else {
                    mem::transmute(val)
                }
            },
            is_query: unsafe {
                unsafe extern "system" fn is_query(_id: GLuint) -> GLboolean {
                    panic!("Unable to load is_query")
                }
                let val = f(c"glIsQuery");
                if val.is_null() {
                    is_query
                } else {
                    mem::transmute(val)
                }
            },
            begin_query: unsafe {
                unsafe extern "system" fn begin_query(_target: GLenum, _id: GLuint) {
                    panic!("Unable to load begin_query")
                }
                let val = f(c"glBeginQuery");
                if val.is_null() {
                    begin_query
                } else {
                    mem::transmute(val)
                }
            },
            end_query: unsafe {
                unsafe extern "system" fn end_query(_target: GLenum) {
                    panic!("Unable to load end_query")
                }
                let val = f(c"glEndQuery");
                if val.is_null() {
                    end_query
                } else {
                    mem::transmute(val)
                }
            },
            get_queryiv: unsafe {
                unsafe extern "system" fn get_queryiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_queryiv")
                }
                let val = f(c"glGetQueryiv");
                if val.is_null() {
                    get_queryiv
                } else {
                    mem::transmute(val)
                }
            },
            get_query_objectiv: unsafe {
                unsafe extern "system" fn get_query_objectiv(
                    _id: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_query_objectiv")
                }
                let val = f(c"glGetQueryObjectiv");
                if val.is_null() {
                    get_query_objectiv
                } else {
                    mem::transmute(val)
                }
            },
            get_query_objectuiv: unsafe {
                unsafe extern "system" fn get_query_objectuiv(
                    _id: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load get_query_objectuiv")
                }
                let val = f(c"glGetQueryObjectuiv");
                if val.is_null() {
                    get_query_objectuiv
                } else {
                    mem::transmute(val)
                }
            },
            bind_buffer: unsafe {
                unsafe extern "system" fn bind_buffer(_target: GLenum, _buffer: GLuint) {
                    panic!("Unable to load bind_buffer")
                }
                let val = f(c"glBindBuffer");
                if val.is_null() {
                    bind_buffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_buffers: unsafe {
                unsafe extern "system" fn delete_buffers(_n: GLsizei, _buffers: *const GLuint) {
                    panic!("Unable to load delete_buffers")
                }
                let val = f(c"glDeleteBuffers");
                if val.is_null() {
                    delete_buffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_buffers: unsafe {
                unsafe extern "system" fn gen_buffers(_n: GLsizei, _buffers: *mut GLuint) {
                    panic!("Unable to load gen_buffers")
                }
                let val = f(c"glGenBuffers");
                if val.is_null() {
                    gen_buffers
                } else {
                    mem::transmute(val)
                }
            },
            is_buffer: unsafe {
                unsafe extern "system" fn is_buffer(_buffer: GLuint) -> GLboolean {
                    panic!("Unable to load is_buffer")
                }
                let val = f(c"glIsBuffer");
                if val.is_null() {
                    is_buffer
                } else {
                    mem::transmute(val)
                }
            },
            buffer_data: unsafe {
                unsafe extern "system" fn buffer_data(
                    _target: GLenum,
                    _size: GLsizeiptr,
                    _data: *const c_void,
                    _usage: GLenum,
                ) {
                    panic!("Unable to load buffer_data")
                }
                let val = f(c"glBufferData");
                if val.is_null() {
                    buffer_data
                } else {
                    mem::transmute(val)
                }
            },
            buffer_sub_data: unsafe {
                unsafe extern "system" fn buffer_sub_data(
                    _target: GLenum,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load buffer_sub_data")
                }
                let val = f(c"glBufferSubData");
                if val.is_null() {
                    buffer_sub_data
                } else {
                    mem::transmute(val)
                }
            },
            get_buffer_sub_data: unsafe {
                unsafe extern "system" fn get_buffer_sub_data(
                    _target: GLenum,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *mut c_void,
                ) {
                    panic!("Unable to load get_buffer_sub_data")
                }
                let val = f(c"glGetBufferSubData");
                if val.is_null() {
                    get_buffer_sub_data
                } else {
                    mem::transmute(val)
                }
            },
            map_buffer: unsafe {
                unsafe extern "system" fn map_buffer(
                    _target: GLenum,
                    _access: GLenum,
                ) -> *mut c_void {
                    panic!("Unable to load map_buffer")
                }
                let val = f(c"glMapBuffer");
                if val.is_null() {
                    map_buffer
                } else {
                    mem::transmute(val)
                }
            },
            unmap_buffer: unsafe {
                unsafe extern "system" fn unmap_buffer(_target: GLenum) -> GLboolean {
                    panic!("Unable to load unmap_buffer")
                }
                let val = f(c"glUnmapBuffer");
                if val.is_null() {
                    unmap_buffer
                } else {
                    mem::transmute(val)
                }
            },
            get_buffer_parameteriv: unsafe {
                unsafe extern "system" fn get_buffer_parameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_buffer_parameteriv")
                }
                let val = f(c"glGetBufferParameteriv");
                if val.is_null() {
                    get_buffer_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            get_buffer_pointerv: unsafe {
                unsafe extern "system" fn get_buffer_pointerv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut *mut c_void,
                ) {
                    panic!("Unable to load get_buffer_pointerv")
                }
                let val = f(c"glGetBufferPointerv");
                if val.is_null() {
                    get_buffer_pointerv
                } else {
                    mem::transmute(val)
                }
            },
            blend_equation_separate: unsafe {
                unsafe extern "system" fn blend_equation_separate(
                    _mode_rgb: GLenum,
                    _mode_alpha: GLenum,
                ) {
                    panic!("Unable to load blend_equation_separate")
                }
                let val = f(c"glBlendEquationSeparate");
                if val.is_null() {
                    blend_equation_separate
                } else {
                    mem::transmute(val)
                }
            },
            draw_buffers: unsafe {
                unsafe extern "system" fn draw_buffers(_n: GLsizei, _bufs: *const GLenum) {
                    panic!("Unable to load draw_buffers")
                }
                let val = f(c"glDrawBuffers");
                if val.is_null() {
                    draw_buffers
                } else {
                    mem::transmute(val)
                }
            },
            stencil_op_separate: unsafe {
                unsafe extern "system" fn stencil_op_separate(
                    _face: GLenum,
                    _sfail: GLenum,
                    _dpfail: GLenum,
                    _dppass: GLenum,
                ) {
                    panic!("Unable to load stencil_op_separate")
                }
                let val = f(c"glStencilOpSeparate");
                if val.is_null() {
                    stencil_op_separate
                } else {
                    mem::transmute(val)
                }
            },
            stencil_func_separate: unsafe {
                unsafe extern "system" fn stencil_func_separate(
                    _face: GLenum,
                    _func: GLenum,
                    _ref: GLint,
                    _mask: GLuint,
                ) {
                    panic!("Unable to load stencil_func_separate")
                }
                let val = f(c"glStencilFuncSeparate");
                if val.is_null() {
                    stencil_func_separate
                } else {
                    mem::transmute(val)
                }
            },
            stencil_mask_separate: unsafe {
                unsafe extern "system" fn stencil_mask_separate(_face: GLenum, _mask: GLuint) {
                    panic!("Unable to load stencil_mask_separate")
                }
                let val = f(c"glStencilMaskSeparate");
                if val.is_null() {
                    stencil_mask_separate
                } else {
                    mem::transmute(val)
                }
            },
            attach_shader: unsafe {
                unsafe extern "system" fn attach_shader(_program: GLuint, _shader: GLuint) {
                    panic!("Unable to load attach_shader")
                }
                let val = f(c"glAttachShader");
                if val.is_null() {
                    attach_shader
                } else {
                    mem::transmute(val)
                }
            },
            bind_attrib_location: unsafe {
                unsafe extern "system" fn bind_attrib_location(
                    _program: GLuint,
                    _index: GLuint,
                    _name: *const GLchar,
                ) {
                    panic!("Unable to load bind_attrib_location")
                }
                let val = f(c"glBindAttribLocation");
                if val.is_null() {
                    bind_attrib_location
                } else {
                    mem::transmute(val)
                }
            },
            compile_shader: unsafe {
                unsafe extern "system" fn compile_shader(_shader: GLuint) {
                    panic!("Unable to load compile_shader")
                }
                let val = f(c"glCompileShader");
                if val.is_null() {
                    compile_shader
                } else {
                    mem::transmute(val)
                }
            },
            create_program: unsafe {
                unsafe extern "system" fn create_program() -> GLuint {
                    panic!("Unable to load create_program")
                }
                let val = f(c"glCreateProgram");
                if val.is_null() {
                    create_program
                } else {
                    mem::transmute(val)
                }
            },
            create_shader: unsafe {
                unsafe extern "system" fn create_shader(_type: GLenum) -> GLuint {
                    panic!("Unable to load create_shader")
                }
                let val = f(c"glCreateShader");
                if val.is_null() {
                    create_shader
                } else {
                    mem::transmute(val)
                }
            },
            delete_program: unsafe {
                unsafe extern "system" fn delete_program(_program: GLuint) {
                    panic!("Unable to load delete_program")
                }
                let val = f(c"glDeleteProgram");
                if val.is_null() {
                    delete_program
                } else {
                    mem::transmute(val)
                }
            },
            delete_shader: unsafe {
                unsafe extern "system" fn delete_shader(_shader: GLuint) {
                    panic!("Unable to load delete_shader")
                }
                let val = f(c"glDeleteShader");
                if val.is_null() {
                    delete_shader
                } else {
                    mem::transmute(val)
                }
            },
            detach_shader: unsafe {
                unsafe extern "system" fn detach_shader(_program: GLuint, _shader: GLuint) {
                    panic!("Unable to load detach_shader")
                }
                let val = f(c"glDetachShader");
                if val.is_null() {
                    detach_shader
                } else {
                    mem::transmute(val)
                }
            },
            disable_vertex_attrib_array: unsafe {
                unsafe extern "system" fn disable_vertex_attrib_array(_index: GLuint) {
                    panic!("Unable to load disable_vertex_attrib_array")
                }
                let val = f(c"glDisableVertexAttribArray");
                if val.is_null() {
                    disable_vertex_attrib_array
                } else {
                    mem::transmute(val)
                }
            },
            enable_vertex_attrib_array: unsafe {
                unsafe extern "system" fn enable_vertex_attrib_array(_index: GLuint) {
                    panic!("Unable to load enable_vertex_attrib_array")
                }
                let val = f(c"glEnableVertexAttribArray");
                if val.is_null() {
                    enable_vertex_attrib_array
                } else {
                    mem::transmute(val)
                }
            },
            get_active_attrib: unsafe {
                unsafe extern "system" fn get_active_attrib(
                    _program: GLuint,
                    _index: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLint,
                    _type: *mut GLenum,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load get_active_attrib")
                }
                let val = f(c"glGetActiveAttrib");
                if val.is_null() {
                    get_active_attrib
                } else {
                    mem::transmute(val)
                }
            },
            get_active_uniform: unsafe {
                unsafe extern "system" fn get_active_uniform(
                    _program: GLuint,
                    _index: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLint,
                    _type: *mut GLenum,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load get_active_uniform")
                }
                let val = f(c"glGetActiveUniform");
                if val.is_null() {
                    get_active_uniform
                } else {
                    mem::transmute(val)
                }
            },
            get_attached_shaders: unsafe {
                unsafe extern "system" fn get_attached_shaders(
                    _program: GLuint,
                    _max_count: GLsizei,
                    _count: *mut GLsizei,
                    _shaders: *mut GLuint,
                ) {
                    panic!("Unable to load get_attached_shaders")
                }
                let val = f(c"glGetAttachedShaders");
                if val.is_null() {
                    get_attached_shaders
                } else {
                    mem::transmute(val)
                }
            },
            get_attrib_location: unsafe {
                unsafe extern "system" fn get_attrib_location(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load get_attrib_location")
                }
                let val = f(c"glGetAttribLocation");
                if val.is_null() {
                    get_attrib_location
                } else {
                    mem::transmute(val)
                }
            },
            get_programiv: unsafe {
                unsafe extern "system" fn get_programiv(
                    _program: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_programiv")
                }
                let val = f(c"glGetProgramiv");
                if val.is_null() {
                    get_programiv
                } else {
                    mem::transmute(val)
                }
            },
            get_program_info_log: unsafe {
                unsafe extern "system" fn get_program_info_log(
                    _program: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _info_log: *mut GLchar,
                ) {
                    panic!("Unable to load get_program_info_log")
                }
                let val = f(c"glGetProgramInfoLog");
                if val.is_null() {
                    get_program_info_log
                } else {
                    mem::transmute(val)
                }
            },
            get_shaderiv: unsafe {
                unsafe extern "system" fn get_shaderiv(
                    _shader: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_shaderiv")
                }
                let val = f(c"glGetShaderiv");
                if val.is_null() {
                    get_shaderiv
                } else {
                    mem::transmute(val)
                }
            },
            get_shader_info_log: unsafe {
                unsafe extern "system" fn get_shader_info_log(
                    _shader: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _info_log: *mut GLchar,
                ) {
                    panic!("Unable to load get_shader_info_log")
                }
                let val = f(c"glGetShaderInfoLog");
                if val.is_null() {
                    get_shader_info_log
                } else {
                    mem::transmute(val)
                }
            },
            get_shader_source: unsafe {
                unsafe extern "system" fn get_shader_source(
                    _shader: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _source: *mut GLchar,
                ) {
                    panic!("Unable to load get_shader_source")
                }
                let val = f(c"glGetShaderSource");
                if val.is_null() {
                    get_shader_source
                } else {
                    mem::transmute(val)
                }
            },
            get_uniform_location: unsafe {
                unsafe extern "system" fn get_uniform_location(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load get_uniform_location")
                }
                let val = f(c"glGetUniformLocation");
                if val.is_null() {
                    get_uniform_location
                } else {
                    mem::transmute(val)
                }
            },
            get_uniformfv: unsafe {
                unsafe extern "system" fn get_uniformfv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load get_uniformfv")
                }
                let val = f(c"glGetUniformfv");
                if val.is_null() {
                    get_uniformfv
                } else {
                    mem::transmute(val)
                }
            },
            get_uniformiv: unsafe {
                unsafe extern "system" fn get_uniformiv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_uniformiv")
                }
                let val = f(c"glGetUniformiv");
                if val.is_null() {
                    get_uniformiv
                } else {
                    mem::transmute(val)
                }
            },
            get_vertex_attribdv: unsafe {
                unsafe extern "system" fn get_vertex_attribdv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLdouble,
                ) {
                    panic!("Unable to load get_vertex_attribdv")
                }
                let val = f(c"glGetVertexAttribdv");
                if val.is_null() {
                    get_vertex_attribdv
                } else {
                    mem::transmute(val)
                }
            },
            get_vertex_attribfv: unsafe {
                unsafe extern "system" fn get_vertex_attribfv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load get_vertex_attribfv")
                }
                let val = f(c"glGetVertexAttribfv");
                if val.is_null() {
                    get_vertex_attribfv
                } else {
                    mem::transmute(val)
                }
            },
            get_vertex_attribiv: unsafe {
                unsafe extern "system" fn get_vertex_attribiv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_vertex_attribiv")
                }
                let val = f(c"glGetVertexAttribiv");
                if val.is_null() {
                    get_vertex_attribiv
                } else {
                    mem::transmute(val)
                }
            },
            get_vertex_attrib_pointerv: unsafe {
                unsafe extern "system" fn get_vertex_attrib_pointerv(
                    _index: GLuint,
                    _pname: GLenum,
                    _pointer: *mut *mut c_void,
                ) {
                    panic!("Unable to load get_vertex_attrib_pointerv")
                }
                let val = f(c"glGetVertexAttribPointerv");
                if val.is_null() {
                    get_vertex_attrib_pointerv
                } else {
                    mem::transmute(val)
                }
            },
            is_program: unsafe {
                unsafe extern "system" fn is_program(_program: GLuint) -> GLboolean {
                    panic!("Unable to load is_program")
                }
                let val = f(c"glIsProgram");
                if val.is_null() {
                    is_program
                } else {
                    mem::transmute(val)
                }
            },
            is_shader: unsafe {
                unsafe extern "system" fn is_shader(_shader: GLuint) -> GLboolean {
                    panic!("Unable to load is_shader")
                }
                let val = f(c"glIsShader");
                if val.is_null() {
                    is_shader
                } else {
                    mem::transmute(val)
                }
            },
            link_program: unsafe {
                unsafe extern "system" fn link_program(_program: GLuint) {
                    panic!("Unable to load link_program")
                }
                let val = f(c"glLinkProgram");
                if val.is_null() {
                    link_program
                } else {
                    mem::transmute(val)
                }
            },
            shader_source: unsafe {
                unsafe extern "system" fn shader_source(
                    _shader: GLuint,
                    _count: GLsizei,
                    _string: *const *const GLchar,
                    _length: *const GLint,
                ) {
                    panic!("Unable to load shader_source")
                }
                let val = f(c"glShaderSource");
                if val.is_null() {
                    shader_source
                } else {
                    mem::transmute(val)
                }
            },
            use_program: unsafe {
                unsafe extern "system" fn use_program(_program: GLuint) {
                    panic!("Unable to load use_program")
                }
                let val = f(c"glUseProgram");
                if val.is_null() {
                    use_program
                } else {
                    mem::transmute(val)
                }
            },
            uniform1f: unsafe {
                unsafe extern "system" fn uniform1f(_location: GLint, _v0: GLfloat) {
                    panic!("Unable to load uniform1f")
                }
                let val = f(c"glUniform1f");
                if val.is_null() {
                    uniform1f
                } else {
                    mem::transmute(val)
                }
            },
            uniform2f: unsafe {
                unsafe extern "system" fn uniform2f(_location: GLint, _v0: GLfloat, _v1: GLfloat) {
                    panic!("Unable to load uniform2f")
                }
                let val = f(c"glUniform2f");
                if val.is_null() {
                    uniform2f
                } else {
                    mem::transmute(val)
                }
            },
            uniform3f: unsafe {
                unsafe extern "system" fn uniform3f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                ) {
                    panic!("Unable to load uniform3f")
                }
                let val = f(c"glUniform3f");
                if val.is_null() {
                    uniform3f
                } else {
                    mem::transmute(val)
                }
            },
            uniform4f: unsafe {
                unsafe extern "system" fn uniform4f(
                    _location: GLint,
                    _v0: GLfloat,
                    _v1: GLfloat,
                    _v2: GLfloat,
                    _v3: GLfloat,
                ) {
                    panic!("Unable to load uniform4f")
                }
                let val = f(c"glUniform4f");
                if val.is_null() {
                    uniform4f
                } else {
                    mem::transmute(val)
                }
            },
            uniform1i: unsafe {
                unsafe extern "system" fn uniform1i(_location: GLint, _v0: GLint) {
                    panic!("Unable to load uniform1i")
                }
                let val = f(c"glUniform1i");
                if val.is_null() {
                    uniform1i
                } else {
                    mem::transmute(val)
                }
            },
            uniform2i: unsafe {
                unsafe extern "system" fn uniform2i(_location: GLint, _v0: GLint, _v1: GLint) {
                    panic!("Unable to load uniform2i")
                }
                let val = f(c"glUniform2i");
                if val.is_null() {
                    uniform2i
                } else {
                    mem::transmute(val)
                }
            },
            uniform3i: unsafe {
                unsafe extern "system" fn uniform3i(
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                ) {
                    panic!("Unable to load uniform3i")
                }
                let val = f(c"glUniform3i");
                if val.is_null() {
                    uniform3i
                } else {
                    mem::transmute(val)
                }
            },
            uniform4i: unsafe {
                unsafe extern "system" fn uniform4i(
                    _location: GLint,
                    _v0: GLint,
                    _v1: GLint,
                    _v2: GLint,
                    _v3: GLint,
                ) {
                    panic!("Unable to load uniform4i")
                }
                let val = f(c"glUniform4i");
                if val.is_null() {
                    uniform4i
                } else {
                    mem::transmute(val)
                }
            },
            uniform1fv: unsafe {
                unsafe extern "system" fn uniform1fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform1fv")
                }
                let val = f(c"glUniform1fv");
                if val.is_null() {
                    uniform1fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform2fv: unsafe {
                unsafe extern "system" fn uniform2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform2fv")
                }
                let val = f(c"glUniform2fv");
                if val.is_null() {
                    uniform2fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform3fv: unsafe {
                unsafe extern "system" fn uniform3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform3fv")
                }
                let val = f(c"glUniform3fv");
                if val.is_null() {
                    uniform3fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform4fv: unsafe {
                unsafe extern "system" fn uniform4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform4fv")
                }
                let val = f(c"glUniform4fv");
                if val.is_null() {
                    uniform4fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform1iv: unsafe {
                unsafe extern "system" fn uniform1iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load uniform1iv")
                }
                let val = f(c"glUniform1iv");
                if val.is_null() {
                    uniform1iv
                } else {
                    mem::transmute(val)
                }
            },
            uniform2iv: unsafe {
                unsafe extern "system" fn uniform2iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load uniform2iv")
                }
                let val = f(c"glUniform2iv");
                if val.is_null() {
                    uniform2iv
                } else {
                    mem::transmute(val)
                }
            },
            uniform3iv: unsafe {
                unsafe extern "system" fn uniform3iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load uniform3iv")
                }
                let val = f(c"glUniform3iv");
                if val.is_null() {
                    uniform3iv
                } else {
                    mem::transmute(val)
                }
            },
            uniform4iv: unsafe {
                unsafe extern "system" fn uniform4iv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load uniform4iv")
                }
                let val = f(c"glUniform4iv");
                if val.is_null() {
                    uniform4iv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix2fv: unsafe {
                unsafe extern "system" fn uniform_matrix2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix2fv")
                }
                let val = f(c"glUniformMatrix2fv");
                if val.is_null() {
                    uniform_matrix2fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix3fv: unsafe {
                unsafe extern "system" fn uniform_matrix3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix3fv")
                }
                let val = f(c"glUniformMatrix3fv");
                if val.is_null() {
                    uniform_matrix3fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix4fv: unsafe {
                unsafe extern "system" fn uniform_matrix4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix4fv")
                }
                let val = f(c"glUniformMatrix4fv");
                if val.is_null() {
                    uniform_matrix4fv
                } else {
                    mem::transmute(val)
                }
            },
            validate_program: unsafe {
                unsafe extern "system" fn validate_program(_program: GLuint) {
                    panic!("Unable to load validate_program")
                }
                let val = f(c"glValidateProgram");
                if val.is_null() {
                    validate_program
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib1d: unsafe {
                unsafe extern "system" fn vertex_attrib1d(_index: GLuint, _x: GLdouble) {
                    panic!("Unable to load vertex_attrib1d")
                }
                let val = f(c"glVertexAttrib1d");
                if val.is_null() {
                    vertex_attrib1d
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib1dv: unsafe {
                unsafe extern "system" fn vertex_attrib1dv(_index: GLuint, _v: *const GLdouble) {
                    panic!("Unable to load vertex_attrib1dv")
                }
                let val = f(c"glVertexAttrib1dv");
                if val.is_null() {
                    vertex_attrib1dv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib1f: unsafe {
                unsafe extern "system" fn vertex_attrib1f(_index: GLuint, _x: GLfloat) {
                    panic!("Unable to load vertex_attrib1f")
                }
                let val = f(c"glVertexAttrib1f");
                if val.is_null() {
                    vertex_attrib1f
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib1fv: unsafe {
                unsafe extern "system" fn vertex_attrib1fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load vertex_attrib1fv")
                }
                let val = f(c"glVertexAttrib1fv");
                if val.is_null() {
                    vertex_attrib1fv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib1s: unsafe {
                unsafe extern "system" fn vertex_attrib1s(_index: GLuint, _x: GLshort) {
                    panic!("Unable to load vertex_attrib1s")
                }
                let val = f(c"glVertexAttrib1s");
                if val.is_null() {
                    vertex_attrib1s
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib1sv: unsafe {
                unsafe extern "system" fn vertex_attrib1sv(_index: GLuint, _v: *const GLshort) {
                    panic!("Unable to load vertex_attrib1sv")
                }
                let val = f(c"glVertexAttrib1sv");
                if val.is_null() {
                    vertex_attrib1sv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib2d: unsafe {
                unsafe extern "system" fn vertex_attrib2d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                ) {
                    panic!("Unable to load vertex_attrib2d")
                }
                let val = f(c"glVertexAttrib2d");
                if val.is_null() {
                    vertex_attrib2d
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib2dv: unsafe {
                unsafe extern "system" fn vertex_attrib2dv(_index: GLuint, _v: *const GLdouble) {
                    panic!("Unable to load vertex_attrib2dv")
                }
                let val = f(c"glVertexAttrib2dv");
                if val.is_null() {
                    vertex_attrib2dv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib2f: unsafe {
                unsafe extern "system" fn vertex_attrib2f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                ) {
                    panic!("Unable to load vertex_attrib2f")
                }
                let val = f(c"glVertexAttrib2f");
                if val.is_null() {
                    vertex_attrib2f
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib2fv: unsafe {
                unsafe extern "system" fn vertex_attrib2fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load vertex_attrib2fv")
                }
                let val = f(c"glVertexAttrib2fv");
                if val.is_null() {
                    vertex_attrib2fv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib2s: unsafe {
                unsafe extern "system" fn vertex_attrib2s(
                    _index: GLuint,
                    _x: GLshort,
                    _y: GLshort,
                ) {
                    panic!("Unable to load vertex_attrib2s")
                }
                let val = f(c"glVertexAttrib2s");
                if val.is_null() {
                    vertex_attrib2s
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib2sv: unsafe {
                unsafe extern "system" fn vertex_attrib2sv(_index: GLuint, _v: *const GLshort) {
                    panic!("Unable to load vertex_attrib2sv")
                }
                let val = f(c"glVertexAttrib2sv");
                if val.is_null() {
                    vertex_attrib2sv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib3d: unsafe {
                unsafe extern "system" fn vertex_attrib3d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                ) {
                    panic!("Unable to load vertex_attrib3d")
                }
                let val = f(c"glVertexAttrib3d");
                if val.is_null() {
                    vertex_attrib3d
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib3dv: unsafe {
                unsafe extern "system" fn vertex_attrib3dv(_index: GLuint, _v: *const GLdouble) {
                    panic!("Unable to load vertex_attrib3dv")
                }
                let val = f(c"glVertexAttrib3dv");
                if val.is_null() {
                    vertex_attrib3dv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib3f: unsafe {
                unsafe extern "system" fn vertex_attrib3f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                ) {
                    panic!("Unable to load vertex_attrib3f")
                }
                let val = f(c"glVertexAttrib3f");
                if val.is_null() {
                    vertex_attrib3f
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib3fv: unsafe {
                unsafe extern "system" fn vertex_attrib3fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load vertex_attrib3fv")
                }
                let val = f(c"glVertexAttrib3fv");
                if val.is_null() {
                    vertex_attrib3fv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib3s: unsafe {
                unsafe extern "system" fn vertex_attrib3s(
                    _index: GLuint,
                    _x: GLshort,
                    _y: GLshort,
                    _z: GLshort,
                ) {
                    panic!("Unable to load vertex_attrib3s")
                }
                let val = f(c"glVertexAttrib3s");
                if val.is_null() {
                    vertex_attrib3s
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib3sv: unsafe {
                unsafe extern "system" fn vertex_attrib3sv(_index: GLuint, _v: *const GLshort) {
                    panic!("Unable to load vertex_attrib3sv")
                }
                let val = f(c"glVertexAttrib3sv");
                if val.is_null() {
                    vertex_attrib3sv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4nbv: unsafe {
                unsafe extern "system" fn vertex_attrib_4nbv(_index: GLuint, _v: *const GLbyte) {
                    panic!("Unable to load vertex_attrib_4nbv")
                }
                let val = f(c"glVertexAttrib4Nbv");
                if val.is_null() {
                    vertex_attrib_4nbv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4niv: unsafe {
                unsafe extern "system" fn vertex_attrib_4niv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load vertex_attrib_4niv")
                }
                let val = f(c"glVertexAttrib4Niv");
                if val.is_null() {
                    vertex_attrib_4niv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4nsv: unsafe {
                unsafe extern "system" fn vertex_attrib_4nsv(_index: GLuint, _v: *const GLshort) {
                    panic!("Unable to load vertex_attrib_4nsv")
                }
                let val = f(c"glVertexAttrib4Nsv");
                if val.is_null() {
                    vertex_attrib_4nsv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4nub: unsafe {
                unsafe extern "system" fn vertex_attrib_4nub(
                    _index: GLuint,
                    _x: GLubyte,
                    _y: GLubyte,
                    _z: GLubyte,
                    _w: GLubyte,
                ) {
                    panic!("Unable to load vertex_attrib_4nub")
                }
                let val = f(c"glVertexAttrib4Nub");
                if val.is_null() {
                    vertex_attrib_4nub
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4nubv: unsafe {
                unsafe extern "system" fn vertex_attrib_4nubv(_index: GLuint, _v: *const GLubyte) {
                    panic!("Unable to load vertex_attrib_4nubv")
                }
                let val = f(c"glVertexAttrib4Nubv");
                if val.is_null() {
                    vertex_attrib_4nubv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4nuiv: unsafe {
                unsafe extern "system" fn vertex_attrib_4nuiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load vertex_attrib_4nuiv")
                }
                let val = f(c"glVertexAttrib4Nuiv");
                if val.is_null() {
                    vertex_attrib_4nuiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_4nusv: unsafe {
                unsafe extern "system" fn vertex_attrib_4nusv(_index: GLuint, _v: *const GLushort) {
                    panic!("Unable to load vertex_attrib_4nusv")
                }
                let val = f(c"glVertexAttrib4Nusv");
                if val.is_null() {
                    vertex_attrib_4nusv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4bv: unsafe {
                unsafe extern "system" fn vertex_attrib4bv(_index: GLuint, _v: *const GLbyte) {
                    panic!("Unable to load vertex_attrib4bv")
                }
                let val = f(c"glVertexAttrib4bv");
                if val.is_null() {
                    vertex_attrib4bv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4d: unsafe {
                unsafe extern "system" fn vertex_attrib4d(
                    _index: GLuint,
                    _x: GLdouble,
                    _y: GLdouble,
                    _z: GLdouble,
                    _w: GLdouble,
                ) {
                    panic!("Unable to load vertex_attrib4d")
                }
                let val = f(c"glVertexAttrib4d");
                if val.is_null() {
                    vertex_attrib4d
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4dv: unsafe {
                unsafe extern "system" fn vertex_attrib4dv(_index: GLuint, _v: *const GLdouble) {
                    panic!("Unable to load vertex_attrib4dv")
                }
                let val = f(c"glVertexAttrib4dv");
                if val.is_null() {
                    vertex_attrib4dv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4f: unsafe {
                unsafe extern "system" fn vertex_attrib4f(
                    _index: GLuint,
                    _x: GLfloat,
                    _y: GLfloat,
                    _z: GLfloat,
                    _w: GLfloat,
                ) {
                    panic!("Unable to load vertex_attrib4f")
                }
                let val = f(c"glVertexAttrib4f");
                if val.is_null() {
                    vertex_attrib4f
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4fv: unsafe {
                unsafe extern "system" fn vertex_attrib4fv(_index: GLuint, _v: *const GLfloat) {
                    panic!("Unable to load vertex_attrib4fv")
                }
                let val = f(c"glVertexAttrib4fv");
                if val.is_null() {
                    vertex_attrib4fv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4iv: unsafe {
                unsafe extern "system" fn vertex_attrib4iv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load vertex_attrib4iv")
                }
                let val = f(c"glVertexAttrib4iv");
                if val.is_null() {
                    vertex_attrib4iv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4s: unsafe {
                unsafe extern "system" fn vertex_attrib4s(
                    _index: GLuint,
                    _x: GLshort,
                    _y: GLshort,
                    _z: GLshort,
                    _w: GLshort,
                ) {
                    panic!("Unable to load vertex_attrib4s")
                }
                let val = f(c"glVertexAttrib4s");
                if val.is_null() {
                    vertex_attrib4s
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4sv: unsafe {
                unsafe extern "system" fn vertex_attrib4sv(_index: GLuint, _v: *const GLshort) {
                    panic!("Unable to load vertex_attrib4sv")
                }
                let val = f(c"glVertexAttrib4sv");
                if val.is_null() {
                    vertex_attrib4sv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4ubv: unsafe {
                unsafe extern "system" fn vertex_attrib4ubv(_index: GLuint, _v: *const GLubyte) {
                    panic!("Unable to load vertex_attrib4ubv")
                }
                let val = f(c"glVertexAttrib4ubv");
                if val.is_null() {
                    vertex_attrib4ubv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4uiv: unsafe {
                unsafe extern "system" fn vertex_attrib4uiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load vertex_attrib4uiv")
                }
                let val = f(c"glVertexAttrib4uiv");
                if val.is_null() {
                    vertex_attrib4uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib4usv: unsafe {
                unsafe extern "system" fn vertex_attrib4usv(_index: GLuint, _v: *const GLushort) {
                    panic!("Unable to load vertex_attrib4usv")
                }
                let val = f(c"glVertexAttrib4usv");
                if val.is_null() {
                    vertex_attrib4usv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_pointer: unsafe {
                unsafe extern "system" fn vertex_attrib_pointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _stride: GLsizei,
                    _pointer: *const c_void,
                ) {
                    panic!("Unable to load vertex_attrib_pointer")
                }
                let val = f(c"glVertexAttribPointer");
                if val.is_null() {
                    vertex_attrib_pointer
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix2x3fv: unsafe {
                unsafe extern "system" fn uniform_matrix2x3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix2x3fv")
                }
                let val = f(c"glUniformMatrix2x3fv");
                if val.is_null() {
                    uniform_matrix2x3fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix3x2fv: unsafe {
                unsafe extern "system" fn uniform_matrix3x2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix3x2fv")
                }
                let val = f(c"glUniformMatrix3x2fv");
                if val.is_null() {
                    uniform_matrix3x2fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix2x4fv: unsafe {
                unsafe extern "system" fn uniform_matrix2x4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix2x4fv")
                }
                let val = f(c"glUniformMatrix2x4fv");
                if val.is_null() {
                    uniform_matrix2x4fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix4x2fv: unsafe {
                unsafe extern "system" fn uniform_matrix4x2fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix4x2fv")
                }
                let val = f(c"glUniformMatrix4x2fv");
                if val.is_null() {
                    uniform_matrix4x2fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix3x4fv: unsafe {
                unsafe extern "system" fn uniform_matrix3x4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix3x4fv")
                }
                let val = f(c"glUniformMatrix3x4fv");
                if val.is_null() {
                    uniform_matrix3x4fv
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix4x3fv: unsafe {
                unsafe extern "system" fn uniform_matrix4x3fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix4x3fv")
                }
                let val = f(c"glUniformMatrix4x3fv");
                if val.is_null() {
                    uniform_matrix4x3fv
                } else {
                    mem::transmute(val)
                }
            },
            color_maski: unsafe {
                unsafe extern "system" fn color_maski(
                    _index: GLuint,
                    _r: GLboolean,
                    _g: GLboolean,
                    _b: GLboolean,
                    _a: GLboolean,
                ) {
                    panic!("Unable to load color_maski")
                }
                let val = f(c"glColorMaski");
                if val.is_null() {
                    color_maski
                } else {
                    mem::transmute(val)
                }
            },
            get_booleani_v: unsafe {
                unsafe extern "system" fn get_booleani_v(
                    _target: GLenum,
                    _index: GLuint,
                    _data: *mut GLboolean,
                ) {
                    panic!("Unable to load get_booleani_v")
                }
                let val = f(c"glGetBooleani_v");
                if val.is_null() {
                    get_booleani_v
                } else {
                    mem::transmute(val)
                }
            },
            get_integeri_v: unsafe {
                unsafe extern "system" fn get_integeri_v(
                    _target: GLenum,
                    _index: GLuint,
                    _data: *mut GLint,
                ) {
                    panic!("Unable to load get_integeri_v")
                }
                let val = f(c"glGetIntegeri_v");
                if val.is_null() {
                    get_integeri_v
                } else {
                    mem::transmute(val)
                }
            },
            enablei: unsafe {
                unsafe extern "system" fn enablei(_target: GLenum, _index: GLuint) {
                    panic!("Unable to load enablei")
                }
                let val = f(c"glEnablei");
                if val.is_null() {
                    enablei
                } else {
                    mem::transmute(val)
                }
            },
            disablei: unsafe {
                unsafe extern "system" fn disablei(_target: GLenum, _index: GLuint) {
                    panic!("Unable to load disablei")
                }
                let val = f(c"glDisablei");
                if val.is_null() {
                    disablei
                } else {
                    mem::transmute(val)
                }
            },
            is_enabledi: unsafe {
                unsafe extern "system" fn is_enabledi(
                    _target: GLenum,
                    _index: GLuint,
                ) -> GLboolean {
                    panic!("Unable to load is_enabledi")
                }
                let val = f(c"glIsEnabledi");
                if val.is_null() {
                    is_enabledi
                } else {
                    mem::transmute(val)
                }
            },
            begin_transform_feedback: unsafe {
                unsafe extern "system" fn begin_transform_feedback(_primitive_mode: GLenum) {
                    panic!("Unable to load begin_transform_feedback")
                }
                let val = f(c"glBeginTransformFeedback");
                if val.is_null() {
                    begin_transform_feedback
                } else {
                    mem::transmute(val)
                }
            },
            end_transform_feedback: unsafe {
                unsafe extern "system" fn end_transform_feedback() {
                    panic!("Unable to load end_transform_feedback")
                }
                let val = f(c"glEndTransformFeedback");
                if val.is_null() {
                    end_transform_feedback
                } else {
                    mem::transmute(val)
                }
            },
            bind_buffer_range: unsafe {
                unsafe extern "system" fn bind_buffer_range(
                    _target: GLenum,
                    _index: GLuint,
                    _buffer: GLuint,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load bind_buffer_range")
                }
                let val = f(c"glBindBufferRange");
                if val.is_null() {
                    bind_buffer_range
                } else {
                    mem::transmute(val)
                }
            },
            bind_buffer_base: unsafe {
                unsafe extern "system" fn bind_buffer_base(
                    _target: GLenum,
                    _index: GLuint,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load bind_buffer_base")
                }
                let val = f(c"glBindBufferBase");
                if val.is_null() {
                    bind_buffer_base
                } else {
                    mem::transmute(val)
                }
            },
            transform_feedback_varyings: unsafe {
                unsafe extern "system" fn transform_feedback_varyings(
                    _program: GLuint,
                    _count: GLsizei,
                    _varyings: *const *const GLchar,
                    _buffer_mode: GLenum,
                ) {
                    panic!("Unable to load transform_feedback_varyings")
                }
                let val = f(c"glTransformFeedbackVaryings");
                if val.is_null() {
                    transform_feedback_varyings
                } else {
                    mem::transmute(val)
                }
            },
            get_transform_feedback_varying: unsafe {
                unsafe extern "system" fn get_transform_feedback_varying(
                    _program: GLuint,
                    _index: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _size: *mut GLsizei,
                    _type: *mut GLenum,
                    _name: *mut GLchar,
                ) {
                    panic!("Unable to load get_transform_feedback_varying")
                }
                let val = f(c"glGetTransformFeedbackVarying");
                if val.is_null() {
                    get_transform_feedback_varying
                } else {
                    mem::transmute(val)
                }
            },
            clamp_color: unsafe {
                unsafe extern "system" fn clamp_color(_target: GLenum, _clamp: GLenum) {
                    panic!("Unable to load clamp_color")
                }
                let val = f(c"glClampColor");
                if val.is_null() {
                    clamp_color
                } else {
                    mem::transmute(val)
                }
            },
            begin_conditional_render: unsafe {
                unsafe extern "system" fn begin_conditional_render(_id: GLuint, _mode: GLenum) {
                    panic!("Unable to load begin_conditional_render")
                }
                let val = f(c"glBeginConditionalRender");
                if val.is_null() {
                    begin_conditional_render
                } else {
                    mem::transmute(val)
                }
            },
            end_conditional_render: unsafe {
                unsafe extern "system" fn end_conditional_render() {
                    panic!("Unable to load end_conditional_render")
                }
                let val = f(c"glEndConditionalRender");
                if val.is_null() {
                    end_conditional_render
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_ipointer: unsafe {
                unsafe extern "system" fn vertex_attrib_ipointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _stride: GLsizei,
                    _pointer: *const c_void,
                ) {
                    panic!("Unable to load vertex_attrib_ipointer")
                }
                let val = f(c"glVertexAttribIPointer");
                if val.is_null() {
                    vertex_attrib_ipointer
                } else {
                    mem::transmute(val)
                }
            },
            get_vertex_attrib_iiv: unsafe {
                unsafe extern "system" fn get_vertex_attrib_iiv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_vertex_attrib_iiv")
                }
                let val = f(c"glGetVertexAttribIiv");
                if val.is_null() {
                    get_vertex_attrib_iiv
                } else {
                    mem::transmute(val)
                }
            },
            get_vertex_attrib_iuiv: unsafe {
                unsafe extern "system" fn get_vertex_attrib_iuiv(
                    _index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load get_vertex_attrib_iuiv")
                }
                let val = f(c"glGetVertexAttribIuiv");
                if val.is_null() {
                    get_vertex_attrib_iuiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i1i: unsafe {
                unsafe extern "system" fn vertex_attrib_i1i(_index: GLuint, _x: GLint) {
                    panic!("Unable to load vertex_attrib_i1i")
                }
                let val = f(c"glVertexAttribI1i");
                if val.is_null() {
                    vertex_attrib_i1i
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i2i: unsafe {
                unsafe extern "system" fn vertex_attrib_i2i(_index: GLuint, _x: GLint, _y: GLint) {
                    panic!("Unable to load vertex_attrib_i2i")
                }
                let val = f(c"glVertexAttribI2i");
                if val.is_null() {
                    vertex_attrib_i2i
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i3i: unsafe {
                unsafe extern "system" fn vertex_attrib_i3i(
                    _index: GLuint,
                    _x: GLint,
                    _y: GLint,
                    _z: GLint,
                ) {
                    panic!("Unable to load vertex_attrib_i3i")
                }
                let val = f(c"glVertexAttribI3i");
                if val.is_null() {
                    vertex_attrib_i3i
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4i: unsafe {
                unsafe extern "system" fn vertex_attrib_i4i(
                    _index: GLuint,
                    _x: GLint,
                    _y: GLint,
                    _z: GLint,
                    _w: GLint,
                ) {
                    panic!("Unable to load vertex_attrib_i4i")
                }
                let val = f(c"glVertexAttribI4i");
                if val.is_null() {
                    vertex_attrib_i4i
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i1ui: unsafe {
                unsafe extern "system" fn vertex_attrib_i1ui(_index: GLuint, _x: GLuint) {
                    panic!("Unable to load vertex_attrib_i1ui")
                }
                let val = f(c"glVertexAttribI1ui");
                if val.is_null() {
                    vertex_attrib_i1ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i2ui: unsafe {
                unsafe extern "system" fn vertex_attrib_i2ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_i2ui")
                }
                let val = f(c"glVertexAttribI2ui");
                if val.is_null() {
                    vertex_attrib_i2ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i3ui: unsafe {
                unsafe extern "system" fn vertex_attrib_i3ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                    _z: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_i3ui")
                }
                let val = f(c"glVertexAttribI3ui");
                if val.is_null() {
                    vertex_attrib_i3ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4ui: unsafe {
                unsafe extern "system" fn vertex_attrib_i4ui(
                    _index: GLuint,
                    _x: GLuint,
                    _y: GLuint,
                    _z: GLuint,
                    _w: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_i4ui")
                }
                let val = f(c"glVertexAttribI4ui");
                if val.is_null() {
                    vertex_attrib_i4ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i1iv: unsafe {
                unsafe extern "system" fn vertex_attrib_i1iv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load vertex_attrib_i1iv")
                }
                let val = f(c"glVertexAttribI1iv");
                if val.is_null() {
                    vertex_attrib_i1iv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i2iv: unsafe {
                unsafe extern "system" fn vertex_attrib_i2iv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load vertex_attrib_i2iv")
                }
                let val = f(c"glVertexAttribI2iv");
                if val.is_null() {
                    vertex_attrib_i2iv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i3iv: unsafe {
                unsafe extern "system" fn vertex_attrib_i3iv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load vertex_attrib_i3iv")
                }
                let val = f(c"glVertexAttribI3iv");
                if val.is_null() {
                    vertex_attrib_i3iv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4iv: unsafe {
                unsafe extern "system" fn vertex_attrib_i4iv(_index: GLuint, _v: *const GLint) {
                    panic!("Unable to load vertex_attrib_i4iv")
                }
                let val = f(c"glVertexAttribI4iv");
                if val.is_null() {
                    vertex_attrib_i4iv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i1uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_i1uiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load vertex_attrib_i1uiv")
                }
                let val = f(c"glVertexAttribI1uiv");
                if val.is_null() {
                    vertex_attrib_i1uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i2uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_i2uiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load vertex_attrib_i2uiv")
                }
                let val = f(c"glVertexAttribI2uiv");
                if val.is_null() {
                    vertex_attrib_i2uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i3uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_i3uiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load vertex_attrib_i3uiv")
                }
                let val = f(c"glVertexAttribI3uiv");
                if val.is_null() {
                    vertex_attrib_i3uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_i4uiv(_index: GLuint, _v: *const GLuint) {
                    panic!("Unable to load vertex_attrib_i4uiv")
                }
                let val = f(c"glVertexAttribI4uiv");
                if val.is_null() {
                    vertex_attrib_i4uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4bv: unsafe {
                unsafe extern "system" fn vertex_attrib_i4bv(_index: GLuint, _v: *const GLbyte) {
                    panic!("Unable to load vertex_attrib_i4bv")
                }
                let val = f(c"glVertexAttribI4bv");
                if val.is_null() {
                    vertex_attrib_i4bv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4sv: unsafe {
                unsafe extern "system" fn vertex_attrib_i4sv(_index: GLuint, _v: *const GLshort) {
                    panic!("Unable to load vertex_attrib_i4sv")
                }
                let val = f(c"glVertexAttribI4sv");
                if val.is_null() {
                    vertex_attrib_i4sv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4ubv: unsafe {
                unsafe extern "system" fn vertex_attrib_i4ubv(_index: GLuint, _v: *const GLubyte) {
                    panic!("Unable to load vertex_attrib_i4ubv")
                }
                let val = f(c"glVertexAttribI4ubv");
                if val.is_null() {
                    vertex_attrib_i4ubv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_i4usv: unsafe {
                unsafe extern "system" fn vertex_attrib_i4usv(_index: GLuint, _v: *const GLushort) {
                    panic!("Unable to load vertex_attrib_i4usv")
                }
                let val = f(c"glVertexAttribI4usv");
                if val.is_null() {
                    vertex_attrib_i4usv
                } else {
                    mem::transmute(val)
                }
            },
            get_uniformuiv: unsafe {
                unsafe extern "system" fn get_uniformuiv(
                    _program: GLuint,
                    _location: GLint,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load get_uniformuiv")
                }
                let val = f(c"glGetUniformuiv");
                if val.is_null() {
                    get_uniformuiv
                } else {
                    mem::transmute(val)
                }
            },
            bind_frag_data_location: unsafe {
                unsafe extern "system" fn bind_frag_data_location(
                    _program: GLuint,
                    _color: GLuint,
                    _name: *const GLchar,
                ) {
                    panic!("Unable to load bind_frag_data_location")
                }
                let val = f(c"glBindFragDataLocation");
                if val.is_null() {
                    bind_frag_data_location
                } else {
                    mem::transmute(val)
                }
            },
            get_frag_data_location: unsafe {
                unsafe extern "system" fn get_frag_data_location(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load get_frag_data_location")
                }
                let val = f(c"glGetFragDataLocation");
                if val.is_null() {
                    get_frag_data_location
                } else {
                    mem::transmute(val)
                }
            },
            uniform1ui: unsafe {
                unsafe extern "system" fn uniform1ui(_location: GLint, _v0: GLuint) {
                    panic!("Unable to load uniform1ui")
                }
                let val = f(c"glUniform1ui");
                if val.is_null() {
                    uniform1ui
                } else {
                    mem::transmute(val)
                }
            },
            uniform2ui: unsafe {
                unsafe extern "system" fn uniform2ui(_location: GLint, _v0: GLuint, _v1: GLuint) {
                    panic!("Unable to load uniform2ui")
                }
                let val = f(c"glUniform2ui");
                if val.is_null() {
                    uniform2ui
                } else {
                    mem::transmute(val)
                }
            },
            uniform3ui: unsafe {
                unsafe extern "system" fn uniform3ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                ) {
                    panic!("Unable to load uniform3ui")
                }
                let val = f(c"glUniform3ui");
                if val.is_null() {
                    uniform3ui
                } else {
                    mem::transmute(val)
                }
            },
            uniform4ui: unsafe {
                unsafe extern "system" fn uniform4ui(
                    _location: GLint,
                    _v0: GLuint,
                    _v1: GLuint,
                    _v2: GLuint,
                    _v3: GLuint,
                ) {
                    panic!("Unable to load uniform4ui")
                }
                let val = f(c"glUniform4ui");
                if val.is_null() {
                    uniform4ui
                } else {
                    mem::transmute(val)
                }
            },
            uniform1uiv: unsafe {
                unsafe extern "system" fn uniform1uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load uniform1uiv")
                }
                let val = f(c"glUniform1uiv");
                if val.is_null() {
                    uniform1uiv
                } else {
                    mem::transmute(val)
                }
            },
            uniform2uiv: unsafe {
                unsafe extern "system" fn uniform2uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load uniform2uiv")
                }
                let val = f(c"glUniform2uiv");
                if val.is_null() {
                    uniform2uiv
                } else {
                    mem::transmute(val)
                }
            },
            uniform3uiv: unsafe {
                unsafe extern "system" fn uniform3uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load uniform3uiv")
                }
                let val = f(c"glUniform3uiv");
                if val.is_null() {
                    uniform3uiv
                } else {
                    mem::transmute(val)
                }
            },
            uniform4uiv: unsafe {
                unsafe extern "system" fn uniform4uiv(
                    _location: GLint,
                    _count: GLsizei,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load uniform4uiv")
                }
                let val = f(c"glUniform4uiv");
                if val.is_null() {
                    uniform4uiv
                } else {
                    mem::transmute(val)
                }
            },
            tex_parameter_iiv: unsafe {
                unsafe extern "system" fn tex_parameter_iiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLint,
                ) {
                    panic!("Unable to load tex_parameter_iiv")
                }
                let val = f(c"glTexParameterIiv");
                if val.is_null() {
                    tex_parameter_iiv
                } else {
                    mem::transmute(val)
                }
            },
            tex_parameter_iuiv: unsafe {
                unsafe extern "system" fn tex_parameter_iuiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *const GLuint,
                ) {
                    panic!("Unable to load tex_parameter_iuiv")
                }
                let val = f(c"glTexParameterIuiv");
                if val.is_null() {
                    tex_parameter_iuiv
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_parameter_iiv: unsafe {
                unsafe extern "system" fn get_tex_parameter_iiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_tex_parameter_iiv")
                }
                let val = f(c"glGetTexParameterIiv");
                if val.is_null() {
                    get_tex_parameter_iiv
                } else {
                    mem::transmute(val)
                }
            },
            get_tex_parameter_iuiv: unsafe {
                unsafe extern "system" fn get_tex_parameter_iuiv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load get_tex_parameter_iuiv")
                }
                let val = f(c"glGetTexParameterIuiv");
                if val.is_null() {
                    get_tex_parameter_iuiv
                } else {
                    mem::transmute(val)
                }
            },
            clear_bufferiv: unsafe {
                unsafe extern "system" fn clear_bufferiv(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _value: *const GLint,
                ) {
                    panic!("Unable to load clear_bufferiv")
                }
                let val = f(c"glClearBufferiv");
                if val.is_null() {
                    clear_bufferiv
                } else {
                    mem::transmute(val)
                }
            },
            clear_bufferuiv: unsafe {
                unsafe extern "system" fn clear_bufferuiv(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load clear_bufferuiv")
                }
                let val = f(c"glClearBufferuiv");
                if val.is_null() {
                    clear_bufferuiv
                } else {
                    mem::transmute(val)
                }
            },
            clear_bufferfv: unsafe {
                unsafe extern "system" fn clear_bufferfv(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load clear_bufferfv")
                }
                let val = f(c"glClearBufferfv");
                if val.is_null() {
                    clear_bufferfv
                } else {
                    mem::transmute(val)
                }
            },
            clear_bufferfi: unsafe {
                unsafe extern "system" fn clear_bufferfi(
                    _buffer: GLenum,
                    _drawbuffer: GLint,
                    _depth: GLfloat,
                    _stencil: GLint,
                ) {
                    panic!("Unable to load clear_bufferfi")
                }
                let val = f(c"glClearBufferfi");
                if val.is_null() {
                    clear_bufferfi
                } else {
                    mem::transmute(val)
                }
            },
            get_stringi: unsafe {
                unsafe extern "system" fn get_stringi(
                    _name: GLenum,
                    _index: GLuint,
                ) -> *const GLubyte {
                    panic!("Unable to load get_stringi")
                }
                let val = f(c"glGetStringi");
                if val.is_null() {
                    get_stringi
                } else {
                    mem::transmute(val)
                }
            },
            is_renderbuffer: unsafe {
                unsafe extern "system" fn is_renderbuffer(_renderbuffer: GLuint) -> GLboolean {
                    panic!("Unable to load is_renderbuffer")
                }
                let val = f(c"glIsRenderbuffer");
                if val.is_null() {
                    is_renderbuffer
                } else {
                    mem::transmute(val)
                }
            },
            bind_renderbuffer: unsafe {
                unsafe extern "system" fn bind_renderbuffer(
                    _target: GLenum,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load bind_renderbuffer")
                }
                let val = f(c"glBindRenderbuffer");
                if val.is_null() {
                    bind_renderbuffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_renderbuffers: unsafe {
                unsafe extern "system" fn delete_renderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *const GLuint,
                ) {
                    panic!("Unable to load delete_renderbuffers")
                }
                let val = f(c"glDeleteRenderbuffers");
                if val.is_null() {
                    delete_renderbuffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_renderbuffers: unsafe {
                unsafe extern "system" fn gen_renderbuffers(
                    _n: GLsizei,
                    _renderbuffers: *mut GLuint,
                ) {
                    panic!("Unable to load gen_renderbuffers")
                }
                let val = f(c"glGenRenderbuffers");
                if val.is_null() {
                    gen_renderbuffers
                } else {
                    mem::transmute(val)
                }
            },
            renderbuffer_storage: unsafe {
                unsafe extern "system" fn renderbuffer_storage(
                    _target: GLenum,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load renderbuffer_storage")
                }
                let val = f(c"glRenderbufferStorage");
                if val.is_null() {
                    renderbuffer_storage
                } else {
                    mem::transmute(val)
                }
            },
            get_renderbuffer_parameteriv: unsafe {
                unsafe extern "system" fn get_renderbuffer_parameteriv(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_renderbuffer_parameteriv")
                }
                let val = f(c"glGetRenderbufferParameteriv");
                if val.is_null() {
                    get_renderbuffer_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            is_framebuffer: unsafe {
                unsafe extern "system" fn is_framebuffer(_framebuffer: GLuint) -> GLboolean {
                    panic!("Unable to load is_framebuffer")
                }
                let val = f(c"glIsFramebuffer");
                if val.is_null() {
                    is_framebuffer
                } else {
                    mem::transmute(val)
                }
            },
            bind_framebuffer: unsafe {
                unsafe extern "system" fn bind_framebuffer(_target: GLenum, _framebuffer: GLuint) {
                    panic!("Unable to load bind_framebuffer")
                }
                let val = f(c"glBindFramebuffer");
                if val.is_null() {
                    bind_framebuffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_framebuffers: unsafe {
                unsafe extern "system" fn delete_framebuffers(
                    _n: GLsizei,
                    _framebuffers: *const GLuint,
                ) {
                    panic!("Unable to load delete_framebuffers")
                }
                let val = f(c"glDeleteFramebuffers");
                if val.is_null() {
                    delete_framebuffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_framebuffers: unsafe {
                unsafe extern "system" fn gen_framebuffers(
                    _n: GLsizei,
                    _framebuffers: *mut GLuint,
                ) {
                    panic!("Unable to load gen_framebuffers")
                }
                let val = f(c"glGenFramebuffers");
                if val.is_null() {
                    gen_framebuffers
                } else {
                    mem::transmute(val)
                }
            },
            check_framebuffer_status: unsafe {
                unsafe extern "system" fn check_framebuffer_status(_target: GLenum) -> GLenum {
                    panic!("Unable to load check_framebuffer_status")
                }
                let val = f(c"glCheckFramebufferStatus");
                if val.is_null() {
                    check_framebuffer_status
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_texture_1d: unsafe {
                unsafe extern "system" fn framebuffer_texture_1d(
                    _target: GLenum,
                    _attachment: GLenum,
                    _textarget: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load framebuffer_texture_1d")
                }
                let val = f(c"glFramebufferTexture1D");
                if val.is_null() {
                    framebuffer_texture_1d
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_texture_2d: unsafe {
                unsafe extern "system" fn framebuffer_texture_2d(
                    _target: GLenum,
                    _attachment: GLenum,
                    _textarget: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load framebuffer_texture_2d")
                }
                let val = f(c"glFramebufferTexture2D");
                if val.is_null() {
                    framebuffer_texture_2d
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_texture_3d: unsafe {
                unsafe extern "system" fn framebuffer_texture_3d(
                    _target: GLenum,
                    _attachment: GLenum,
                    _textarget: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                    _zoffset: GLint,
                ) {
                    panic!("Unable to load framebuffer_texture_3d")
                }
                let val = f(c"glFramebufferTexture3D");
                if val.is_null() {
                    framebuffer_texture_3d
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_renderbuffer: unsafe {
                unsafe extern "system" fn framebuffer_renderbuffer(
                    _target: GLenum,
                    _attachment: GLenum,
                    _renderbuffertarget: GLenum,
                    _renderbuffer: GLuint,
                ) {
                    panic!("Unable to load framebuffer_renderbuffer")
                }
                let val = f(c"glFramebufferRenderbuffer");
                if val.is_null() {
                    framebuffer_renderbuffer
                } else {
                    mem::transmute(val)
                }
            },
            get_framebuffer_attachment_parameteriv: unsafe {
                unsafe extern "system" fn get_framebuffer_attachment_parameteriv(
                    _target: GLenum,
                    _attachment: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_framebuffer_attachment_parameteriv")
                }
                let val = f(c"glGetFramebufferAttachmentParameteriv");
                if val.is_null() {
                    get_framebuffer_attachment_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            generate_mipmap: unsafe {
                unsafe extern "system" fn generate_mipmap(_target: GLenum) {
                    panic!("Unable to load generate_mipmap")
                }
                let val = f(c"glGenerateMipmap");
                if val.is_null() {
                    generate_mipmap
                } else {
                    mem::transmute(val)
                }
            },
            blit_framebuffer: unsafe {
                unsafe extern "system" fn blit_framebuffer(
                    _src_x0: GLint,
                    _src_y0: GLint,
                    _src_x1: GLint,
                    _src_y1: GLint,
                    _dst_x0: GLint,
                    _dst_y0: GLint,
                    _dst_x1: GLint,
                    _dst_y1: GLint,
                    _mask: GLbitfield,
                    _filter: GLenum,
                ) {
                    panic!("Unable to load blit_framebuffer")
                }
                let val = f(c"glBlitFramebuffer");
                if val.is_null() {
                    blit_framebuffer
                } else {
                    mem::transmute(val)
                }
            },
            renderbuffer_storage_multisample: unsafe {
                unsafe extern "system" fn renderbuffer_storage_multisample(
                    _target: GLenum,
                    _samples: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load renderbuffer_storage_multisample")
                }
                let val = f(c"glRenderbufferStorageMultisample");
                if val.is_null() {
                    renderbuffer_storage_multisample
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_texture_layer: unsafe {
                unsafe extern "system" fn framebuffer_texture_layer(
                    _target: GLenum,
                    _attachment: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                    _layer: GLint,
                ) {
                    panic!("Unable to load framebuffer_texture_layer")
                }
                let val = f(c"glFramebufferTextureLayer");
                if val.is_null() {
                    framebuffer_texture_layer
                } else {
                    mem::transmute(val)
                }
            },
            map_buffer_range: unsafe {
                unsafe extern "system" fn map_buffer_range(
                    _target: GLenum,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                    _access: GLbitfield,
                ) -> *mut c_void {
                    panic!("Unable to load map_buffer_range")
                }
                let val = f(c"glMapBufferRange");
                if val.is_null() {
                    map_buffer_range
                } else {
                    mem::transmute(val)
                }
            },
            flush_mapped_buffer_range: unsafe {
                unsafe extern "system" fn flush_mapped_buffer_range(
                    _target: GLenum,
                    _offset: GLintptr,
                    _length: GLsizeiptr,
                ) {
                    panic!("Unable to load flush_mapped_buffer_range")
                }
                let val = f(c"glFlushMappedBufferRange");
                if val.is_null() {
                    flush_mapped_buffer_range
                } else {
                    mem::transmute(val)
                }
            },
            bind_vertex_array: unsafe {
                unsafe extern "system" fn bind_vertex_array(_array: GLuint) {
                    panic!("Unable to load bind_vertex_array")
                }
                let val = f(c"glBindVertexArray");
                if val.is_null() {
                    bind_vertex_array
                } else {
                    mem::transmute(val)
                }
            },
            delete_vertex_arrays: unsafe {
                unsafe extern "system" fn delete_vertex_arrays(
                    _n: GLsizei,
                    _arrays: *const GLuint,
                ) {
                    panic!("Unable to load delete_vertex_arrays")
                }
                let val = f(c"glDeleteVertexArrays");
                if val.is_null() {
                    delete_vertex_arrays
                } else {
                    mem::transmute(val)
                }
            },
            gen_vertex_arrays: unsafe {
                unsafe extern "system" fn gen_vertex_arrays(_n: GLsizei, _arrays: *mut GLuint) {
                    panic!("Unable to load gen_vertex_arrays")
                }
                let val = f(c"glGenVertexArrays");
                if val.is_null() {
                    gen_vertex_arrays
                } else {
                    mem::transmute(val)
                }
            },
            is_vertex_array: unsafe {
                unsafe extern "system" fn is_vertex_array(_array: GLuint) -> GLboolean {
                    panic!("Unable to load is_vertex_array")
                }
                let val = f(c"glIsVertexArray");
                if val.is_null() {
                    is_vertex_array
                } else {
                    mem::transmute(val)
                }
            },
            draw_arrays_instanced: unsafe {
                unsafe extern "system" fn draw_arrays_instanced(
                    _mode: GLenum,
                    _first: GLint,
                    _count: GLsizei,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load draw_arrays_instanced")
                }
                let val = f(c"glDrawArraysInstanced");
                if val.is_null() {
                    draw_arrays_instanced
                } else {
                    mem::transmute(val)
                }
            },
            draw_elements_instanced: unsafe {
                unsafe extern "system" fn draw_elements_instanced(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const c_void,
                    _instancecount: GLsizei,
                ) {
                    panic!("Unable to load draw_elements_instanced")
                }
                let val = f(c"glDrawElementsInstanced");
                if val.is_null() {
                    draw_elements_instanced
                } else {
                    mem::transmute(val)
                }
            },
            tex_buffer: unsafe {
                unsafe extern "system" fn tex_buffer(
                    _target: GLenum,
                    _internalformat: GLenum,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load tex_buffer")
                }
                let val = f(c"glTexBuffer");
                if val.is_null() {
                    tex_buffer
                } else {
                    mem::transmute(val)
                }
            },
            primitive_restart_index: unsafe {
                unsafe extern "system" fn primitive_restart_index(_index: GLuint) {
                    panic!("Unable to load primitive_restart_index")
                }
                let val = f(c"glPrimitiveRestartIndex");
                if val.is_null() {
                    primitive_restart_index
                } else {
                    mem::transmute(val)
                }
            },
            copy_buffer_sub_data: unsafe {
                unsafe extern "system" fn copy_buffer_sub_data(
                    _read_target: GLenum,
                    _write_target: GLenum,
                    _read_offset: GLintptr,
                    _write_offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load copy_buffer_sub_data")
                }
                let val = f(c"glCopyBufferSubData");
                if val.is_null() {
                    copy_buffer_sub_data
                } else {
                    mem::transmute(val)
                }
            },
            get_uniform_indices: unsafe {
                unsafe extern "system" fn get_uniform_indices(
                    _program: GLuint,
                    _uniform_count: GLsizei,
                    _uniform_names: *const *const GLchar,
                    _uniform_indices: *mut GLuint,
                ) {
                    panic!("Unable to load get_uniform_indices")
                }
                let val = f(c"glGetUniformIndices");
                if val.is_null() {
                    get_uniform_indices
                } else {
                    mem::transmute(val)
                }
            },
            get_active_uniformsiv: unsafe {
                unsafe extern "system" fn get_active_uniformsiv(
                    _program: GLuint,
                    _uniform_count: GLsizei,
                    _uniform_indices: *const GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_active_uniformsiv")
                }
                let val = f(c"glGetActiveUniformsiv");
                if val.is_null() {
                    get_active_uniformsiv
                } else {
                    mem::transmute(val)
                }
            },
            get_active_uniform_name: unsafe {
                unsafe extern "system" fn get_active_uniform_name(
                    _program: GLuint,
                    _uniform_index: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _uniform_name: *mut GLchar,
                ) {
                    panic!("Unable to load get_active_uniform_name")
                }
                let val = f(c"glGetActiveUniformName");
                if val.is_null() {
                    get_active_uniform_name
                } else {
                    mem::transmute(val)
                }
            },
            get_uniform_block_index: unsafe {
                unsafe extern "system" fn get_uniform_block_index(
                    _program: GLuint,
                    _uniform_block_name: *const GLchar,
                ) -> GLuint {
                    panic!("Unable to load get_uniform_block_index")
                }
                let val = f(c"glGetUniformBlockIndex");
                if val.is_null() {
                    get_uniform_block_index
                } else {
                    mem::transmute(val)
                }
            },
            get_active_uniform_blockiv: unsafe {
                unsafe extern "system" fn get_active_uniform_blockiv(
                    _program: GLuint,
                    _uniform_block_index: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_active_uniform_blockiv")
                }
                let val = f(c"glGetActiveUniformBlockiv");
                if val.is_null() {
                    get_active_uniform_blockiv
                } else {
                    mem::transmute(val)
                }
            },
            get_active_uniform_block_name: unsafe {
                unsafe extern "system" fn get_active_uniform_block_name(
                    _program: GLuint,
                    _uniform_block_index: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _uniform_block_name: *mut GLchar,
                ) {
                    panic!("Unable to load get_active_uniform_block_name")
                }
                let val = f(c"glGetActiveUniformBlockName");
                if val.is_null() {
                    get_active_uniform_block_name
                } else {
                    mem::transmute(val)
                }
            },
            uniform_block_binding: unsafe {
                unsafe extern "system" fn uniform_block_binding(
                    _program: GLuint,
                    _uniform_block_index: GLuint,
                    _uniform_block_binding: GLuint,
                ) {
                    panic!("Unable to load uniform_block_binding")
                }
                let val = f(c"glUniformBlockBinding");
                if val.is_null() {
                    uniform_block_binding
                } else {
                    mem::transmute(val)
                }
            },
            draw_elements_base_vertex: unsafe {
                unsafe extern "system" fn draw_elements_base_vertex(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const c_void,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load draw_elements_base_vertex")
                }
                let val = f(c"glDrawElementsBaseVertex");
                if val.is_null() {
                    draw_elements_base_vertex
                } else {
                    mem::transmute(val)
                }
            },
            draw_range_elements_base_vertex: unsafe {
                unsafe extern "system" fn draw_range_elements_base_vertex(
                    _mode: GLenum,
                    _start: GLuint,
                    _end: GLuint,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const c_void,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load draw_range_elements_base_vertex")
                }
                let val = f(c"glDrawRangeElementsBaseVertex");
                if val.is_null() {
                    draw_range_elements_base_vertex
                } else {
                    mem::transmute(val)
                }
            },
            draw_elements_instanced_base_vertex: unsafe {
                unsafe extern "system" fn draw_elements_instanced_base_vertex(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const c_void,
                    _instancecount: GLsizei,
                    _basevertex: GLint,
                ) {
                    panic!("Unable to load draw_elements_instanced_base_vertex")
                }
                let val = f(c"glDrawElementsInstancedBaseVertex");
                if val.is_null() {
                    draw_elements_instanced_base_vertex
                } else {
                    mem::transmute(val)
                }
            },
            multi_draw_elements_base_vertex: unsafe {
                unsafe extern "system" fn multi_draw_elements_base_vertex(
                    _mode: GLenum,
                    _count: *const GLsizei,
                    _type: GLenum,
                    _indices: *const *const c_void,
                    _drawcount: GLsizei,
                    _basevertex: *const GLint,
                ) {
                    panic!("Unable to load multi_draw_elements_base_vertex")
                }
                let val = f(c"glMultiDrawElementsBaseVertex");
                if val.is_null() {
                    multi_draw_elements_base_vertex
                } else {
                    mem::transmute(val)
                }
            },
            provoking_vertex: unsafe {
                unsafe extern "system" fn provoking_vertex(_mode: GLenum) {
                    panic!("Unable to load provoking_vertex")
                }
                let val = f(c"glProvokingVertex");
                if val.is_null() {
                    provoking_vertex
                } else {
                    mem::transmute(val)
                }
            },
            fence_sync: unsafe {
                unsafe extern "system" fn fence_sync(
                    _condition: GLenum,
                    _flags: GLbitfield,
                ) -> GLsync {
                    panic!("Unable to load fence_sync")
                }
                let val = f(c"glFenceSync");
                if val.is_null() {
                    fence_sync
                } else {
                    mem::transmute(val)
                }
            },
            is_sync: unsafe {
                unsafe extern "system" fn is_sync(_sync: GLsync) -> GLboolean {
                    panic!("Unable to load is_sync")
                }
                let val = f(c"glIsSync");
                if val.is_null() {
                    is_sync
                } else {
                    mem::transmute(val)
                }
            },
            delete_sync: unsafe {
                unsafe extern "system" fn delete_sync(_sync: GLsync) {
                    panic!("Unable to load delete_sync")
                }
                let val = f(c"glDeleteSync");
                if val.is_null() {
                    delete_sync
                } else {
                    mem::transmute(val)
                }
            },
            client_wait_sync: unsafe {
                unsafe extern "system" fn client_wait_sync(
                    _sync: GLsync,
                    _flags: GLbitfield,
                    _timeout: GLuint64,
                ) -> GLenum {
                    panic!("Unable to load client_wait_sync")
                }
                let val = f(c"glClientWaitSync");
                if val.is_null() {
                    client_wait_sync
                } else {
                    mem::transmute(val)
                }
            },
            wait_sync: unsafe {
                unsafe extern "system" fn wait_sync(
                    _sync: GLsync,
                    _flags: GLbitfield,
                    _timeout: GLuint64,
                ) {
                    panic!("Unable to load wait_sync")
                }
                let val = f(c"glWaitSync");
                if val.is_null() {
                    wait_sync
                } else {
                    mem::transmute(val)
                }
            },
            get_integer64v: unsafe {
                unsafe extern "system" fn get_integer64v(_pname: GLenum, _data: *mut GLint64) {
                    panic!("Unable to load get_integer64v")
                }
                let val = f(c"glGetInteger64v");
                if val.is_null() {
                    get_integer64v
                } else {
                    mem::transmute(val)
                }
            },
            get_synciv: unsafe {
                unsafe extern "system" fn get_synciv(
                    _sync: GLsync,
                    _pname: GLenum,
                    _count: GLsizei,
                    _length: *mut GLsizei,
                    _values: *mut GLint,
                ) {
                    panic!("Unable to load get_synciv")
                }
                let val = f(c"glGetSynciv");
                if val.is_null() {
                    get_synciv
                } else {
                    mem::transmute(val)
                }
            },
            get_integer64i_v: unsafe {
                unsafe extern "system" fn get_integer64i_v(
                    _target: GLenum,
                    _index: GLuint,
                    _data: *mut GLint64,
                ) {
                    panic!("Unable to load get_integer64i_v")
                }
                let val = f(c"glGetInteger64i_v");
                if val.is_null() {
                    get_integer64i_v
                } else {
                    mem::transmute(val)
                }
            },
            get_buffer_parameteri64v: unsafe {
                unsafe extern "system" fn get_buffer_parameteri64v(
                    _target: GLenum,
                    _pname: GLenum,
                    _params: *mut GLint64,
                ) {
                    panic!("Unable to load get_buffer_parameteri64v")
                }
                let val = f(c"glGetBufferParameteri64v");
                if val.is_null() {
                    get_buffer_parameteri64v
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_texture: unsafe {
                unsafe extern "system" fn framebuffer_texture(
                    _target: GLenum,
                    _attachment: GLenum,
                    _texture: GLuint,
                    _level: GLint,
                ) {
                    panic!("Unable to load framebuffer_texture")
                }
                let val = f(c"glFramebufferTexture");
                if val.is_null() {
                    framebuffer_texture
                } else {
                    mem::transmute(val)
                }
            },
            tex_image_2dmultisample: unsafe {
                unsafe extern "system" fn tex_image_2dmultisample(
                    _target: GLenum,
                    _samples: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _fixedsamplelocations: GLboolean,
                ) {
                    panic!("Unable to load tex_image_2dmultisample")
                }
                let val = f(c"glTexImage2DMultisample");
                if val.is_null() {
                    tex_image_2dmultisample
                } else {
                    mem::transmute(val)
                }
            },
            tex_image_3dmultisample: unsafe {
                unsafe extern "system" fn tex_image_3dmultisample(
                    _target: GLenum,
                    _samples: GLsizei,
                    _internalformat: GLenum,
                    _width: GLsizei,
                    _height: GLsizei,
                    _depth: GLsizei,
                    _fixedsamplelocations: GLboolean,
                ) {
                    panic!("Unable to load tex_image_3dmultisample")
                }
                let val = f(c"glTexImage3DMultisample");
                if val.is_null() {
                    tex_image_3dmultisample
                } else {
                    mem::transmute(val)
                }
            },
            get_multisamplefv: unsafe {
                unsafe extern "system" fn get_multisamplefv(
                    _pname: GLenum,
                    _index: GLuint,
                    _val: *mut GLfloat,
                ) {
                    panic!("Unable to load get_multisamplefv")
                }
                let val = f(c"glGetMultisamplefv");
                if val.is_null() {
                    get_multisamplefv
                } else {
                    mem::transmute(val)
                }
            },
            sample_maski: unsafe {
                unsafe extern "system" fn sample_maski(_mask_number: GLuint, _mask: GLbitfield) {
                    panic!("Unable to load sample_maski")
                }
                let val = f(c"glSampleMaski");
                if val.is_null() {
                    sample_maski
                } else {
                    mem::transmute(val)
                }
            },
            bind_frag_data_location_indexed: unsafe {
                unsafe extern "system" fn bind_frag_data_location_indexed(
                    _program: GLuint,
                    _color_number: GLuint,
                    _index: GLuint,
                    _name: *const GLchar,
                ) {
                    panic!("Unable to load bind_frag_data_location_indexed")
                }
                let val = f(c"glBindFragDataLocationIndexed");
                if val.is_null() {
                    bind_frag_data_location_indexed
                } else {
                    mem::transmute(val)
                }
            },
            get_frag_data_index: unsafe {
                unsafe extern "system" fn get_frag_data_index(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load get_frag_data_index")
                }
                let val = f(c"glGetFragDataIndex");
                if val.is_null() {
                    get_frag_data_index
                } else {
                    mem::transmute(val)
                }
            },
            gen_samplers: unsafe {
                unsafe extern "system" fn gen_samplers(_count: GLsizei, _samplers: *mut GLuint) {
                    panic!("Unable to load gen_samplers")
                }
                let val = f(c"glGenSamplers");
                if val.is_null() {
                    gen_samplers
                } else {
                    mem::transmute(val)
                }
            },
            delete_samplers: unsafe {
                unsafe extern "system" fn delete_samplers(
                    _count: GLsizei,
                    _samplers: *const GLuint,
                ) {
                    panic!("Unable to load delete_samplers")
                }
                let val = f(c"glDeleteSamplers");
                if val.is_null() {
                    delete_samplers
                } else {
                    mem::transmute(val)
                }
            },
            is_sampler: unsafe {
                unsafe extern "system" fn is_sampler(_sampler: GLuint) -> GLboolean {
                    panic!("Unable to load is_sampler")
                }
                let val = f(c"glIsSampler");
                if val.is_null() {
                    is_sampler
                } else {
                    mem::transmute(val)
                }
            },
            bind_sampler: unsafe {
                unsafe extern "system" fn bind_sampler(_unit: GLuint, _sampler: GLuint) {
                    panic!("Unable to load bind_sampler")
                }
                let val = f(c"glBindSampler");
                if val.is_null() {
                    bind_sampler
                } else {
                    mem::transmute(val)
                }
            },
            sampler_parameteri: unsafe {
                unsafe extern "system" fn sampler_parameteri(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load sampler_parameteri")
                }
                let val = f(c"glSamplerParameteri");
                if val.is_null() {
                    sampler_parameteri
                } else {
                    mem::transmute(val)
                }
            },
            sampler_parameteriv: unsafe {
                unsafe extern "system" fn sampler_parameteriv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLint,
                ) {
                    panic!("Unable to load sampler_parameteriv")
                }
                let val = f(c"glSamplerParameteriv");
                if val.is_null() {
                    sampler_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            sampler_parameterf: unsafe {
                unsafe extern "system" fn sampler_parameterf(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: GLfloat,
                ) {
                    panic!("Unable to load sampler_parameterf")
                }
                let val = f(c"glSamplerParameterf");
                if val.is_null() {
                    sampler_parameterf
                } else {
                    mem::transmute(val)
                }
            },
            sampler_parameterfv: unsafe {
                unsafe extern "system" fn sampler_parameterfv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLfloat,
                ) {
                    panic!("Unable to load sampler_parameterfv")
                }
                let val = f(c"glSamplerParameterfv");
                if val.is_null() {
                    sampler_parameterfv
                } else {
                    mem::transmute(val)
                }
            },
            sampler_parameter_iiv: unsafe {
                unsafe extern "system" fn sampler_parameter_iiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLint,
                ) {
                    panic!("Unable to load sampler_parameter_iiv")
                }
                let val = f(c"glSamplerParameterIiv");
                if val.is_null() {
                    sampler_parameter_iiv
                } else {
                    mem::transmute(val)
                }
            },
            sampler_parameter_iuiv: unsafe {
                unsafe extern "system" fn sampler_parameter_iuiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _param: *const GLuint,
                ) {
                    panic!("Unable to load sampler_parameter_iuiv")
                }
                let val = f(c"glSamplerParameterIuiv");
                if val.is_null() {
                    sampler_parameter_iuiv
                } else {
                    mem::transmute(val)
                }
            },
            get_sampler_parameteriv: unsafe {
                unsafe extern "system" fn get_sampler_parameteriv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_sampler_parameteriv")
                }
                let val = f(c"glGetSamplerParameteriv");
                if val.is_null() {
                    get_sampler_parameteriv
                } else {
                    mem::transmute(val)
                }
            },
            get_sampler_parameter_iiv: unsafe {
                unsafe extern "system" fn get_sampler_parameter_iiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_sampler_parameter_iiv")
                }
                let val = f(c"glGetSamplerParameterIiv");
                if val.is_null() {
                    get_sampler_parameter_iiv
                } else {
                    mem::transmute(val)
                }
            },
            get_sampler_parameterfv: unsafe {
                unsafe extern "system" fn get_sampler_parameterfv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLfloat,
                ) {
                    panic!("Unable to load get_sampler_parameterfv")
                }
                let val = f(c"glGetSamplerParameterfv");
                if val.is_null() {
                    get_sampler_parameterfv
                } else {
                    mem::transmute(val)
                }
            },
            get_sampler_parameter_iuiv: unsafe {
                unsafe extern "system" fn get_sampler_parameter_iuiv(
                    _sampler: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint,
                ) {
                    panic!("Unable to load get_sampler_parameter_iuiv")
                }
                let val = f(c"glGetSamplerParameterIuiv");
                if val.is_null() {
                    get_sampler_parameter_iuiv
                } else {
                    mem::transmute(val)
                }
            },
            query_counter: unsafe {
                unsafe extern "system" fn query_counter(_id: GLuint, _target: GLenum) {
                    panic!("Unable to load query_counter")
                }
                let val = f(c"glQueryCounter");
                if val.is_null() {
                    query_counter
                } else {
                    mem::transmute(val)
                }
            },
            get_query_objecti64v: unsafe {
                unsafe extern "system" fn get_query_objecti64v(
                    _id: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint64,
                ) {
                    panic!("Unable to load get_query_objecti64v")
                }
                let val = f(c"glGetQueryObjecti64v");
                if val.is_null() {
                    get_query_objecti64v
                } else {
                    mem::transmute(val)
                }
            },
            get_query_objectui64v: unsafe {
                unsafe extern "system" fn get_query_objectui64v(
                    _id: GLuint,
                    _pname: GLenum,
                    _params: *mut GLuint64,
                ) {
                    panic!("Unable to load get_query_objectui64v")
                }
                let val = f(c"glGetQueryObjectui64v");
                if val.is_null() {
                    get_query_objectui64v
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_divisor: unsafe {
                unsafe extern "system" fn vertex_attrib_divisor(_index: GLuint, _divisor: GLuint) {
                    panic!("Unable to load vertex_attrib_divisor")
                }
                let val = f(c"glVertexAttribDivisor");
                if val.is_null() {
                    vertex_attrib_divisor
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p1ui: unsafe {
                unsafe extern "system" fn vertex_attrib_p1ui(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p1ui")
                }
                let val = f(c"glVertexAttribP1ui");
                if val.is_null() {
                    vertex_attrib_p1ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p1uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_p1uiv(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p1uiv")
                }
                let val = f(c"glVertexAttribP1uiv");
                if val.is_null() {
                    vertex_attrib_p1uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p2ui: unsafe {
                unsafe extern "system" fn vertex_attrib_p2ui(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p2ui")
                }
                let val = f(c"glVertexAttribP2ui");
                if val.is_null() {
                    vertex_attrib_p2ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p2uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_p2uiv(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p2uiv")
                }
                let val = f(c"glVertexAttribP2uiv");
                if val.is_null() {
                    vertex_attrib_p2uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p3ui: unsafe {
                unsafe extern "system" fn vertex_attrib_p3ui(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p3ui")
                }
                let val = f(c"glVertexAttribP3ui");
                if val.is_null() {
                    vertex_attrib_p3ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p3uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_p3uiv(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p3uiv")
                }
                let val = f(c"glVertexAttribP3uiv");
                if val.is_null() {
                    vertex_attrib_p3uiv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p4ui: unsafe {
                unsafe extern "system" fn vertex_attrib_p4ui(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p4ui")
                }
                let val = f(c"glVertexAttribP4ui");
                if val.is_null() {
                    vertex_attrib_p4ui
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_p4uiv: unsafe {
                unsafe extern "system" fn vertex_attrib_p4uiv(
                    _index: GLuint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _value: *const GLuint,
                ) {
                    panic!("Unable to load vertex_attrib_p4uiv")
                }
                let val = f(c"glVertexAttribP4uiv");
                if val.is_null() {
                    vertex_attrib_p4uiv
                } else {
                    mem::transmute(val)
                }
            },
        }
    }
    #[inline]
    pub unsafe fn cull_face(&self, mode: GLenum) {
        unsafe { (self.cull_face)(mode) }
    }
    #[inline]
    pub unsafe fn front_face(&self, mode: GLenum) {
        unsafe { (self.front_face)(mode) }
    }
    #[inline]
    pub unsafe fn hint(&self, target: GLenum, mode: GLenum) {
        unsafe { (self.hint)(target, mode) }
    }
    #[inline]
    pub unsafe fn line_width(&self, width: GLfloat) {
        unsafe { (self.line_width)(width) }
    }
    #[inline]
    pub unsafe fn point_size(&self, size: GLfloat) {
        unsafe { (self.point_size)(size) }
    }
    #[inline]
    pub unsafe fn polygon_mode(&self, face: GLenum, mode: GLenum) {
        unsafe { (self.polygon_mode)(face, mode) }
    }
    #[inline]
    pub unsafe fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { (self.scissor)(x, y, width, height) }
    }
    #[inline]
    pub unsafe fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        unsafe { (self.tex_parameterf)(target, pname, param) }
    }
    #[inline]
    pub unsafe fn tex_parameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        unsafe { (self.tex_parameterfv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        unsafe { (self.tex_parameteri)(target, pname, param) }
    }
    #[inline]
    pub unsafe fn tex_parameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        unsafe { (self.tex_parameteriv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn tex_image_1d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void,
    ) {
        unsafe {
            (self.tex_image_1d)(
                target,
                level,
                internalformat,
                width,
                border,
                format,
                type_,
                pixels,
            )
        }
    }
    #[inline]
    pub unsafe fn tex_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void,
    ) {
        unsafe {
            (self.tex_image_2d)(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                type_,
                pixels,
            )
        }
    }
    #[inline]
    pub unsafe fn draw_buffer(&self, buf: GLenum) {
        unsafe { (self.draw_buffer)(buf) }
    }
    #[inline]
    pub unsafe fn clear(&self, mask: GLbitfield) {
        unsafe { (self.clear)(mask) }
    }
    #[inline]
    pub unsafe fn clear_color(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        unsafe { (self.clear_color)(red, green, blue, alpha) }
    }
    #[inline]
    pub unsafe fn clear_stencil(&self, s: GLint) {
        unsafe { (self.clear_stencil)(s) }
    }
    #[inline]
    pub unsafe fn clear_depth(&self, depth: GLdouble) {
        unsafe { (self.clear_depth)(depth) }
    }
    #[inline]
    pub unsafe fn stencil_mask(&self, mask: GLuint) {
        unsafe { (self.stencil_mask)(mask) }
    }
    #[inline]
    pub unsafe fn color_mask(
        &self,
        red: GLboolean,
        green: GLboolean,
        blue: GLboolean,
        alpha: GLboolean,
    ) {
        unsafe { (self.color_mask)(red, green, blue, alpha) }
    }
    #[inline]
    pub unsafe fn depth_mask(&self, flag: GLboolean) {
        unsafe { (self.depth_mask)(flag) }
    }
    #[inline]
    pub unsafe fn disable(&self, cap: GLenum) {
        unsafe { (self.disable)(cap) }
    }
    #[inline]
    pub unsafe fn enable(&self, cap: GLenum) {
        unsafe { (self.enable)(cap) }
    }
    #[inline]
    pub unsafe fn finish(&self) {
        unsafe { (self.finish)() }
    }
    #[inline]
    pub unsafe fn flush(&self) {
        unsafe { (self.flush)() }
    }
    #[inline]
    pub unsafe fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        unsafe { (self.blend_func)(sfactor, dfactor) }
    }
    #[inline]
    pub unsafe fn logic_op(&self, opcode: GLenum) {
        unsafe { (self.logic_op)(opcode) }
    }
    #[inline]
    pub unsafe fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        unsafe { (self.stencil_func)(func, ref_, mask) }
    }
    #[inline]
    pub unsafe fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        unsafe { (self.stencil_op)(fail, zfail, zpass) }
    }
    #[inline]
    pub unsafe fn depth_func(&self, func: GLenum) {
        unsafe { (self.depth_func)(func) }
    }
    #[inline]
    pub unsafe fn pixel_storef(&self, pname: GLenum, param: GLfloat) {
        unsafe { (self.pixel_storef)(pname, param) }
    }
    #[inline]
    pub unsafe fn pixel_storei(&self, pname: GLenum, param: GLint) {
        unsafe { (self.pixel_storei)(pname, param) }
    }
    #[inline]
    pub unsafe fn read_buffer(&self, src: GLenum) {
        unsafe { (self.read_buffer)(src) }
    }
    #[inline]
    pub unsafe fn read_pixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut c_void,
    ) {
        unsafe { (self.read_pixels)(x, y, width, height, format, type_, pixels) }
    }
    #[inline]
    pub unsafe fn get_booleanv(&self, pname: GLenum, data: *mut GLboolean) {
        unsafe { (self.get_booleanv)(pname, data) }
    }
    #[inline]
    pub unsafe fn get_doublev(&self, pname: GLenum, data: *mut GLdouble) {
        unsafe { (self.get_doublev)(pname, data) }
    }
    #[inline]
    pub unsafe fn get_error(&self) -> GLenum {
        unsafe { (self.get_error)() }
    }
    #[inline]
    pub unsafe fn get_floatv(&self, pname: GLenum, data: *mut GLfloat) {
        unsafe { (self.get_floatv)(pname, data) }
    }
    #[inline]
    pub unsafe fn get_integerv(&self, pname: GLenum, data: *mut GLint) {
        unsafe { (self.get_integerv)(pname, data) }
    }
    #[inline]
    pub unsafe fn get_string(&self, name: GLenum) -> *const GLubyte {
        unsafe { (self.get_string)(name) }
    }
    #[inline]
    pub unsafe fn get_tex_image(
        &self,
        target: GLenum,
        level: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *mut c_void,
    ) {
        unsafe { (self.get_tex_image)(target, level, format, type_, pixels) }
    }
    #[inline]
    pub unsafe fn get_tex_parameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
        unsafe { (self.get_tex_parameterfv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn get_tex_parameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_tex_parameteriv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn get_tex_level_parameterfv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        unsafe { (self.get_tex_level_parameterfv)(target, level, pname, params) }
    }
    #[inline]
    pub unsafe fn get_tex_level_parameteriv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe { (self.get_tex_level_parameteriv)(target, level, pname, params) }
    }
    #[inline]
    pub unsafe fn is_enabled(&self, cap: GLenum) -> GLboolean {
        unsafe { (self.is_enabled)(cap) }
    }
    #[inline]
    pub unsafe fn depth_range(&self, n: GLdouble, f: GLdouble) {
        unsafe { (self.depth_range)(n, f) }
    }
    #[inline]
    pub unsafe fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { (self.viewport)(x, y, width, height) }
    }
    #[inline]
    pub unsafe fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        unsafe { (self.draw_arrays)(mode, first, count) }
    }
    #[inline]
    pub unsafe fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
    ) {
        unsafe { (self.draw_elements)(mode, count, type_, indices) }
    }
    #[inline]
    pub unsafe fn get_pointerv(&self, pname: GLenum, params: *mut *mut c_void) {
        unsafe { (self.get_pointerv)(pname, params) }
    }
    #[inline]
    pub unsafe fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        unsafe { (self.polygon_offset)(factor, units) }
    }
    #[inline]
    pub unsafe fn copy_tex_image_1d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        border: GLint,
    ) {
        unsafe { (self.copy_tex_image_1d)(target, level, internalformat, x, y, width, border) }
    }
    #[inline]
    pub unsafe fn copy_tex_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        unsafe {
            (self.copy_tex_image_2d)(target, level, internalformat, x, y, width, height, border)
        }
    }
    #[inline]
    pub unsafe fn copy_tex_sub_image_1d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
    ) {
        unsafe { (self.copy_tex_sub_image_1d)(target, level, xoffset, x, y, width) }
    }
    #[inline]
    pub unsafe fn copy_tex_sub_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            (self.copy_tex_sub_image_2d)(target, level, xoffset, yoffset, x, y, width, height)
        }
    }
    #[inline]
    pub unsafe fn tex_sub_image_1d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void,
    ) {
        unsafe { (self.tex_sub_image_1d)(target, level, xoffset, width, format, type_, pixels) }
    }
    #[inline]
    pub unsafe fn tex_sub_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void,
    ) {
        unsafe {
            (self.tex_sub_image_2d)(
                target, level, xoffset, yoffset, width, height, format, type_, pixels,
            )
        }
    }
    #[inline]
    pub unsafe fn bind_texture(&self, target: GLenum, texture: GLuint) {
        unsafe { (self.bind_texture)(target, texture) }
    }
    #[inline]
    pub unsafe fn delete_textures(&self, n: GLsizei, textures: *const GLuint) {
        unsafe { (self.delete_textures)(n, textures) }
    }
    #[inline]
    pub unsafe fn gen_textures(&self, n: GLsizei, textures: *mut GLuint) {
        unsafe { (self.gen_textures)(n, textures) }
    }
    #[inline]
    pub unsafe fn is_texture(&self, texture: GLuint) -> GLboolean {
        unsafe { (self.is_texture)(texture) }
    }
    #[inline]
    pub unsafe fn draw_range_elements(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
    ) {
        unsafe { (self.draw_range_elements)(mode, start, end, count, type_, indices) }
    }
    #[inline]
    pub unsafe fn tex_image_3d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void,
    ) {
        unsafe {
            (self.tex_image_3d)(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                type_,
                pixels,
            )
        }
    }
    #[inline]
    pub unsafe fn tex_sub_image_3d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void,
    ) {
        unsafe {
            (self.tex_sub_image_3d)(
                target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_,
                pixels,
            )
        }
    }
    #[inline]
    pub unsafe fn copy_tex_sub_image_3d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            (self.copy_tex_sub_image_3d)(
                target, level, xoffset, yoffset, zoffset, x, y, width, height,
            )
        }
    }
    #[inline]
    pub unsafe fn active_texture(&self, texture: GLenum) {
        unsafe { (self.active_texture)(texture) }
    }
    #[inline]
    pub unsafe fn sample_coverage(&self, value: GLfloat, invert: GLboolean) {
        unsafe { (self.sample_coverage)(value, invert) }
    }
    #[inline]
    pub unsafe fn compressed_tex_image_3d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const c_void,
    ) {
        unsafe {
            (self.compressed_tex_image_3d)(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                image_size,
                data,
            )
        }
    }
    #[inline]
    pub unsafe fn compressed_tex_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const c_void,
    ) {
        unsafe {
            (self.compressed_tex_image_2d)(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                image_size,
                data,
            )
        }
    }
    #[inline]
    pub unsafe fn compressed_tex_image_1d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const c_void,
    ) {
        unsafe {
            (self.compressed_tex_image_1d)(
                target,
                level,
                internalformat,
                width,
                border,
                image_size,
                data,
            )
        }
    }
    #[inline]
    pub unsafe fn compressed_tex_sub_image_3d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const c_void,
    ) {
        unsafe {
            (self.compressed_tex_sub_image_3d)(
                target, level, xoffset, yoffset, zoffset, width, height, depth, format, image_size,
                data,
            )
        }
    }
    #[inline]
    pub unsafe fn compressed_tex_sub_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const c_void,
    ) {
        unsafe {
            (self.compressed_tex_sub_image_2d)(
                target, level, xoffset, yoffset, width, height, format, image_size, data,
            )
        }
    }
    #[inline]
    pub unsafe fn compressed_tex_sub_image_1d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        data: *const c_void,
    ) {
        unsafe {
            (self.compressed_tex_sub_image_1d)(
                target, level, xoffset, width, format, image_size, data,
            )
        }
    }
    #[inline]
    pub unsafe fn get_compressed_tex_image(&self, target: GLenum, level: GLint, img: *mut c_void) {
        unsafe { (self.get_compressed_tex_image)(target, level, img) }
    }
    #[inline]
    pub unsafe fn blend_func_separate(
        &self,
        sfactor_rgb: GLenum,
        dfactor_rgb: GLenum,
        sfactor_alpha: GLenum,
        dfactor_alpha: GLenum,
    ) {
        unsafe {
            (self.blend_func_separate)(sfactor_rgb, dfactor_rgb, sfactor_alpha, dfactor_alpha)
        }
    }
    #[inline]
    pub unsafe fn multi_draw_arrays(
        &self,
        mode: GLenum,
        first: *const GLint,
        count: *const GLsizei,
        drawcount: GLsizei,
    ) {
        unsafe { (self.multi_draw_arrays)(mode, first, count, drawcount) }
    }
    #[inline]
    pub unsafe fn multi_draw_elements(
        &self,
        mode: GLenum,
        count: *const GLsizei,
        type_: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei,
    ) {
        unsafe { (self.multi_draw_elements)(mode, count, type_, indices, drawcount) }
    }
    #[inline]
    pub unsafe fn point_parameterf(&self, pname: GLenum, param: GLfloat) {
        unsafe { (self.point_parameterf)(pname, param) }
    }
    #[inline]
    pub unsafe fn point_parameterfv(&self, pname: GLenum, params: *const GLfloat) {
        unsafe { (self.point_parameterfv)(pname, params) }
    }
    #[inline]
    pub unsafe fn point_parameteri(&self, pname: GLenum, param: GLint) {
        unsafe { (self.point_parameteri)(pname, param) }
    }
    #[inline]
    pub unsafe fn point_parameteriv(&self, pname: GLenum, params: *const GLint) {
        unsafe { (self.point_parameteriv)(pname, params) }
    }
    #[inline]
    pub unsafe fn blend_color(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        unsafe { (self.blend_color)(red, green, blue, alpha) }
    }
    #[inline]
    pub unsafe fn blend_equation(&self, mode: GLenum) {
        unsafe { (self.blend_equation)(mode) }
    }
    #[inline]
    pub unsafe fn gen_queries(&self, n: GLsizei, ids: *mut GLuint) {
        unsafe { (self.gen_queries)(n, ids) }
    }
    #[inline]
    pub unsafe fn delete_queries(&self, n: GLsizei, ids: *const GLuint) {
        unsafe { (self.delete_queries)(n, ids) }
    }
    #[inline]
    pub unsafe fn is_query(&self, id: GLuint) -> GLboolean {
        unsafe { (self.is_query)(id) }
    }
    #[inline]
    pub unsafe fn begin_query(&self, target: GLenum, id: GLuint) {
        unsafe { (self.begin_query)(target, id) }
    }
    #[inline]
    pub unsafe fn end_query(&self, target: GLenum) {
        unsafe { (self.end_query)(target) }
    }
    #[inline]
    pub unsafe fn get_queryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_queryiv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn get_query_objectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_query_objectiv)(id, pname, params) }
    }
    #[inline]
    pub unsafe fn get_query_objectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) {
        unsafe { (self.get_query_objectuiv)(id, pname, params) }
    }
    #[inline]
    pub unsafe fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
        unsafe { (self.bind_buffer)(target, buffer) }
    }
    #[inline]
    pub unsafe fn delete_buffers(&self, n: GLsizei, buffers: *const GLuint) {
        unsafe { (self.delete_buffers)(n, buffers) }
    }
    #[inline]
    pub unsafe fn gen_buffers(&self, n: GLsizei, buffers: *mut GLuint) {
        unsafe { (self.gen_buffers)(n, buffers) }
    }
    #[inline]
    pub unsafe fn is_buffer(&self, buffer: GLuint) -> GLboolean {
        unsafe { (self.is_buffer)(buffer) }
    }
    #[inline]
    pub unsafe fn buffer_data(
        &self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
    ) {
        unsafe { (self.buffer_data)(target, size, data, usage) }
    }
    #[inline]
    pub unsafe fn buffer_sub_data(
        &self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void,
    ) {
        unsafe { (self.buffer_sub_data)(target, offset, size, data) }
    }
    #[inline]
    pub unsafe fn get_buffer_sub_data(
        &self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut c_void,
    ) {
        unsafe { (self.get_buffer_sub_data)(target, offset, size, data) }
    }
    #[inline]
    pub unsafe fn map_buffer(&self, target: GLenum, access: GLenum) -> *mut c_void {
        unsafe { (self.map_buffer)(target, access) }
    }
    #[inline]
    pub unsafe fn unmap_buffer(&self, target: GLenum) -> GLboolean {
        unsafe { (self.unmap_buffer)(target) }
    }
    #[inline]
    pub unsafe fn get_buffer_parameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_buffer_parameteriv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn get_buffer_pointerv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut *mut c_void,
    ) {
        unsafe { (self.get_buffer_pointerv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        unsafe { (self.blend_equation_separate)(mode_rgb, mode_alpha) }
    }
    #[inline]
    pub unsafe fn draw_buffers(&self, n: GLsizei, bufs: *const GLenum) {
        unsafe { (self.draw_buffers)(n, bufs) }
    }
    #[inline]
    pub unsafe fn stencil_op_separate(
        &self,
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum,
    ) {
        unsafe { (self.stencil_op_separate)(face, sfail, dpfail, dppass) }
    }
    #[inline]
    pub unsafe fn stencil_func_separate(
        &self,
        face: GLenum,
        func: GLenum,
        ref_: GLint,
        mask: GLuint,
    ) {
        unsafe { (self.stencil_func_separate)(face, func, ref_, mask) }
    }
    #[inline]
    pub unsafe fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        unsafe { (self.stencil_mask_separate)(face, mask) }
    }
    #[inline]
    pub unsafe fn attach_shader(&self, program: GLuint, shader: GLuint) {
        unsafe { (self.attach_shader)(program, shader) }
    }
    #[inline]
    pub unsafe fn bind_attrib_location(&self, program: GLuint, index: GLuint, name: *const GLchar) {
        unsafe { (self.bind_attrib_location)(program, index, name) }
    }
    #[inline]
    pub unsafe fn compile_shader(&self, shader: GLuint) {
        unsafe { (self.compile_shader)(shader) }
    }
    #[inline]
    pub unsafe fn create_program(&self) -> GLuint {
        unsafe { (self.create_program)() }
    }
    #[inline]
    pub unsafe fn create_shader(&self, type_: GLenum) -> GLuint {
        unsafe { (self.create_shader)(type_) }
    }
    #[inline]
    pub unsafe fn delete_program(&self, program: GLuint) {
        unsafe { (self.delete_program)(program) }
    }
    #[inline]
    pub unsafe fn delete_shader(&self, shader: GLuint) {
        unsafe { (self.delete_shader)(shader) }
    }
    #[inline]
    pub unsafe fn detach_shader(&self, program: GLuint, shader: GLuint) {
        unsafe { (self.detach_shader)(program, shader) }
    }
    #[inline]
    pub unsafe fn disable_vertex_attrib_array(&self, index: GLuint) {
        unsafe { (self.disable_vertex_attrib_array)(index) }
    }
    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, index: GLuint) {
        unsafe { (self.enable_vertex_attrib_array)(index) }
    }
    #[inline]
    pub unsafe fn get_active_attrib(
        &self,
        program: GLuint,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    ) {
        unsafe { (self.get_active_attrib)(program, index, buf_size, length, size, type_, name) }
    }
    #[inline]
    pub unsafe fn get_active_uniform(
        &self,
        program: GLuint,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    ) {
        unsafe { (self.get_active_uniform)(program, index, buf_size, length, size, type_, name) }
    }
    #[inline]
    pub unsafe fn get_attached_shaders(
        &self,
        program: GLuint,
        max_count: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    ) {
        unsafe { (self.get_attached_shaders)(program, max_count, count, shaders) }
    }
    #[inline]
    pub unsafe fn get_attrib_location(&self, program: GLuint, name: *const GLchar) -> GLint {
        unsafe { (self.get_attrib_location)(program, name) }
    }
    #[inline]
    pub unsafe fn get_programiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_programiv)(program, pname, params) }
    }
    #[inline]
    pub unsafe fn get_program_info_log(
        &self,
        program: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        unsafe { (self.get_program_info_log)(program, buf_size, length, info_log) }
    }
    #[inline]
    pub unsafe fn get_shaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_shaderiv)(shader, pname, params) }
    }
    #[inline]
    pub unsafe fn get_shader_info_log(
        &self,
        shader: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        unsafe { (self.get_shader_info_log)(shader, buf_size, length, info_log) }
    }
    #[inline]
    pub unsafe fn get_shader_source(
        &self,
        shader: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    ) {
        unsafe { (self.get_shader_source)(shader, buf_size, length, source) }
    }
    #[inline]
    pub unsafe fn get_uniform_location(&self, program: GLuint, name: *const GLchar) -> GLint {
        unsafe { (self.get_uniform_location)(program, name) }
    }
    #[inline]
    pub unsafe fn get_uniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
        unsafe { (self.get_uniformfv)(program, location, params) }
    }
    #[inline]
    pub unsafe fn get_uniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
        unsafe { (self.get_uniformiv)(program, location, params) }
    }
    #[inline]
    pub unsafe fn get_vertex_attribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
        unsafe { (self.get_vertex_attribdv)(index, pname, params) }
    }
    #[inline]
    pub unsafe fn get_vertex_attribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) {
        unsafe { (self.get_vertex_attribfv)(index, pname, params) }
    }
    #[inline]
    pub unsafe fn get_vertex_attribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_vertex_attribiv)(index, pname, params) }
    }
    #[inline]
    pub unsafe fn get_vertex_attrib_pointerv(
        &self,
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut c_void,
    ) {
        unsafe { (self.get_vertex_attrib_pointerv)(index, pname, pointer) }
    }
    #[inline]
    pub unsafe fn is_program(&self, program: GLuint) -> GLboolean {
        unsafe { (self.is_program)(program) }
    }
    #[inline]
    pub unsafe fn is_shader(&self, shader: GLuint) -> GLboolean {
        unsafe { (self.is_shader)(shader) }
    }
    #[inline]
    pub unsafe fn link_program(&self, program: GLuint) {
        unsafe { (self.link_program)(program) }
    }
    #[inline]
    pub unsafe fn shader_source(
        &self,
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        unsafe { (self.shader_source)(shader, count, string, length) }
    }
    #[inline]
    pub unsafe fn use_program(&self, program: GLuint) {
        unsafe { (self.use_program)(program) }
    }
    #[inline]
    pub unsafe fn uniform1f(&self, location: GLint, v0: GLfloat) {
        unsafe { (self.uniform1f)(location, v0) }
    }
    #[inline]
    pub unsafe fn uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
        unsafe { (self.uniform2f)(location, v0, v1) }
    }
    #[inline]
    pub unsafe fn uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
        unsafe { (self.uniform3f)(location, v0, v1, v2) }
    }
    #[inline]
    pub unsafe fn uniform4f(
        &self,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) {
        unsafe { (self.uniform4f)(location, v0, v1, v2, v3) }
    }
    #[inline]
    pub unsafe fn uniform1i(&self, location: GLint, v0: GLint) {
        unsafe { (self.uniform1i)(location, v0) }
    }
    #[inline]
    pub unsafe fn uniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
        unsafe { (self.uniform2i)(location, v0, v1) }
    }
    #[inline]
    pub unsafe fn uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
        unsafe { (self.uniform3i)(location, v0, v1, v2) }
    }
    #[inline]
    pub unsafe fn uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
        unsafe { (self.uniform4i)(location, v0, v1, v2, v3) }
    }
    #[inline]
    pub unsafe fn uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        unsafe { (self.uniform1fv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        unsafe { (self.uniform2fv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        unsafe { (self.uniform3fv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
        unsafe { (self.uniform4fv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        unsafe { (self.uniform1iv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        unsafe { (self.uniform2iv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        unsafe { (self.uniform3iv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
        unsafe { (self.uniform4iv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix2fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix3fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix4fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn validate_program(&self, program: GLuint) {
        unsafe { (self.validate_program)(program) }
    }
    #[inline]
    pub unsafe fn vertex_attrib1d(&self, index: GLuint, x: GLdouble) {
        unsafe { (self.vertex_attrib1d)(index, x) }
    }
    #[inline]
    pub unsafe fn vertex_attrib1dv(&self, index: GLuint, v: *const GLdouble) {
        unsafe { (self.vertex_attrib1dv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        unsafe { (self.vertex_attrib1f)(index, x) }
    }
    #[inline]
    pub unsafe fn vertex_attrib1fv(&self, index: GLuint, v: *const GLfloat) {
        unsafe { (self.vertex_attrib1fv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib1s(&self, index: GLuint, x: GLshort) {
        unsafe { (self.vertex_attrib1s)(index, x) }
    }
    #[inline]
    pub unsafe fn vertex_attrib1sv(&self, index: GLuint, v: *const GLshort) {
        unsafe { (self.vertex_attrib1sv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
        unsafe { (self.vertex_attrib2d)(index, x, y) }
    }
    #[inline]
    pub unsafe fn vertex_attrib2dv(&self, index: GLuint, v: *const GLdouble) {
        unsafe { (self.vertex_attrib2dv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        unsafe { (self.vertex_attrib2f)(index, x, y) }
    }
    #[inline]
    pub unsafe fn vertex_attrib2fv(&self, index: GLuint, v: *const GLfloat) {
        unsafe { (self.vertex_attrib2fv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
        unsafe { (self.vertex_attrib2s)(index, x, y) }
    }
    #[inline]
    pub unsafe fn vertex_attrib2sv(&self, index: GLuint, v: *const GLshort) {
        unsafe { (self.vertex_attrib2sv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
        unsafe { (self.vertex_attrib3d)(index, x, y, z) }
    }
    #[inline]
    pub unsafe fn vertex_attrib3dv(&self, index: GLuint, v: *const GLdouble) {
        unsafe { (self.vertex_attrib3dv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        unsafe { (self.vertex_attrib3f)(index, x, y, z) }
    }
    #[inline]
    pub unsafe fn vertex_attrib3fv(&self, index: GLuint, v: *const GLfloat) {
        unsafe { (self.vertex_attrib3fv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
        unsafe { (self.vertex_attrib3s)(index, x, y, z) }
    }
    #[inline]
    pub unsafe fn vertex_attrib3sv(&self, index: GLuint, v: *const GLshort) {
        unsafe { (self.vertex_attrib3sv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4nbv(&self, index: GLuint, v: *const GLbyte) {
        unsafe { (self.vertex_attrib_4nbv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4niv(&self, index: GLuint, v: *const GLint) {
        unsafe { (self.vertex_attrib_4niv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4nsv(&self, index: GLuint, v: *const GLshort) {
        unsafe { (self.vertex_attrib_4nsv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4nub(
        &self,
        index: GLuint,
        x: GLubyte,
        y: GLubyte,
        z: GLubyte,
        w: GLubyte,
    ) {
        unsafe { (self.vertex_attrib_4nub)(index, x, y, z, w) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4nubv(&self, index: GLuint, v: *const GLubyte) {
        unsafe { (self.vertex_attrib_4nubv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4nuiv(&self, index: GLuint, v: *const GLuint) {
        unsafe { (self.vertex_attrib_4nuiv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_4nusv(&self, index: GLuint, v: *const GLushort) {
        unsafe { (self.vertex_attrib_4nusv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4bv(&self, index: GLuint, v: *const GLbyte) {
        unsafe { (self.vertex_attrib4bv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4d(
        &self,
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble,
    ) {
        unsafe { (self.vertex_attrib4d)(index, x, y, z, w) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4dv(&self, index: GLuint, v: *const GLdouble) {
        unsafe { (self.vertex_attrib4dv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4f(
        &self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        unsafe { (self.vertex_attrib4f)(index, x, y, z, w) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4fv(&self, index: GLuint, v: *const GLfloat) {
        unsafe { (self.vertex_attrib4fv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4iv(&self, index: GLuint, v: *const GLint) {
        unsafe { (self.vertex_attrib4iv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4s(
        &self,
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort,
        w: GLshort,
    ) {
        unsafe { (self.vertex_attrib4s)(index, x, y, z, w) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4sv(&self, index: GLuint, v: *const GLshort) {
        unsafe { (self.vertex_attrib4sv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4ubv(&self, index: GLuint, v: *const GLubyte) {
        unsafe { (self.vertex_attrib4ubv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4uiv(&self, index: GLuint, v: *const GLuint) {
        unsafe { (self.vertex_attrib4uiv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib4usv(&self, index: GLuint, v: *const GLushort) {
        unsafe { (self.vertex_attrib4usv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        unsafe { (self.vertex_attrib_pointer)(index, size, type_, normalized, stride, pointer) }
    }
    #[inline]
    pub unsafe fn uniform_matrix2x3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix2x3fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix3x2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix3x2fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix2x4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix2x4fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix4x2fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix4x2fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix3x4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix3x4fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn uniform_matrix4x3fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix4x3fv)(location, count, transpose, value) }
    }
    #[inline]
    pub unsafe fn color_maski(
        &self,
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean,
    ) {
        unsafe { (self.color_maski)(index, r, g, b, a) }
    }
    #[inline]
    pub unsafe fn get_booleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
        unsafe { (self.get_booleani_v)(target, index, data) }
    }
    #[inline]
    pub unsafe fn get_integeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) {
        unsafe { (self.get_integeri_v)(target, index, data) }
    }
    #[inline]
    pub unsafe fn enablei(&self, target: GLenum, index: GLuint) {
        unsafe { (self.enablei)(target, index) }
    }
    #[inline]
    pub unsafe fn disablei(&self, target: GLenum, index: GLuint) {
        unsafe { (self.disablei)(target, index) }
    }
    #[inline]
    pub unsafe fn is_enabledi(&self, target: GLenum, index: GLuint) -> GLboolean {
        unsafe { (self.is_enabledi)(target, index) }
    }
    #[inline]
    pub unsafe fn begin_transform_feedback(&self, primitive_mode: GLenum) {
        unsafe { (self.begin_transform_feedback)(primitive_mode) }
    }
    #[inline]
    pub unsafe fn end_transform_feedback(&self) {
        unsafe { (self.end_transform_feedback)() }
    }
    #[inline]
    pub unsafe fn bind_buffer_range(
        &self,
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        unsafe { (self.bind_buffer_range)(target, index, buffer, offset, size) }
    }
    #[inline]
    pub unsafe fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: GLuint) {
        unsafe { (self.bind_buffer_base)(target, index, buffer) }
    }
    #[inline]
    pub unsafe fn transform_feedback_varyings(
        &self,
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        buffer_mode: GLenum,
    ) {
        unsafe { (self.transform_feedback_varyings)(program, count, varyings, buffer_mode) }
    }
    #[inline]
    pub unsafe fn get_transform_feedback_varying(
        &self,
        program: GLuint,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar,
    ) {
        unsafe {
            (self.get_transform_feedback_varying)(
                program, index, buf_size, length, size, type_, name,
            )
        }
    }
    #[inline]
    pub unsafe fn clamp_color(&self, target: GLenum, clamp: GLenum) {
        unsafe { (self.clamp_color)(target, clamp) }
    }
    #[inline]
    pub unsafe fn begin_conditional_render(&self, id: GLuint, mode: GLenum) {
        unsafe { (self.begin_conditional_render)(id, mode) }
    }
    #[inline]
    pub unsafe fn end_conditional_render(&self) {
        unsafe { (self.end_conditional_render)() }
    }
    #[inline]
    pub unsafe fn vertex_attrib_ipointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        unsafe { (self.vertex_attrib_ipointer)(index, size, type_, stride, pointer) }
    }
    #[inline]
    pub unsafe fn get_vertex_attrib_iiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_vertex_attrib_iiv)(index, pname, params) }
    }
    #[inline]
    pub unsafe fn get_vertex_attrib_iuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) {
        unsafe { (self.get_vertex_attrib_iuiv)(index, pname, params) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i1i(&self, index: GLuint, x: GLint) {
        unsafe { (self.vertex_attrib_i1i)(index, x) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i2i(&self, index: GLuint, x: GLint, y: GLint) {
        unsafe { (self.vertex_attrib_i2i)(index, x, y) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
        unsafe { (self.vertex_attrib_i3i)(index, x, y, z) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        unsafe { (self.vertex_attrib_i4i)(index, x, y, z, w) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i1ui(&self, index: GLuint, x: GLuint) {
        unsafe { (self.vertex_attrib_i1ui)(index, x) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
        unsafe { (self.vertex_attrib_i2ui)(index, x, y) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
        unsafe { (self.vertex_attrib_i3ui)(index, x, y, z) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4ui(
        &self,
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    ) {
        unsafe { (self.vertex_attrib_i4ui)(index, x, y, z, w) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i1iv(&self, index: GLuint, v: *const GLint) {
        unsafe { (self.vertex_attrib_i1iv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i2iv(&self, index: GLuint, v: *const GLint) {
        unsafe { (self.vertex_attrib_i2iv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i3iv(&self, index: GLuint, v: *const GLint) {
        unsafe { (self.vertex_attrib_i3iv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4iv(&self, index: GLuint, v: *const GLint) {
        unsafe { (self.vertex_attrib_i4iv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i1uiv(&self, index: GLuint, v: *const GLuint) {
        unsafe { (self.vertex_attrib_i1uiv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i2uiv(&self, index: GLuint, v: *const GLuint) {
        unsafe { (self.vertex_attrib_i2uiv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i3uiv(&self, index: GLuint, v: *const GLuint) {
        unsafe { (self.vertex_attrib_i3uiv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4uiv(&self, index: GLuint, v: *const GLuint) {
        unsafe { (self.vertex_attrib_i4uiv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4bv(&self, index: GLuint, v: *const GLbyte) {
        unsafe { (self.vertex_attrib_i4bv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4sv(&self, index: GLuint, v: *const GLshort) {
        unsafe { (self.vertex_attrib_i4sv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4ubv(&self, index: GLuint, v: *const GLubyte) {
        unsafe { (self.vertex_attrib_i4ubv)(index, v) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_i4usv(&self, index: GLuint, v: *const GLushort) {
        unsafe { (self.vertex_attrib_i4usv)(index, v) }
    }
    #[inline]
    pub unsafe fn get_uniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
        unsafe { (self.get_uniformuiv)(program, location, params) }
    }
    #[inline]
    pub unsafe fn bind_frag_data_location(
        &self,
        program: GLuint,
        color: GLuint,
        name: *const GLchar,
    ) {
        unsafe { (self.bind_frag_data_location)(program, color, name) }
    }
    #[inline]
    pub unsafe fn get_frag_data_location(&self, program: GLuint, name: *const GLchar) -> GLint {
        unsafe { (self.get_frag_data_location)(program, name) }
    }
    #[inline]
    pub unsafe fn uniform1ui(&self, location: GLint, v0: GLuint) {
        unsafe { (self.uniform1ui)(location, v0) }
    }
    #[inline]
    pub unsafe fn uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
        unsafe { (self.uniform2ui)(location, v0, v1) }
    }
    #[inline]
    pub unsafe fn uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
        unsafe { (self.uniform3ui)(location, v0, v1, v2) }
    }
    #[inline]
    pub unsafe fn uniform4ui(
        &self,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        unsafe { (self.uniform4ui)(location, v0, v1, v2, v3) }
    }
    #[inline]
    pub unsafe fn uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        unsafe { (self.uniform1uiv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        unsafe { (self.uniform2uiv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        unsafe { (self.uniform3uiv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
        unsafe { (self.uniform4uiv)(location, count, value) }
    }
    #[inline]
    pub unsafe fn tex_parameter_iiv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
        unsafe { (self.tex_parameter_iiv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn tex_parameter_iuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) {
        unsafe { (self.tex_parameter_iuiv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn get_tex_parameter_iiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
        unsafe { (self.get_tex_parameter_iiv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn get_tex_parameter_iuiv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        unsafe { (self.get_tex_parameter_iuiv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn clear_bufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
        unsafe { (self.clear_bufferiv)(buffer, drawbuffer, value) }
    }
    #[inline]
    pub unsafe fn clear_bufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
        unsafe { (self.clear_bufferuiv)(buffer, drawbuffer, value) }
    }
    #[inline]
    pub unsafe fn clear_bufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
        unsafe { (self.clear_bufferfv)(buffer, drawbuffer, value) }
    }
    #[inline]
    pub unsafe fn clear_bufferfi(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        unsafe { (self.clear_bufferfi)(buffer, drawbuffer, depth, stencil) }
    }
    #[inline]
    pub unsafe fn get_stringi(&self, name: GLenum, index: GLuint) -> *const GLubyte {
        unsafe { (self.get_stringi)(name, index) }
    }
    #[inline]
    pub unsafe fn is_renderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
        unsafe { (self.is_renderbuffer)(renderbuffer) }
    }
    #[inline]
    pub unsafe fn bind_renderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
        unsafe { (self.bind_renderbuffer)(target, renderbuffer) }
    }
    #[inline]
    pub unsafe fn delete_renderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
        unsafe { (self.delete_renderbuffers)(n, renderbuffers) }
    }
    #[inline]
    pub unsafe fn gen_renderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
        unsafe { (self.gen_renderbuffers)(n, renderbuffers) }
    }
    #[inline]
    pub unsafe fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe { (self.renderbuffer_storage)(target, internalformat, width, height) }
    }
    #[inline]
    pub unsafe fn get_renderbuffer_parameteriv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe { (self.get_renderbuffer_parameteriv)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn is_framebuffer(&self, framebuffer: GLuint) -> GLboolean {
        unsafe { (self.is_framebuffer)(framebuffer) }
    }
    #[inline]
    pub unsafe fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {
        unsafe { (self.bind_framebuffer)(target, framebuffer) }
    }
    #[inline]
    pub unsafe fn delete_framebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
        unsafe { (self.delete_framebuffers)(n, framebuffers) }
    }
    #[inline]
    pub unsafe fn gen_framebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
        unsafe { (self.gen_framebuffers)(n, framebuffers) }
    }
    #[inline]
    pub unsafe fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        unsafe { (self.check_framebuffer_status)(target) }
    }
    #[inline]
    pub unsafe fn framebuffer_texture_1d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        unsafe { (self.framebuffer_texture_1d)(target, attachment, textarget, texture, level) }
    }
    #[inline]
    pub unsafe fn framebuffer_texture_2d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        unsafe { (self.framebuffer_texture_2d)(target, attachment, textarget, texture, level) }
    }
    #[inline]
    pub unsafe fn framebuffer_texture_3d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
        zoffset: GLint,
    ) {
        unsafe {
            (self.framebuffer_texture_3d)(target, attachment, textarget, texture, level, zoffset)
        }
    }
    #[inline]
    pub unsafe fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        unsafe {
            (self.framebuffer_renderbuffer)(target, attachment, renderbuffertarget, renderbuffer)
        }
    }
    #[inline]
    pub unsafe fn get_framebuffer_attachment_parameteriv(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe { (self.get_framebuffer_attachment_parameteriv)(target, attachment, pname, params) }
    }
    #[inline]
    pub unsafe fn generate_mipmap(&self, target: GLenum) {
        unsafe { (self.generate_mipmap)(target) }
    }
    #[inline]
    pub unsafe fn blit_framebuffer(
        &self,
        src_x0: GLint,
        src_y0: GLint,
        src_x1: GLint,
        src_y1: GLint,
        dst_x0: GLint,
        dst_y0: GLint,
        dst_x1: GLint,
        dst_y1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        unsafe {
            (self.blit_framebuffer)(
                src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
            )
        }
    }
    #[inline]
    pub unsafe fn renderbuffer_storage_multisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            (self.renderbuffer_storage_multisample)(target, samples, internalformat, width, height)
        }
    }
    #[inline]
    pub unsafe fn framebuffer_texture_layer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) {
        unsafe { (self.framebuffer_texture_layer)(target, attachment, texture, level, layer) }
    }
    #[inline]
    pub unsafe fn map_buffer_range(
        &self,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut c_void {
        unsafe { (self.map_buffer_range)(target, offset, length, access) }
    }
    #[inline]
    pub unsafe fn flush_mapped_buffer_range(
        &self,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
    ) {
        unsafe { (self.flush_mapped_buffer_range)(target, offset, length) }
    }
    #[inline]
    pub unsafe fn bind_vertex_array(&self, array: GLuint) {
        unsafe { (self.bind_vertex_array)(array) }
    }
    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, n: GLsizei, arrays: *const GLuint) {
        unsafe { (self.delete_vertex_arrays)(n, arrays) }
    }
    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, n: GLsizei, arrays: *mut GLuint) {
        unsafe { (self.gen_vertex_arrays)(n, arrays) }
    }
    #[inline]
    pub unsafe fn is_vertex_array(&self, array: GLuint) -> GLboolean {
        unsafe { (self.is_vertex_array)(array) }
    }
    #[inline]
    pub unsafe fn draw_arrays_instanced(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    ) {
        unsafe { (self.draw_arrays_instanced)(mode, first, count, instancecount) }
    }
    #[inline]
    pub unsafe fn draw_elements_instanced(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
    ) {
        unsafe { (self.draw_elements_instanced)(mode, count, type_, indices, instancecount) }
    }
    #[inline]
    pub unsafe fn tex_buffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) {
        unsafe { (self.tex_buffer)(target, internalformat, buffer) }
    }
    #[inline]
    pub unsafe fn primitive_restart_index(&self, index: GLuint) {
        unsafe { (self.primitive_restart_index)(index) }
    }
    #[inline]
    pub unsafe fn copy_buffer_sub_data(
        &self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        unsafe {
            (self.copy_buffer_sub_data)(read_target, write_target, read_offset, write_offset, size)
        }
    }
    #[inline]
    pub unsafe fn get_uniform_indices(
        &self,
        program: GLuint,
        uniform_count: GLsizei,
        uniform_names: *const *const GLchar,
        uniform_indices: *mut GLuint,
    ) {
        unsafe {
            (self.get_uniform_indices)(program, uniform_count, uniform_names, uniform_indices)
        }
    }
    #[inline]
    pub unsafe fn get_active_uniformsiv(
        &self,
        program: GLuint,
        uniform_count: GLsizei,
        uniform_indices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe {
            (self.get_active_uniformsiv)(program, uniform_count, uniform_indices, pname, params)
        }
    }
    #[inline]
    pub unsafe fn get_active_uniform_name(
        &self,
        program: GLuint,
        uniform_index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        uniform_name: *mut GLchar,
    ) {
        unsafe {
            (self.get_active_uniform_name)(program, uniform_index, buf_size, length, uniform_name)
        }
    }
    #[inline]
    pub unsafe fn get_uniform_block_index(
        &self,
        program: GLuint,
        uniform_block_name: *const GLchar,
    ) -> GLuint {
        unsafe { (self.get_uniform_block_index)(program, uniform_block_name) }
    }
    #[inline]
    pub unsafe fn get_active_uniform_blockiv(
        &self,
        program: GLuint,
        uniform_block_index: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe { (self.get_active_uniform_blockiv)(program, uniform_block_index, pname, params) }
    }
    #[inline]
    pub unsafe fn get_active_uniform_block_name(
        &self,
        program: GLuint,
        uniform_block_index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        uniform_block_name: *mut GLchar,
    ) {
        unsafe {
            (self.get_active_uniform_block_name)(
                program,
                uniform_block_index,
                buf_size,
                length,
                uniform_block_name,
            )
        }
    }
    #[inline]
    pub unsafe fn uniform_block_binding(
        &self,
        program: GLuint,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) {
        unsafe { (self.uniform_block_binding)(program, uniform_block_index, uniform_block_binding) }
    }
    #[inline]
    pub unsafe fn draw_elements_base_vertex(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        basevertex: GLint,
    ) {
        unsafe { (self.draw_elements_base_vertex)(mode, count, type_, indices, basevertex) }
    }
    #[inline]
    pub unsafe fn draw_range_elements_base_vertex(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        basevertex: GLint,
    ) {
        unsafe {
            (self.draw_range_elements_base_vertex)(
                mode, start, end, count, type_, indices, basevertex,
            )
        }
    }
    #[inline]
    pub unsafe fn draw_elements_instanced_base_vertex(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        basevertex: GLint,
    ) {
        unsafe {
            (self.draw_elements_instanced_base_vertex)(
                mode,
                count,
                type_,
                indices,
                instancecount,
                basevertex,
            )
        }
    }
    #[inline]
    pub unsafe fn multi_draw_elements_base_vertex(
        &self,
        mode: GLenum,
        count: *const GLsizei,
        type_: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei,
        basevertex: *const GLint,
    ) {
        unsafe {
            (self.multi_draw_elements_base_vertex)(
                mode, count, type_, indices, drawcount, basevertex,
            )
        }
    }
    #[inline]
    pub unsafe fn provoking_vertex(&self, mode: GLenum) {
        unsafe { (self.provoking_vertex)(mode) }
    }
    #[inline]
    pub unsafe fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
        unsafe { (self.fence_sync)(condition, flags) }
    }
    #[inline]
    pub unsafe fn is_sync(&self, sync: GLsync) -> GLboolean {
        unsafe { (self.is_sync)(sync) }
    }
    #[inline]
    pub unsafe fn delete_sync(&self, sync: GLsync) {
        unsafe { (self.delete_sync)(sync) }
    }
    #[inline]
    pub unsafe fn client_wait_sync(
        &self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        unsafe { (self.client_wait_sync)(sync, flags, timeout) }
    }
    #[inline]
    pub unsafe fn wait_sync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
        unsafe { (self.wait_sync)(sync, flags, timeout) }
    }
    #[inline]
    pub unsafe fn get_integer64v(&self, pname: GLenum, data: *mut GLint64) {
        unsafe { (self.get_integer64v)(pname, data) }
    }
    #[inline]
    pub unsafe fn get_synciv(
        &self,
        sync: GLsync,
        pname: GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    ) {
        unsafe { (self.get_synciv)(sync, pname, count, length, values) }
    }
    #[inline]
    pub unsafe fn get_integer64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) {
        unsafe { (self.get_integer64i_v)(target, index, data) }
    }
    #[inline]
    pub unsafe fn get_buffer_parameteri64v(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint64,
    ) {
        unsafe { (self.get_buffer_parameteri64v)(target, pname, params) }
    }
    #[inline]
    pub unsafe fn framebuffer_texture(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        unsafe { (self.framebuffer_texture)(target, attachment, texture, level) }
    }
    #[inline]
    pub unsafe fn tex_image_2dmultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        unsafe {
            (self.tex_image_2dmultisample)(
                target,
                samples,
                internalformat,
                width,
                height,
                fixedsamplelocations,
            )
        }
    }
    #[inline]
    pub unsafe fn tex_image_3dmultisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        unsafe {
            (self.tex_image_3dmultisample)(
                target,
                samples,
                internalformat,
                width,
                height,
                depth,
                fixedsamplelocations,
            )
        }
    }
    #[inline]
    pub unsafe fn get_multisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
        unsafe { (self.get_multisamplefv)(pname, index, val) }
    }
    #[inline]
    pub unsafe fn sample_maski(&self, mask_number: GLuint, mask: GLbitfield) {
        unsafe { (self.sample_maski)(mask_number, mask) }
    }
    #[inline]
    pub unsafe fn bind_frag_data_location_indexed(
        &self,
        program: GLuint,
        color_number: GLuint,
        index: GLuint,
        name: *const GLchar,
    ) {
        unsafe { (self.bind_frag_data_location_indexed)(program, color_number, index, name) }
    }
    #[inline]
    pub unsafe fn get_frag_data_index(&self, program: GLuint, name: *const GLchar) -> GLint {
        unsafe { (self.get_frag_data_index)(program, name) }
    }
    #[inline]
    pub unsafe fn gen_samplers(&self, count: GLsizei, samplers: *mut GLuint) {
        unsafe { (self.gen_samplers)(count, samplers) }
    }
    #[inline]
    pub unsafe fn delete_samplers(&self, count: GLsizei, samplers: *const GLuint) {
        unsafe { (self.delete_samplers)(count, samplers) }
    }
    #[inline]
    pub unsafe fn is_sampler(&self, sampler: GLuint) -> GLboolean {
        unsafe { (self.is_sampler)(sampler) }
    }
    #[inline]
    pub unsafe fn bind_sampler(&self, unit: GLuint, sampler: GLuint) {
        unsafe { (self.bind_sampler)(unit, sampler) }
    }
    #[inline]
    pub unsafe fn sampler_parameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) {
        unsafe { (self.sampler_parameteri)(sampler, pname, param) }
    }
    #[inline]
    pub unsafe fn sampler_parameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
        unsafe { (self.sampler_parameteriv)(sampler, pname, param) }
    }
    #[inline]
    pub unsafe fn sampler_parameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
        unsafe { (self.sampler_parameterf)(sampler, pname, param) }
    }
    #[inline]
    pub unsafe fn sampler_parameterfv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLfloat,
    ) {
        unsafe { (self.sampler_parameterfv)(sampler, pname, param) }
    }
    #[inline]
    pub unsafe fn sampler_parameter_iiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) {
        unsafe { (self.sampler_parameter_iiv)(sampler, pname, param) }
    }
    #[inline]
    pub unsafe fn sampler_parameter_iuiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint,
    ) {
        unsafe { (self.sampler_parameter_iuiv)(sampler, pname, param) }
    }
    #[inline]
    pub unsafe fn get_sampler_parameteriv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe { (self.get_sampler_parameteriv)(sampler, pname, params) }
    }
    #[inline]
    pub unsafe fn get_sampler_parameter_iiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe { (self.get_sampler_parameter_iiv)(sampler, pname, params) }
    }
    #[inline]
    pub unsafe fn get_sampler_parameterfv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        unsafe { (self.get_sampler_parameterfv)(sampler, pname, params) }
    }
    #[inline]
    pub unsafe fn get_sampler_parameter_iuiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) {
        unsafe { (self.get_sampler_parameter_iuiv)(sampler, pname, params) }
    }
    #[inline]
    pub unsafe fn query_counter(&self, id: GLuint, target: GLenum) {
        unsafe { (self.query_counter)(id, target) }
    }
    #[inline]
    pub unsafe fn get_query_objecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64) {
        unsafe { (self.get_query_objecti64v)(id, pname, params) }
    }
    #[inline]
    pub unsafe fn get_query_objectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64) {
        unsafe { (self.get_query_objectui64v)(id, pname, params) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
        unsafe { (self.vertex_attrib_divisor)(index, divisor) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p1ui(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        unsafe { (self.vertex_attrib_p1ui)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p1uiv(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        unsafe { (self.vertex_attrib_p1uiv)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p2ui(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        unsafe { (self.vertex_attrib_p2ui)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p2uiv(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        unsafe { (self.vertex_attrib_p2uiv)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p3ui(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        unsafe { (self.vertex_attrib_p3ui)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p3uiv(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        unsafe { (self.vertex_attrib_p3uiv)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p4ui(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint,
    ) {
        unsafe { (self.vertex_attrib_p4ui)(index, type_, normalized, value) }
    }
    #[inline]
    pub unsafe fn vertex_attrib_p4uiv(
        &self,
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint,
    ) {
        unsafe { (self.vertex_attrib_p4uiv)(index, type_, normalized, value) }
    }
}
