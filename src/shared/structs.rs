#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[test]
fn bindgen_test_layout_Vector2() {
    const UNINIT: ::std::mem::MaybeUninit<Vector2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector2>(),
        8usize,
        concat!("Size of: ", stringify!(Vector2))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector2>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector2),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector2),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[test]
fn bindgen_test_layout_Vector3() {
    const UNINIT: ::std::mem::MaybeUninit<Vector3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector3>(),
        12usize,
        concat!("Size of: ", stringify!(Vector3))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector3>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector3))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[test]
fn bindgen_test_layout_Vector4() {
    const UNINIT: ::std::mem::MaybeUninit<Vector4> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector4>(),
        16usize,
        concat!("Size of: ", stringify!(Vector4))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector4>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector4))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(w)
        )
    );
}
pub type Quaternion = Vector4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}
#[test]
fn bindgen_test_layout_Matrix() {
    const UNINIT: ::std::mem::MaybeUninit<Matrix> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Matrix>(),
        64usize,
        concat!("Size of: ", stringify!(Matrix))
    );
    assert_eq!(
        ::std::mem::align_of::<Matrix>(),
        4usize,
        concat!("Alignment of ", stringify!(Matrix))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m4) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m8) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m12) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m12)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m1) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m5) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m9) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m9)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m13) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m13)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m2) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m6) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m10) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m14) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m14)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m3) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m7) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m7)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m11) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m11)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m15) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m15)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[test]
fn bindgen_test_layout_Color() {
    const UNINIT: ::std::mem::MaybeUninit<Color> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Color>(),
        4usize,
        concat!("Size of: ", stringify!(Color))
    );
    assert_eq!(
        ::std::mem::align_of::<Color>(),
        1usize,
        concat!("Alignment of ", stringify!(Color))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(r))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(g))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        3usize,
        concat!("Offset of field: ", stringify!(Color), "::", stringify!(a))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[test]
fn bindgen_test_layout_Rectangle() {
    const UNINIT: ::std::mem::MaybeUninit<Rectangle> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Rectangle>(),
        16usize,
        concat!("Size of: ", stringify!(Rectangle))
    );
    assert_eq!(
        ::std::mem::align_of::<Rectangle>(),
        4usize,
        concat!("Alignment of ", stringify!(Rectangle))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Rectangle),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut ::std::os::raw::c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}
#[test]
fn bindgen_test_layout_Image() {
    const UNINIT: ::std::mem::MaybeUninit<Image> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Image>(),
        24usize,
        concat!("Size of: ", stringify!(Image))
    );
    assert_eq!(
        ::std::mem::align_of::<Image>(),
        8usize,
        concat!("Alignment of ", stringify!(Image))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mipmaps) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(mipmaps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Image),
            "::",
            stringify!(format)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}
#[test]
fn bindgen_test_layout_Texture() {
    const UNINIT: ::std::mem::MaybeUninit<Texture> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Texture>(),
        20usize,
        concat!("Size of: ", stringify!(Texture))
    );
    assert_eq!(
        ::std::mem::align_of::<Texture>(),
        4usize,
        concat!("Alignment of ", stringify!(Texture))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mipmaps) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(mipmaps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Texture),
            "::",
            stringify!(format)
        )
    );
}
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: u32,
    pub texture: Texture,
    pub depth: Texture,
}
#[test]
fn bindgen_test_layout_RenderTexture() {
    const UNINIT: ::std::mem::MaybeUninit<RenderTexture> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RenderTexture>(),
        44usize,
        concat!("Size of: ", stringify!(RenderTexture))
    );
    assert_eq!(
        ::std::mem::align_of::<RenderTexture>(),
        4usize,
        concat!("Alignment of ", stringify!(RenderTexture))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RenderTexture),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RenderTexture),
            "::",
            stringify!(texture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RenderTexture),
            "::",
            stringify!(depth)
        )
    );
}
pub type RenderTexture2D = RenderTexture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub layout: i32,
}
#[test]
fn bindgen_test_layout_NPatchInfo() {
    const UNINIT: ::std::mem::MaybeUninit<NPatchInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<NPatchInfo>(),
        36usize,
        concat!("Size of: ", stringify!(NPatchInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<NPatchInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(NPatchInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).left) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).top) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(top)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).right) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(right)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bottom) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(bottom)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).layout) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(NPatchInfo),
            "::",
            stringify!(layout)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfo {
    pub value: i32,
    pub offsetX: i32,
    pub offsetY: i32,
    pub advanceX: i32,
    pub image: Image,
}
#[test]
fn bindgen_test_layout_GlyphInfo() {
    const UNINIT: ::std::mem::MaybeUninit<GlyphInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<GlyphInfo>(),
        40usize,
        concat!("Size of: ", stringify!(GlyphInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<GlyphInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(GlyphInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offsetX) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(offsetX)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offsetY) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(offsetY)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).advanceX) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(advanceX)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).image) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GlyphInfo),
            "::",
            stringify!(image)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub baseSize: i32,
    pub glyphCount: i32,
    pub glyphPadding: i32,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
