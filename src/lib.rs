use std::{
    ffi::{CStr, c_void},
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
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
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
}
