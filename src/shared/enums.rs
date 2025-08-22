/// System/Window config flags
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConfigFlags {
    /// Set to try enabling V-Sync on GPU
    VsyncHint = 64,
    /// Set to run program in fullscreen
    FullscreenMode = 2,
    /// Set to allow resizable window
    WindowResizable = 4,
    /// Set to disable window decoration (frame and buttons)
    WindowUndecorated = 8,
    /// Set to hide window
    WindowHidden = 128,
    /// Set to minimize window (iconify)
    WindowMinimized = 512,
    /// Set to maximize window (expanded to monitor)
    WindowMaximized = 1024,
    /// Set to window non focused
    WindowUnfocused = 2048,
    /// Set to window always on top
    WindowTopmost = 4096,
    /// Set to allow windows running while minimized
    WindowAlwaysRun = 256,
    /// Set to allow transparent framebuffer
    WindowTransparent = 16,
    /// Set to support HighDPI
    WindowHighdpi = 8192,
    /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    WindowMousePassthrough = 16384,
    /// Set to run program in borderless windowed mode
    BorderlessWindowedMode = 32768,
    /// Set to try enabling MSAA 4X
    Msaa4xHint = 32,
    /// Set to try enabling interlaced video format (for V3D)
    InterlacedHint = 65536,
}

/// Trace log level
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TraceLogLevel {
    /// Display all logs
    All = 0,
    /// Trace logging, intended for internal use only
    Trace = 1,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug = 2,
    /// Info logging, used for program execution info
    Info = 3,
    /// Warning logging, used on recoverable failures
    Warning = 4,
    /// Error logging, used on unrecoverable failures
    Error = 5,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal = 6,
    /// Disable logging
    None = 7,
}

/// Keyboard keys (US keyboard layout)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    /// Key: NULL, used for no key pressed
    Null = 0,
    /// Key: '
    Apostrophe = 39,
    /// Key: ,
    Comma = 44,
    /// Key: -
    Minus = 45,
    /// Key: .
    Period = 46,
    /// Key: /
    Slash = 47,
    /// Key: 0
    Zero = 48,
    /// Key: 1
    One = 49,
    /// Key: 2
    Two = 50,
    /// Key: 3
    Three = 51,
    /// Key: 4
    Four = 52,
    /// Key: 5
    Five = 53,
    /// Key: 6
    Six = 54,
    /// Key: 7
    Seven = 55,
    /// Key: 8
    Eight = 56,
    /// Key: 9
    Nine = 57,
    /// Key: ;
    Semicolon = 59,
    /// Key: =
    Equal = 61,
    /// Key: A | a
    A = 65,
    /// Key: B | b
    B = 66,
    /// Key: C | c
    C = 67,
    /// Key: D | d
    D = 68,
    /// Key: E | e
    E = 69,
    /// Key: F | f
    F = 70,
    /// Key: G | g
    G = 71,
    /// Key: H | h
    H = 72,
    /// Key: I | i
    I = 73,
    /// Key: J | j
    J = 74,
    /// Key: K | k
    K = 75,
    /// Key: L | l
    L = 76,
    /// Key: M | m
    M = 77,
    /// Key: N | n
    N = 78,
    /// Key: O | o
    O = 79,
    /// Key: P | p
    P = 80,
    /// Key: Q | q
    Q = 81,
    /// Key: R | r
    R = 82,
    /// Key: S | s
    S = 83,
    /// Key: T | t
    T = 84,
    /// Key: U | u
    U = 85,
    /// Key: V | v
    V = 86,
    /// Key: W | w
    W = 87,
    /// Key: X | x
    X = 88,
    /// Key: Y | y
    Y = 89,
    /// Key: Z | z
    Z = 90,
    /// Key: [
    LeftBracket = 91,
    /// Key: '\'
    Backslash = 92,
    /// Key: ]
    RightBracket = 93,
    /// Key: `
    Grave = 96,
    /// Key: Space
    Space = 32,
    /// Key: Esc
    Escape = 256,
    /// Key: Enter
    Enter = 257,
    /// Key: Tab
    Tab = 258,
    /// Key: Backspace
    Backspace = 259,
    /// Key: Ins
    Insert = 260,
    /// Key: Del
    Delete = 261,
    /// Key: Cursor right
    Right = 262,
    /// Key: Cursor left
    Left = 263,
    /// Key: Cursor down
    Down = 264,
    /// Key: Cursor up
    Up = 265,
    /// Key: Page up
    PageUp = 266,
    /// Key: Page down
    PageDown = 267,
    /// Key: Home
    Home = 268,
    /// Key: End
    End = 269,
    /// Key: Caps lock
    CapsLock = 280,
    /// Key: Scroll down
    ScrollLock = 281,
    /// Key: Num lock
    NumLock = 282,
    /// Key: Print screen
    PrintScreen = 283,
    /// Key: Pause
    Pause = 284,
    /// Key: F1
    F1 = 290,
    /// Key: F2
    F2 = 291,
    /// Key: F3
    F3 = 292,
    /// Key: F4
    F4 = 293,
    /// Key: F5
    F5 = 294,
    /// Key: F6
    F6 = 295,
    /// Key: F7
    F7 = 296,
    /// Key: F8
    F8 = 297,
    /// Key: F9
    F9 = 298,
    /// Key: F10
    F10 = 299,
    /// Key: F11
    F11 = 300,
    /// Key: F12
    F12 = 301,
    /// Key: Shift left
    LeftShift = 340,
    /// Key: Control left
    LeftControl = 341,
    /// Key: Alt left
    LeftAlt = 342,
    /// Key: Super left
    LeftSuper = 343,
    /// Key: Shift right
    RightShift = 344,
    /// Key: Control right
    RightControl = 345,
    /// Key: Alt right
    RightAlt = 346,
    /// Key: Super right
    RightSuper = 347,
    /// Key: KB menu
    KbMenu = 348,
    /// Key: Keypad 0
    Kp0 = 320,
    /// Key: Keypad 1
    Kp1 = 321,
    /// Key: Keypad 2
    Kp2 = 322,
    /// Key: Keypad 3
    Kp3 = 323,
    /// Key: Keypad 4
    Kp4 = 324,
    /// Key: Keypad 5
    Kp5 = 325,
    /// Key: Keypad 6
    Kp6 = 326,
    /// Key: Keypad 7
    Kp7 = 327,
    /// Key: Keypad 8
    Kp8 = 328,
    /// Key: Keypad 9
    Kp9 = 329,
    /// Key: Keypad .
    KpDecimal = 330,
    /// Key: Keypad /
    KpDivide = 331,
    /// Key: Keypad *
    KpMultiply = 332,
    /// Key: Keypad -
    KpSubtract = 333,
    /// Key: Keypad +
    KpAdd = 334,
    /// Key: Keypad Enter
    KpEnter = 335,
    /// Key: Keypad =
    KpEqual = 336,
    /// Key: Android back button
    Back = 4,
    /// Key: Android volume up button
    VolumeUp = 24,
    /// Key: Android volume down button
    VolumeDown = 25,
}