#[test]
fn bindgen_test_layout_Font() {
    const UNINIT: ::std::mem::MaybeUninit<Font> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Font>(),
        48usize,
        concat!("Size of: ", stringify!(Font))
    );
    assert_eq!(
        ::std::mem::align_of::<Font>(),
        8usize,
        concat!("Alignment of ", stringify!(Font))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baseSize) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(baseSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).glyphCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(glyphCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).glyphPadding) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(glyphPadding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(texture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).recs) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(recs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).glyphs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Font),
            "::",
            stringify!(glyphs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: i32,
}
#[test]
fn bindgen_test_layout_Camera3D() {
    const UNINIT: ::std::mem::MaybeUninit<Camera3D> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Camera3D>(),
        44usize,
        concat!("Size of: ", stringify!(Camera3D))
    );
    assert_eq!(
        ::std::mem::align_of::<Camera3D>(),
        4usize,
        concat!("Alignment of ", stringify!(Camera3D))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).position) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(position)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).up) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(up)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fovy) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(fovy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).projection) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera3D),
            "::",
            stringify!(projection)
        )
    );
}
pub type Camera = Camera3D;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}
#[test]
fn bindgen_test_layout_Camera2D() {
    const UNINIT: ::std::mem::MaybeUninit<Camera2D> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Camera2D>(),
        24usize,
        concat!("Size of: ", stringify!(Camera2D))
    );
    assert_eq!(
        ::std::mem::align_of::<Camera2D>(),
        4usize,
        concat!("Alignment of ", stringify!(Camera2D))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rotation) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(rotation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).zoom) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Camera2D),
            "::",
            stringify!(zoom)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertexCount: i32,
    pub triangleCount: i32,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub texcoords2: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut u8,
    pub indices: *mut ::std::os::raw::c_ushort,
    pub animVertices: *mut f32,
    pub animNormals: *mut f32,
    pub boneIds: *mut u8,
    pub boneWeights: *mut f32,
    pub vaoId: u32,
    pub vboId: *mut u32,
}
#[test]
fn bindgen_test_layout_Mesh() {
    const UNINIT: ::std::mem::MaybeUninit<Mesh> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Mesh>(),
        112usize,
        concat!("Size of: ", stringify!(Mesh))
    );
    assert_eq!(
        ::std::mem::align_of::<Mesh>(),
        8usize,
        concat!("Alignment of ", stringify!(Mesh))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vertexCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vertexCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).triangleCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(triangleCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vertices) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vertices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texcoords) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(texcoords)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texcoords2) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(texcoords2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).normals) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(normals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tangents) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(tangents)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colors) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(colors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).indices) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(indices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).animVertices) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(animVertices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).animNormals) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(animNormals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneIds) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(boneIds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneWeights) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(boneWeights)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vaoId) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vaoId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vboId) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Mesh),
            "::",
            stringify!(vboId)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub id: u32,
    pub locs: *mut i32,
}
#[test]
fn bindgen_test_layout_Shader() {
    const UNINIT: ::std::mem::MaybeUninit<Shader> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Shader>(),
        16usize,
        concat!("Size of: ", stringify!(Shader))
    );
    assert_eq!(
        ::std::mem::align_of::<Shader>(),
        8usize,
        concat!("Alignment of ", stringify!(Shader))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Shader),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).locs) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Shader),
            "::",
            stringify!(locs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
    pub texture: Texture2D,
    pub color: Color,
    pub value: f32,
}
#[test]
fn bindgen_test_layout_MaterialMap() {
    const UNINIT: ::std::mem::MaybeUninit<MaterialMap> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<MaterialMap>(),
        28usize,
        concat!("Size of: ", stringify!(MaterialMap))
    );
    assert_eq!(
        ::std::mem::align_of::<MaterialMap>(),
        4usize,
        concat!("Alignment of ", stringify!(MaterialMap))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MaterialMap),
            "::",
            stringify!(texture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).color) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MaterialMap),
            "::",
            stringify!(color)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MaterialMap),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub shader: Shader,
    pub maps: *mut MaterialMap,
    pub params: [f32; 4usize],
}
#[test]
fn bindgen_test_layout_Material() {
    const UNINIT: ::std::mem::MaybeUninit<Material> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Material>(),
        40usize,
        concat!("Size of: ", stringify!(Material))
    );
    assert_eq!(
        ::std::mem::align_of::<Material>(),
        8usize,
        concat!("Alignment of ", stringify!(Material))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shader) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Material),
            "::",
            stringify!(shader)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maps) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Material),
            "::",
            stringify!(maps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).params) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Material),
            "::",
            stringify!(params)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