/// Mouse buttons
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Mouse button left
    Left = 0,
    /// Mouse button right
    Right = 1,
    /// Mouse button middle (pressed wheel)
    Middle = 2,
    /// Mouse button side (advanced mouse device)
    Side = 3,
    /// Mouse button extra (advanced mouse device)
    Extra = 4,
    /// Mouse button forward (advanced mouse device)
    Forward = 5,
    /// Mouse button back (advanced mouse device)
    Back = 6,
}

/// Mouse cursor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseCursor {
    /// Default pointer shape
    Default = 0,
    /// Arrow shape
    Arrow = 1,
    /// Text writing cursor shape
    Ibeam = 2,
    /// Cross shape
    Crosshair = 3,
    /// Pointing hand cursor
    PointingHand = 4,
    /// Horizontal resize/move arrow shape
    ResizeEw = 5,
    /// Vertical resize/move arrow shape
    ResizeNs = 6,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNwse = 7,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNesw = 8,
    /// The omnidirectional resize/move cursor shape
    ResizeAll = 9,
    /// The operation-not-allowed shape
    NotAllowed = 10,
}

/// Gamepad buttons
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    /// Unknown button, just for error checking
    Unknown = 0,
    /// Gamepad left DPAD up button
    LeftFaceUp = 1,
    /// Gamepad left DPAD right button
    LeftFaceRight = 2,
    /// Gamepad left DPAD down button
    LeftFaceDown = 3,
    /// Gamepad left DPAD left button
    LeftFaceLeft = 4,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp = 5,
    /// Gamepad right button right (i.e. PS3: Square, Xbox: X)
    RightFaceRight = 6,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown = 7,
    /// Gamepad right button left (i.e. PS3: Circle, Xbox: B)
    RightFaceLeft = 8,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1 = 9,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2 = 10,
    /// Gamepad top/back trigger right (one), it could be a trailing button
    RightTrigger1 = 11,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2 = 12,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft = 13,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle = 14,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight = 15,
    /// Gamepad joystick pressed button left
    LeftThumb = 16,
    /// Gamepad joystick pressed button right
    RightThumb = 17,
}

/// Gamepad axis
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX = 0,
    /// Gamepad left stick Y axis
    LeftY = 1,
    /// Gamepad right stick X axis
    RightX = 2,
    /// Gamepad right stick Y axis
    RightY = 3,
    /// Gamepad back trigger left, pressure level: [1..-1]
    LeftTrigger = 4,
    /// Gamepad back trigger right, pressure level: [1..-1]
    RightTrigger = 5,
}

/// Material map index
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MaterialMapIndex {
    /// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
    Albedo = 0,
    /// Metalness material (same as: MATERIAL_MAP_SPECULAR)
    Metalness = 1,
    /// Normal material
    Normal = 2,
    /// Roughness material
    Roughness = 3,
    /// Ambient occlusion material
    Occlusion = 4,
    /// Emission material
    Emission = 5,
    /// Heightmap material
    Height = 6,
    /// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Cubemap = 7,
    /// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Irradiance = 8,
    /// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Prefilter = 9,
    /// Brdf material
    Brdf = 10,
}

/// Shader location index
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderLocationIndex {
    /// Shader location: vertex attribute: position
    VertexPosition = 0,
    /// Shader location: vertex attribute: texcoord01
    VertexTexcoord01 = 1,
    /// Shader location: vertex attribute: texcoord02
    VertexTexcoord02 = 2,
    /// Shader location: vertex attribute: normal
    VertexNormal = 3,
    /// Shader location: vertex attribute: tangent
    VertexTangent = 4,
    /// Shader location: vertex attribute: color
    VertexColor = 5,
    /// Shader location: matrix uniform: model-view-projection
    MatrixMvp = 6,
    /// Shader location: matrix uniform: view (camera transform)
    MatrixView = 7,
    /// Shader location: matrix uniform: projection
    MatrixProjection = 8,
    /// Shader location: matrix uniform: model (transform)
    MatrixModel = 9,
    /// Shader location: matrix uniform: normal
    MatrixNormal = 10,
    /// Shader location: vector uniform: view
    VectorView = 11,
    /// Shader location: vector uniform: diffuse color
    ColorDiffuse = 12,
    /// Shader location: vector uniform: specular color
    ColorSpecular = 13,
    /// Shader location: vector uniform: ambient color
    ColorAmbient = 14,
    /// Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
    MapAlbedo = 15,
    /// Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
    MapMetalness = 16,
    /// Shader location: sampler2d texture: normal
    MapNormal = 17,
    /// Shader location: sampler2d texture: roughness
    MapRoughness = 18,
    /// Shader location: sampler2d texture: occlusion
    MapOcclusion = 19,
    /// Shader location: sampler2d texture: emission
    MapEmission = 20,
    /// Shader location: sampler2d texture: height
    MapHeight = 21,
    /// Shader location: samplerCube texture: cubemap
    MapCubemap = 22,
    /// Shader location: samplerCube texture: irradiance
    MapIrradiance = 23,
    /// Shader location: samplerCube texture: prefilter
    MapPrefilter = 24,
    /// Shader location: sampler2d texture: brdf
    MapBrdf = 25,
}

/// Shader uniform data type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderUniformDataType {
    /// Shader uniform type: float
    Float = 0,
    /// Shader uniform type: vec2 (2 float)
    Vec2 = 1,
    /// Shader uniform type: vec3 (3 float)
    Vec3 = 2,
    /// Shader uniform type: vec4 (4 float)
    Vec4 = 3,
    /// Shader uniform type: int
    Int = 4,
    /// Shader uniform type: ivec2 (2 int)
    Ivec2 = 5,
    /// Shader uniform type: ivec3 (3 int)
    Ivec3 = 6,
    /// Shader uniform type: ivec4 (4 int)
    Ivec4 = 7,
    /// Shader uniform type: sampler2d
    Sampler2d = 8,
}

/// Shader attribute data types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderAttributeDataType {
    /// Shader attribute type: float
    Float = 0,
    /// Shader attribute type: vec2 (2 float)
    Vec2 = 1,
    /// Shader attribute type: vec3 (3 float)
    Vec3 = 2,
    /// Shader attribute type: vec4 (4 float)
    Vec4 = 3,
}