#[test]
fn bindgen_test_layout_Transform() {
    const UNINIT: ::std::mem::MaybeUninit<Transform> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Transform>(),
        40usize,
        concat!("Size of: ", stringify!(Transform))
    );
    assert_eq!(
        ::std::mem::align_of::<Transform>(),
        4usize,
        concat!("Alignment of ", stringify!(Transform))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).translation) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Transform),
            "::",
            stringify!(translation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rotation) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Transform),
            "::",
            stringify!(rotation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scale) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Transform),
            "::",
            stringify!(scale)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoneInfo {
    pub name: [u8; 32usize],
    pub parent: i32,
}
#[test]
fn bindgen_test_layout_BoneInfo() {
    const UNINIT: ::std::mem::MaybeUninit<BoneInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<BoneInfo>(),
        36usize,
        concat!("Size of: ", stringify!(BoneInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<BoneInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(BoneInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BoneInfo),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(BoneInfo),
            "::",
            stringify!(parent)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
    pub transform: Matrix,
    pub meshCount: i32,
    pub materialCount: i32,
    pub meshes: *mut Mesh,
    pub materials: *mut Material,
    pub meshMaterial: *mut i32,
    pub boneCount: i32,
    pub bones: *mut BoneInfo,
    pub bindPose: *mut Transform,
}
#[test]
fn bindgen_test_layout_Model() {
    const UNINIT: ::std::mem::MaybeUninit<Model> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Model>(),
        120usize,
        concat!("Size of: ", stringify!(Model))
    );
    assert_eq!(
        ::std::mem::align_of::<Model>(),
        8usize,
        concat!("Alignment of ", stringify!(Model))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).transform) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(transform)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).meshCount) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(meshCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).materialCount) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(materialCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).meshes) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(meshes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).materials) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(materials)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).meshMaterial) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(meshMaterial)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneCount) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(boneCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bones) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(bones)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bindPose) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(Model),
            "::",
            stringify!(bindPose)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelAnimation {
    pub boneCount: i32,
    pub frameCount: i32,
    pub bones: *mut BoneInfo,
    pub framePoses: *mut *mut Transform,
    pub name: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_ModelAnimation() {
    const UNINIT: ::std::mem::MaybeUninit<ModelAnimation> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ModelAnimation>(),
        56usize,
        concat!("Size of: ", stringify!(ModelAnimation))
    );
    assert_eq!(
        ::std::mem::align_of::<ModelAnimation>(),
        8usize,
        concat!("Alignment of ", stringify!(ModelAnimation))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boneCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(boneCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(frameCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bones) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(bones)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).framePoses) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(framePoses)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ModelAnimation),
            "::",
            stringify!(name)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}