/// Pixel formats
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8 bit per pixel (no alpha)
    Grayscale = 1,
    /// 8*2 bpp (2 channels)
    GrayAlpha = 2,
    /// 16 bpp
    R5g6b5 = 3,
    /// 24 bpp
    R8g8b8 = 4,
    /// 16 bpp (1 bit alpha)
    R5g5b5a1 = 5,
    /// 16 bpp (4 bit alpha)
    R4g4b4a4 = 6,
    /// 32 bpp
    R8g8b8a8 = 7,
    /// 32 bpp (1 channel - float)
    R32 = 8,
    /// 32*3 bpp (3 channels - float)
    R32g32b32 = 9,
    /// 32*4 bpp (4 channels - float)
    R32g32b32a32 = 10,
    /// 16 bpp (1 channel - half float)
    R16 = 11,
    /// 16*3 bpp (3 channels - half float)
    R16g16b16 = 12,
    /// 16*4 bpp (4 channels - half float)
    R16g16b16a16 = 13,
    /// 4 bpp (no alpha)
    Dxt1Rgb = 14,
    /// 4 bpp (1 bit alpha)
    Dxt1Rgba = 15,
    /// 8 bpp
    Dxt3Rgba = 16,
    /// 8 bpp
    Dxt5Rgba = 17,
    /// 4 bpp
    Etc1Rgb = 18,
    /// 4 bpp
    Etc2Rgb = 19,
    /// 8 bpp
    Etc2EacRgba = 20,
    /// 4 bpp
    PvrtRgb = 21,
    /// 4 bpp
    PvrtRgba = 22,
    /// 8 bpp
    Astc4x4Rgba = 23,
    /// 2 bpp
    Astc8x8Rgba = 24,
}

/// Texture parameters: filter mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureFilter {
    /// No filter, just pixel approximation
    Point = 0,
    /// Linear filtering
    Bilinear = 1,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear = 2,
    /// Anisotropic filtering 4x
    Anisotropic4x = 3,
    /// Anisotropic filtering 8x
    Anisotropic8x = 4,
    /// Anisotropic filtering 16x
    Anisotropic16x = 5,
}

/// Texture parameters: wrap mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureWrap {
    /// Repeats texture in tiled mode
    Repeat = 0,
    /// Clamps texture to edge pixel in tiled mode
    Clamp = 1,
    /// Mirrors and repeats the texture in tiled mode
    MirrorRepeat = 2,
    /// Mirrors and clamps to border the texture in tiled mode
    MirrorClamp = 3,
}

/// Cubemap layouts
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CubemapLayout {
    /// Automatically detect layout type
    AutoDetect = 0,
    /// Layout is defined by a vertical line with faces
    LineVertical = 1,
    /// Layout is defined by a horizontal line with faces
    LineHorizontal = 2,
    /// Layout is defined by a 3x4 cross with cubemap faces
    CrossThreeByFour = 3,
    /// Layout is defined by a 4x3 cross with cubemap faces
    CrossFourByThree = 4,
    /// Layout is defined by a panorama image (equirrectangular map)
    Panorama = 5,
}

/// Font type, defines generation method
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FontType {
    /// Default font generation, anti-aliased
    Default = 0,
    /// Bitmap font generation, no anti-aliasing
    Bitmap = 1,
    /// SDF font generation, requires external shader
    Sdf = 2,
}

/// Color blending modes (pre-defined)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// Blend textures considering alpha (default)
    Alpha = 0,
    /// Blend textures adding colors
    Additive = 1,
    /// Blend textures multiplying colors
    Multiplied = 2,
    /// Blend textures adding colors (alternative)
    AddColors = 3,
    /// Blend textures subtracting colors (alternative)
    SubtractColors = 4,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply = 5,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom = 6,
    /// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate = 7,
}

/// Gesture
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Gesture {
    /// No gesture
    None = 0,
    /// Tap gesture
    Tap = 1,
    /// Double tap gesture
    Doubletap = 2,
    /// Hold gesture
    Hold = 4,
    /// Drag gesture
    Drag = 8,
    /// Swipe right gesture
    SwipeRight = 16,
    /// Swipe left gesture
    SwipeLeft = 32,
    /// Swipe up gesture
    SwipeUp = 64,
    /// Swipe down gesture
    SwipeDown = 128,
    /// Pinch in gesture
    PinchIn = 256,
    /// Pinch out gesture
    PinchOut = 512,
}

/// Camera system modes
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CameraMode {
    /// Custom camera
    Custom = 0,
    /// Free camera
    Free = 1,
    /// Orbital camera
    Orbital = 2,
    /// First person camera
    FirstPerson = 3,
    /// Third person camera
    ThirdPerson = 4,
}

/// Camera projection
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CameraProjection {
    /// Perspective projection
    Perspective = 0,
    /// Orthographic projection
    Orthographic = 1,
}

/// N-patch layout
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NPatchLayout {
    /// Npatch layout: 3x3 tiles
    NinePatch = 0,
    /// Npatch layout: 1x3 tiles
    ThreePatchVertical = 1,
    /// Npatch layout: 3x1 tiles
    ThreePatchHorizontal = 2,
}