#[test]
fn bindgen_test_layout_Ray() {
    const UNINIT: ::std::mem::MaybeUninit<Ray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Ray>(),
        24usize,
        concat!("Size of: ", stringify!(Ray))
    );
    assert_eq!(
        ::std::mem::align_of::<Ray>(),
        4usize,
        concat!("Alignment of ", stringify!(Ray))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).position) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Ray),
            "::",
            stringify!(position)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).direction) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Ray),
            "::",
            stringify!(direction)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayCollision {
    pub hit: bool,
    pub distance: f32,
    pub point: Vector3,
    pub normal: Vector3,
}
#[test]
fn bindgen_test_layout_RayCollision() {
    const UNINIT: ::std::mem::MaybeUninit<RayCollision> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RayCollision>(),
        32usize,
        concat!("Size of: ", stringify!(RayCollision))
    );
    assert_eq!(
        ::std::mem::align_of::<RayCollision>(),
        4usize,
        concat!("Alignment of ", stringify!(RayCollision))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hit) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(hit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).distance) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(distance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).point) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(point)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).normal) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(RayCollision),
            "::",
            stringify!(normal)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}
#[test]
fn bindgen_test_layout_BoundingBox() {
    const UNINIT: ::std::mem::MaybeUninit<BoundingBox> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<BoundingBox>(),
        24usize,
        concat!("Size of: ", stringify!(BoundingBox))
    );
    assert_eq!(
        ::std::mem::align_of::<BoundingBox>(),
        4usize,
        concat!("Alignment of ", stringify!(BoundingBox))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BoundingBox),
            "::",
            stringify!(min)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(BoundingBox),
            "::",
            stringify!(max)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub frameCount: u32,
    pub sampleRate: u32,
    pub sampleSize: u32,
    pub channels: u32,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_Wave() {
    const UNINIT: ::std::mem::MaybeUninit<Wave> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Wave>(),
        24usize,
        concat!("Size of: ", stringify!(Wave))
    );
    assert_eq!(
        ::std::mem::align_of::<Wave>(),
        8usize,
        concat!("Alignment of ", stringify!(Wave))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(frameCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleRate) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(sampleRate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(sampleSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channels) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Wave),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rAudioProcessor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStream {
    pub buffer: *mut rAudioBuffer,
    pub processor: *mut rAudioProcessor,
    pub sampleRate: u32,
    pub sampleSize: u32,
    pub channels: u32,
}
#[test]
fn bindgen_test_layout_AudioStream() {
    const UNINIT: ::std::mem::MaybeUninit<AudioStream> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AudioStream>(),
        32usize,
        concat!("Size of: ", stringify!(AudioStream))
    );
    assert_eq!(
        ::std::mem::align_of::<AudioStream>(),
        8usize,
        concat!("Alignment of ", stringify!(AudioStream))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).processor) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(processor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleRate) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(sampleRate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sampleSize) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(sampleSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channels) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AudioStream),
            "::",
            stringify!(channels)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
    pub stream: AudioStream,
    pub frameCount: u32,
}
#[test]
fn bindgen_test_layout_Sound() {
    const UNINIT: ::std::mem::MaybeUninit<Sound> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Sound>(),
        40usize,
        concat!("Size of: ", stringify!(Sound))
    );
    assert_eq!(
        ::std::mem::align_of::<Sound>(),
        8usize,
        concat!("Alignment of ", stringify!(Sound))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stream) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Sound),
            "::",
            stringify!(stream)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Sound),
            "::",
            stringify!(frameCount)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
    pub stream: AudioStream,
    pub frameCount: u32,
    pub looping: bool,
    pub ctxType: i32,
    pub ctxData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_Music() {
    const UNINIT: ::std::mem::MaybeUninit<Music> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Music>(),
        56usize,
        concat!("Size of: ", stringify!(Music))
    );
    assert_eq!(
        ::std::mem::align_of::<Music>(),
        8usize,
        concat!("Alignment of ", stringify!(Music))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stream) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(stream)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frameCount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(frameCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).looping) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(looping)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ctxType) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(ctxType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ctxData) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Music),
            "::",
            stringify!(ctxData)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrDeviceInfo {
    pub hResolution: i32,
    pub vResolution: i32,
    pub hScreenSize: f32,
    pub vScreenSize: f32,
    pub vScreenCenter: f32,
    pub eyeToScreenDistance: f32,
    pub lensSeparationDistance: f32,
    pub interpupillaryDistance: f32,
    pub lensDistortionValues: [f32; 4usize],
    pub chromaAbCorrection: [f32; 4usize],
}
#[test]
fn bindgen_test_layout_VrDeviceInfo() {
    const UNINIT: ::std::mem::MaybeUninit<VrDeviceInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VrDeviceInfo>(),
        64usize,
        concat!("Size of: ", stringify!(VrDeviceInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<VrDeviceInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(VrDeviceInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hResolution) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(hResolution)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vResolution) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(vResolution)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hScreenSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(hScreenSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vScreenSize) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(vScreenSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vScreenCenter) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(vScreenCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).eyeToScreenDistance) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(eyeToScreenDistance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lensSeparationDistance) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(lensSeparationDistance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interpupillaryDistance) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(interpupillaryDistance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lensDistortionValues) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(lensDistortionValues)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).chromaAbCorrection) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(VrDeviceInfo),
            "::",
            stringify!(chromaAbCorrection)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2usize],
    pub viewOffset: [Matrix; 2usize],
    pub leftLensCenter: [f32; 2usize],
    pub rightLensCenter: [f32; 2usize],
    pub leftScreenCenter: [f32; 2usize],
    pub rightScreenCenter: [f32; 2usize],
    pub scale: [f32; 2usize],
    pub scaleIn: [f32; 2usize],
}
#[test]
fn bindgen_test_layout_VrStereoConfig() {
    const UNINIT: ::std::mem::MaybeUninit<VrStereoConfig> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VrStereoConfig>(),
        304usize,
        concat!("Size of: ", stringify!(VrStereoConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<VrStereoConfig>(),
        4usize,
        concat!("Alignment of ", stringify!(VrStereoConfig))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).projection) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(projection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).viewOffset) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(viewOffset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).leftLensCenter) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(leftLensCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rightLensCenter) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(rightLensCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).leftScreenCenter) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(leftScreenCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rightScreenCenter) as usize - ptr as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(rightScreenCenter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scale) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(scale)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scaleIn) as usize - ptr as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(VrStereoConfig),
            "::",
            stringify!(scaleIn)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FilePathList {
    pub capacity: u32,
    pub count: u32,
    pub paths: *mut *mut u8,
}
#[test]
fn bindgen_test_layout_FilePathList() {
    const UNINIT: ::std::mem::MaybeUninit<FilePathList> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<FilePathList>(),
        16usize,
        concat!("Size of: ", stringify!(FilePathList))
    );
    assert_eq!(
        ::std::mem::align_of::<FilePathList>(),
        8usize,
        concat!("Alignment of ", stringify!(FilePathList))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).capacity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FilePathList),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FilePathList),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).paths) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FilePathList),
            "::",
            stringify!(paths)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEvent {
    pub frame: u32,
    pub type_: u32,
    pub params: [i32; 4usize],
}
#[test]
fn bindgen_test_layout_AutomationEvent() {
    const UNINIT: ::std::mem::MaybeUninit<AutomationEvent> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AutomationEvent>(),
        24usize,
        concat!("Size of: ", stringify!(AutomationEvent))
    );
    assert_eq!(
        ::std::mem::align_of::<AutomationEvent>(),
        4usize,
        concat!("Alignment of ", stringify!(AutomationEvent))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frame) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEvent),
            "::",
            stringify!(frame)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEvent),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).params) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEvent),
            "::",
            stringify!(params)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEventList {
    pub capacity: u32,
    pub count: u32,
    pub events: *mut AutomationEvent,
}
#[test]
fn bindgen_test_layout_AutomationEventList() {
    const UNINIT: ::std::mem::MaybeUninit<AutomationEventList> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AutomationEventList>(),
        16usize,
        concat!("Size of: ", stringify!(AutomationEventList))
    );
    assert_eq!(
        ::std::mem::align_of::<AutomationEventList>(),
        8usize,
        concat!("Alignment of ", stringify!(AutomationEventList))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).capacity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEventList),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEventList),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).events) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AutomationEventList),
            "::",
            stringify!(events)
        )
    );
}
