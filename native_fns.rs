use crate::structs::*;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __builtin_va_list;
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
pub type TraceLogCallback = ::std::option::Option<
    unsafe extern "C" fn(
        logLevel: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ),
>;
pub type LoadFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar,
>;
pub type SaveFileDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
    ) -> bool,
>;
pub type LoadFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type SaveFileTextCallback = ::std::option::Option<
    unsafe extern "C" fn(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool,
>;
extern "C" {
    pub fn InitWindow(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn CloseWindow();
}
extern "C" {
    pub fn WindowShouldClose() -> bool;
}
extern "C" {
    pub fn IsWindowReady() -> bool;
}
extern "C" {
    pub fn IsWindowFullscreen() -> bool;
}
extern "C" {
    pub fn IsWindowHidden() -> bool;
}
extern "C" {
    pub fn IsWindowMinimized() -> bool;
}
extern "C" {
    pub fn IsWindowMaximized() -> bool;
}
extern "C" {
    pub fn IsWindowFocused() -> bool;
}
extern "C" {
    pub fn IsWindowResized() -> bool;
}
extern "C" {
    pub fn IsWindowState(flag: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn SetWindowState(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ClearWindowState(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ToggleFullscreen();
}
extern "C" {
    pub fn ToggleBorderlessWindowed();
}
extern "C" {
    pub fn MaximizeWindow();
}
extern "C" {
    pub fn MinimizeWindow();
}
extern "C" {
    pub fn RestoreWindow();
}
extern "C" {
    pub fn SetWindowIcon(image: Image);
}
extern "C" {
    pub fn SetWindowIcons(images: *mut Image, count: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowTitle(title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetWindowPosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMonitor(monitor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMinSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowMaxSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetWindowOpacity(opacity: f32);
}
extern "C" {
    pub fn SetWindowFocused();
}
extern "C" {
    pub fn GetWindowHandle() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GetScreenWidth() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetScreenHeight() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetRenderWidth() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetRenderHeight() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCurrentMonitor() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPosition(monitor: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn GetMonitorWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPhysicalWidth(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorPhysicalHeight(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMonitorRefreshRate(monitor: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetWindowPosition() -> Vector2;
}
extern "C" {
    pub fn GetWindowScaleDPI() -> Vector2;
}
extern "C" {
    pub fn GetMonitorName(monitor: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetClipboardText(text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn GetClipboardText() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn EnableEventWaiting();
}
extern "C" {
    pub fn DisableEventWaiting();
}
extern "C" {
    pub fn ShowCursor();
}
extern "C" {
    pub fn HideCursor();
}
extern "C" {
    pub fn IsCursorHidden() -> bool;
}
extern "C" {
    pub fn EnableCursor();
}
extern "C" {
    pub fn DisableCursor();
}
extern "C" {
    pub fn IsCursorOnScreen() -> bool;
}
extern "C" {
    pub fn ClearBackground(color: Color);
}
extern "C" {
    pub fn BeginDrawing();
}
extern "C" {
    pub fn EndDrawing();
}
extern "C" {
    pub fn BeginMode2D(camera: Camera2D);
}
extern "C" {
    pub fn EndMode2D();
}
extern "C" {
    pub fn BeginMode3D(camera: Camera3D);
}
extern "C" {
    pub fn EndMode3D();
}
extern "C" {
    pub fn BeginTextureMode(target: RenderTexture2D);
}
extern "C" {
    pub fn EndTextureMode();
}
extern "C" {
    pub fn BeginShaderMode(shader: Shader);
}
extern "C" {
    pub fn EndShaderMode();
}
extern "C" {
    pub fn BeginBlendMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn EndBlendMode();
}
extern "C" {
    pub fn BeginScissorMode(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn EndScissorMode();
}
extern "C" {
    pub fn BeginVrStereoMode(config: VrStereoConfig);
}
extern "C" {
    pub fn EndVrStereoMode();
}
extern "C" {
    pub fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;
}
extern "C" {
    pub fn UnloadVrStereoConfig(config: VrStereoConfig);
}
extern "C" {
    pub fn LoadShader(
        vsFileName: *const ::std::os::raw::c_char,
        fsFileName: *const ::std::os::raw::c_char,
    ) -> Shader;
}
extern "C" {
    pub fn LoadShaderFromMemory(
        vsCode: *const ::std::os::raw::c_char,
        fsCode: *const ::std::os::raw::c_char,
    ) -> Shader;
}
extern "C" {
    pub fn IsShaderReady(shader: Shader) -> bool;
}
extern "C" {
    pub fn GetShaderLocation(
        shader: Shader,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetShaderLocationAttrib(
        shader: Shader,
        attribName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetShaderValue(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShaderValueV(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_void,
        uniformType: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SetShaderValueMatrix(shader: Shader, locIndex: ::std::os::raw::c_int, mat: Matrix);
}
extern "C" {
    pub fn SetShaderValueTexture(
        shader: Shader,
        locIndex: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
extern "C" {
    pub fn UnloadShader(shader: Shader);
}
extern "C" {
    pub fn GetMouseRay(mousePosition: Vector2, camera: Camera) -> Ray;
}
extern "C" {
    pub fn GetCameraMatrix(camera: Camera) -> Matrix;
}
extern "C" {
    pub fn GetCameraMatrix2D(camera: Camera2D) -> Matrix;
}
extern "C" {
    pub fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2;
}
extern "C" {
    pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;
}
extern "C" {
    pub fn GetWorldToScreenEx(
        position: Vector3,
        camera: Camera,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Vector2;
}
extern "C" {
    pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2;
}
extern "C" {
    pub fn SetTargetFPS(fps: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetFrameTime() -> f32;
}
extern "C" {
    pub fn GetTime() -> f64;
}
extern "C" {
    pub fn GetFPS() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SwapScreenBuffer();
}
extern "C" {
    pub fn PollInputEvents();
}
extern "C" {
    pub fn WaitTime(seconds: f64);
}
extern "C" {
    pub fn SetRandomSeed(seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn GetRandomValue(
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LoadRandomSequence(
        count: ::std::os::raw::c_uint,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn UnloadRandomSequence(sequence: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn TakeScreenshot(fileName: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn SetConfigFlags(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn OpenURL(url: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn TraceLog(logLevel: ::std::os::raw::c_int, text: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn SetTraceLogLevel(logLevel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn MemAlloc(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn MemRealloc(
        ptr: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn MemFree(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn SetTraceLogCallback(callback: TraceLogCallback);
}
extern "C" {
    pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback);
}
extern "C" {
    pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback);
}
extern "C" {
    pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback);
}
extern "C" {
    pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback);
}
extern "C" {
    pub fn LoadFileData(
        fileName: *const ::std::os::raw::c_char,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn UnloadFileData(data: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn SaveFileData(
        fileName: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn ExportDataAsCode(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fileName: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn LoadFileText(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn UnloadFileText(text: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn SaveFileText(
        fileName: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn FileExists(fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn DirectoryExists(dirPath: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsFileExtension(
        fileName: *const ::std::os::raw::c_char,
        ext: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn GetFileLength(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetFileExtension(
        fileName: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetFileName(filePath: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetFileNameWithoutExt(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetDirectoryPath(
        filePath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetPrevDirectoryPath(
        dirPath: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetWorkingDirectory() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetApplicationDirectory() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ChangeDirectory(dir: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn IsPathFile(path: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn LoadDirectoryFiles(dirPath: *const ::std::os::raw::c_char) -> FilePathList;
}
extern "C" {
    pub fn LoadDirectoryFilesEx(
        basePath: *const ::std::os::raw::c_char,
        filter: *const ::std::os::raw::c_char,
        scanSubdirs: bool,
    ) -> FilePathList;
}
extern "C" {
    pub fn UnloadDirectoryFiles(files: FilePathList);
}
extern "C" {
    pub fn IsFileDropped() -> bool;
}
extern "C" {
    pub fn LoadDroppedFiles() -> FilePathList;
}
extern "C" {
    pub fn UnloadDroppedFiles(files: FilePathList);
}
extern "C" {
    pub fn GetFileModTime(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn CompressData(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        compDataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn DecompressData(
        compData: *const ::std::os::raw::c_uchar,
        compDataSize: ::std::os::raw::c_int,
        dataSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn EncodeDataBase64(
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        outputSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DecodeDataBase64(
        data: *const ::std::os::raw::c_uchar,
        outputSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn LoadAutomationEventList(fileName: *const ::std::os::raw::c_char) -> AutomationEventList;
}
extern "C" {
    pub fn UnloadAutomationEventList(list: *mut AutomationEventList);
}
extern "C" {
    pub fn ExportAutomationEventList(
        list: AutomationEventList,
        fileName: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn SetAutomationEventList(list: *mut AutomationEventList);
}
extern "C" {
    pub fn SetAutomationEventBaseFrame(frame: ::std::os::raw::c_int);
}
extern "C" {
    pub fn StartAutomationEventRecording();
}
extern "C" {
    pub fn StopAutomationEventRecording();
}
extern "C" {
    pub fn PlayAutomationEvent(event: AutomationEvent);
}
use crate::KeyboardKey;
extern "C" {
    pub fn IsKeyPressed(key: KeyboardKey) -> bool;
}
extern "C" {
    pub fn IsKeyPressedRepeat(key: KeyboardKey) -> bool;
}
extern "C" {
    pub fn IsKeyDown(key: KeyboardKey) -> bool;
}
extern "C" {
    pub fn IsKeyReleased(key: KeyboardKey) -> bool;
}
extern "C" {
    pub fn IsKeyUp(key: KeyboardKey) -> bool;
}
extern "C" {
    pub fn GetKeyPressed() -> KeyboardKey;
}
extern "C" {
    pub fn GetCharPressed() -> KeyboardKey;
}
extern "C" {
    pub fn SetExitKey(key: KeyboardKey);
}
extern "C" {
    pub fn IsGamepadAvailable(gamepad: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetGamepadName(gamepad: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn IsGamepadButtonPressed(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonDown(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonReleased(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn IsGamepadButtonUp(gamepad: ::std::os::raw::c_int, button: ::std::os::raw::c_int)
        -> bool;
}
extern "C" {
    pub fn GetGamepadButtonPressed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGamepadAxisCount(gamepad: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGamepadAxisMovement(
        gamepad: ::std::os::raw::c_int,
        axis: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn SetGamepadMappings(mappings: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IsMouseButtonPressed(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonDown(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonReleased(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn IsMouseButtonUp(button: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn GetMouseX() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMouseY() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMousePosition() -> Vector2;
}
extern "C" {
    pub fn GetMouseDelta() -> Vector2;
}
extern "C" {
    pub fn SetMousePosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetMouseOffset(offsetX: ::std::os::raw::c_int, offsetY: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetMouseScale(scaleX: f32, scaleY: f32);
}
extern "C" {
    pub fn GetMouseWheelMove() -> f32;
}
extern "C" {
    pub fn GetMouseWheelMoveV() -> Vector2;
}
extern "C" {
    pub fn SetMouseCursor(cursor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetTouchX() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchY() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchPosition(index: ::std::os::raw::c_int) -> Vector2;
}
extern "C" {
    pub fn GetTouchPointId(index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetTouchPointCount() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetGesturesEnabled(flags: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn IsGestureDetected(gesture: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn GetGestureDetected() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGestureHoldDuration() -> f32;
}
extern "C" {
    pub fn GetGestureDragVector() -> Vector2;
}
extern "C" {
    pub fn GetGestureDragAngle() -> f32;
}
extern "C" {
    pub fn GetGesturePinchVector() -> Vector2;
}
extern "C" {
    pub fn GetGesturePinchAngle() -> f32;
}
extern "C" {
    pub fn UpdateCamera(camera: *mut Camera, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: f32);
}
extern "C" {
    pub fn SetShapesTexture(texture: Texture2D, source: Rectangle);
}
extern "C" {
    pub fn DrawPixel(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawPixelV(position: Vector2, color: Color);
}
extern "C" {
    pub fn DrawLine(
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
}
extern "C" {
    pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawLineStrip(points: *mut Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawCircle(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleSector(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleSectorLines(
        center: Vector2,
        radius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleGradient(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color1: Color,
        color2: Color,
    );
}
extern "C" {
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawCircleLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawEllipse(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawEllipseLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radiusH: f32,
        radiusV: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRing(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRingLines(
        center: Vector2,
        innerRadius: f32,
        outerRadius: f32,
        startAngle: f32,
        endAngle: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangle(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
}
extern "C" {
    pub fn DrawRectangleRec(rec: Rectangle, color: Color);
}
extern "C" {
    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
}
extern "C" {
    pub fn DrawRectangleGradientV(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color1: Color,
        color2: Color,
    );
}
extern "C" {
    pub fn DrawRectangleGradientH(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color1: Color,
        color2: Color,
    );
}
extern "C" {
    pub fn DrawRectangleGradientEx(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    );
}
extern "C" {
    pub fn DrawRectangleLines(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: f32, color: Color);
}
extern "C" {
    pub fn DrawRectangleRounded(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawRectangleRoundedLines(
        rec: Rectangle,
        roundness: f32,
        segments: ::std::os::raw::c_int,
        lineThick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
}
extern "C" {
    pub fn DrawTriangleFan(points: *mut Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawTriangleStrip(points: *mut Vector2, pointCount: ::std::os::raw::c_int, color: Color);
}
extern "C" {
    pub fn DrawPoly(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPolyLines(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPolyLinesEx(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        lineThick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineLinear(
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineBasis(
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineCatmullRom(
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineBezierQuadratic(
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineBezierCubic(
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color);
}
extern "C" {
    pub fn DrawSplineSegmentBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentBezierQuadratic(
        p1: Vector2,
        c2: Vector2,
        p3: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSplineSegmentBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    );
}
extern "C" {
    pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointBezierQuad(p1: Vector2, c2: Vector2, p3: Vector2, t: f32) -> Vector2;
}
extern "C" {
    pub fn GetSplinePointBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointTriangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointPoly(
        point: Vector2,
        points: *mut Vector2,
        pointCount: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionLines(
        startPos1: Vector2,
        endPos1: Vector2,
        startPos2: Vector2,
        endPos2: Vector2,
        collisionPoint: *mut Vector2,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionPointLine(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
}
extern "C" {
    pub fn LoadImage(fileName: *const ::std::os::raw::c_char) -> Image;
}
extern "C" {
    pub fn LoadImageRaw(
        fileName: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        headerSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageSvg(
        fileNameOrString: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageAnim(
        fileName: *const ::std::os::raw::c_char,
        frames: *mut ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn LoadImageFromTexture(texture: Texture2D) -> Image;
}
extern "C" {
    pub fn LoadImageFromScreen() -> Image;
}
extern "C" {
    pub fn IsImageReady(image: Image) -> bool;
}
extern "C" {
    pub fn UnloadImage(image: Image);
}
extern "C" {
    pub fn ExportImage(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportImageToMemory(
        image: Image,
        fileType: *const ::std::os::raw::c_char,
        fileSize: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn ExportImageAsCode(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GenImageColor(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientLinear(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
        start: Color,
        end: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientRadial(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageGradientSquare(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageChecked(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        checksX: ::std::os::raw::c_int,
        checksY: ::std::os::raw::c_int,
        col1: Color,
        col2: Color,
    ) -> Image;
}
extern "C" {
    pub fn GenImageWhiteNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        factor: f32,
    ) -> Image;
}
extern "C" {
    pub fn GenImagePerlinNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        scale: f32,
    ) -> Image;
}
extern "C" {
    pub fn GenImageCellular(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        tileSize: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn GenImageText(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    ) -> Image;
}
extern "C" {
    pub fn ImageCopy(image: Image) -> Image;
}
extern "C" {
    pub fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
}
extern "C" {
    pub fn ImageText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;
}
extern "C" {
    pub fn ImageTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    ) -> Image;
}
extern "C" {
    pub fn ImageFormat(image: *mut Image, newFormat: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageToPOT(image: *mut Image, fill: Color);
}
extern "C" {
    pub fn ImageCrop(image: *mut Image, crop: Rectangle);
}
extern "C" {
    pub fn ImageAlphaCrop(image: *mut Image, threshold: f32);
}
extern "C" {
    pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32);
}
extern "C" {
    pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
}
extern "C" {
    pub fn ImageAlphaPremultiply(image: *mut Image);
}
extern "C" {
    pub fn ImageBlurGaussian(image: *mut Image, blurSize: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageResize(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResizeNN(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageResizeCanvas(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        fill: Color,
    );
}
extern "C" {
    pub fn ImageMipmaps(image: *mut Image);
}
extern "C" {
    pub fn ImageDither(
        image: *mut Image,
        rBpp: ::std::os::raw::c_int,
        gBpp: ::std::os::raw::c_int,
        bBpp: ::std::os::raw::c_int,
        aBpp: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ImageFlipVertical(image: *mut Image);
}
extern "C" {
    pub fn ImageFlipHorizontal(image: *mut Image);
}
extern "C" {
    pub fn ImageRotate(image: *mut Image, degrees: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageRotateCW(image: *mut Image);
}
extern "C" {
    pub fn ImageRotateCCW(image: *mut Image);
}
extern "C" {
    pub fn ImageColorTint(image: *mut Image, color: Color);
}
extern "C" {
    pub fn ImageColorInvert(image: *mut Image);
}
extern "C" {
    pub fn ImageColorGrayscale(image: *mut Image);
}
extern "C" {
    pub fn ImageColorContrast(image: *mut Image, contrast: f32);
}
extern "C" {
    pub fn ImageColorBrightness(image: *mut Image, brightness: ::std::os::raw::c_int);
}
extern "C" {
    pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
}
extern "C" {
    pub fn LoadImageColors(image: Image) -> *mut Color;
}
extern "C" {
    pub fn LoadImagePalette(
        image: Image,
        maxPaletteSize: ::std::os::raw::c_int,
        colorCount: *mut ::std::os::raw::c_int,
    ) -> *mut Color;
}
extern "C" {
    pub fn UnloadImageColors(colors: *mut Color);
}
extern "C" {
    pub fn UnloadImagePalette(colors: *mut Color);
}
extern "C" {
    pub fn GetImageAlphaBorder(image: Image, threshold: f32) -> Rectangle;
}
extern "C" {
    pub fn GetImageColor(image: Image, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int)
        -> Color;
}
extern "C" {
    pub fn ImageClearBackground(dst: *mut Image, color: Color);
}
extern "C" {
    pub fn ImageDrawPixel(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawLine(
        dst: *mut Image,
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawCircle(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleLines(
        dst: *mut Image,
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawCircleLinesV(
        dst: *mut Image,
        center: Vector2,
        radius: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawRectangle(
        dst: *mut Image,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
}
extern "C" {
    pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
}
extern "C" {
    pub fn ImageDrawRectangleLines(
        dst: *mut Image,
        rec: Rectangle,
        thick: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDraw(
        dst: *mut Image,
        src: Image,
        srcRec: Rectangle,
        dstRec: Rectangle,
        tint: Color,
    );
}
extern "C" {
    pub fn ImageDrawText(
        dst: *mut Image,
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn ImageDrawTextEx(
        dst: *mut Image,
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn LoadTexture(fileName: *const ::std::os::raw::c_char) -> Texture2D;
}
extern "C" {
    pub fn LoadTextureFromImage(image: Image) -> Texture2D;
}
extern "C" {
    pub fn LoadTextureCubemap(image: Image, layout: ::std::os::raw::c_int) -> TextureCubemap;
}
extern "C" {
    pub fn LoadRenderTexture(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> RenderTexture2D;
}
extern "C" {
    pub fn IsTextureReady(texture: Texture2D) -> bool;
}
extern "C" {
    pub fn UnloadTexture(texture: Texture2D);
}
extern "C" {
    pub fn IsRenderTextureReady(target: RenderTexture2D) -> bool;
}
extern "C" {
    pub fn UnloadRenderTexture(target: RenderTexture2D);
}
extern "C" {
    pub fn UpdateTexture(texture: Texture2D, pixels: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn UpdateTextureRec(
        texture: Texture2D,
        rec: Rectangle,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn GenTextureMipmaps(texture: *mut Texture2D);
}
extern "C" {
    pub fn SetTextureFilter(texture: Texture2D, filter: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetTextureWrap(texture: Texture2D, wrap: ::std::os::raw::c_int);
}
extern "C" {
    pub fn DrawTexture(
        texture: Texture2D,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
}
extern "C" {
    pub fn DrawTextureEx(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
}
extern "C" {
    pub fn DrawTexturePro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextureNPatch(
        texture: Texture2D,
        nPatchInfo: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn Fade(color: Color, alpha: f32) -> Color;
}
extern "C" {
    pub fn ColorToInt(color: Color) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ColorNormalize(color: Color) -> Vector4;
}
extern "C" {
    pub fn ColorFromNormalized(normalized: Vector4) -> Color;
}
extern "C" {
    pub fn ColorToHSV(color: Color) -> Vector3;
}
extern "C" {
    pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> Color;
}
extern "C" {
    pub fn ColorTint(color: Color, tint: Color) -> Color;
}
extern "C" {
    pub fn ColorBrightness(color: Color, factor: f32) -> Color;
}
extern "C" {
    pub fn ColorContrast(color: Color, contrast: f32) -> Color;
}
extern "C" {
    pub fn ColorAlpha(color: Color, alpha: f32) -> Color;
}
extern "C" {
    pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color;
}
extern "C" {
    pub fn GetColor(hexValue: ::std::os::raw::c_uint) -> Color;
}
extern "C" {
    pub fn GetPixelColor(
        srcPtr: *mut ::std::os::raw::c_void,
        format: ::std::os::raw::c_int,
    ) -> Color;
}
extern "C" {
    pub fn SetPixelColor(
        dstPtr: *mut ::std::os::raw::c_void,
        color: Color,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn GetPixelDataSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetFontDefault() -> Font;
}
extern "C" {
    pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;
}
extern "C" {
    pub fn LoadFontEx(
        fileName: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
    ) -> Font;
}
extern "C" {
    pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ::std::os::raw::c_int) -> Font;
}
extern "C" {
    pub fn LoadFontFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
    ) -> Font;
}
extern "C" {
    pub fn IsFontReady(font: Font) -> bool;
}
extern "C" {
    pub fn LoadFontData(
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        codepoints: *mut ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
    ) -> *mut GlyphInfo;
}
extern "C" {
    pub fn GenImageFontAtlas(
        glyphs: *const GlyphInfo,
        glyphRecs: *mut *mut Rectangle,
        glyphCount: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        padding: ::std::os::raw::c_int,
        packMethod: ::std::os::raw::c_int,
    ) -> Image;
}
extern "C" {
    pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UnloadFont(font: Font);
}
extern "C" {
    pub fn ExportFontAsCode(font: Font, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);
}
extern "C" {
    pub fn DrawText(
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextPro(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        origin: Vector2,
        rotation: f32,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextCodepoint(
        font: Font,
        codepoint: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawTextCodepoints(
        font: Font,
        codepoints: *const ::std::os::raw::c_int,
        codepointCount: ::std::os::raw::c_int,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn SetTextLineSpacing(spacing: ::std::os::raw::c_int);
}
extern "C" {
    pub fn MeasureText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MeasureTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
    ) -> Vector2;
}
extern "C" {
    pub fn GetGlyphIndex(font: Font, codepoint: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetGlyphInfo(font: Font, codepoint: ::std::os::raw::c_int) -> GlyphInfo;
}
extern "C" {
    pub fn GetGlyphAtlasRec(font: Font, codepoint: ::std::os::raw::c_int) -> Rectangle;
}
extern "C" {
    pub fn LoadUTF8(
        codepoints: *const ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn UnloadUTF8(text: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn LoadCodepoints(
        text: *const ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn UnloadCodepoints(codepoints: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn GetCodepointCount(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepoint(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepointNext(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetCodepointPrevious(
        text: *const ::std::os::raw::c_char,
        codepointSize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CodepointToUTF8(
        codepoint: ::std::os::raw::c_int,
        utf8Size: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextCopy(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextIsEqual(
        text1: *const ::std::os::raw::c_char,
        text2: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn TextLength(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn TextFormat(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextSubtext(
        text: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextReplace(
        text: *mut ::std::os::raw::c_char,
        replace: *const ::std::os::raw::c_char,
        by: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextInsert(
        text: *const ::std::os::raw::c_char,
        insert: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextJoin(
        textList: *mut *const ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        delimiter: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextSplit(
        text: *const ::std::os::raw::c_char,
        delimiter: ::std::os::raw::c_char,
        count: *mut ::std::os::raw::c_int,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextAppend(
        text: *mut ::std::os::raw::c_char,
        append: *const ::std::os::raw::c_char,
        position: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn TextFindIndex(
        text: *const ::std::os::raw::c_char,
        find: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TextToUpper(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToLower(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToPascal(text: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TextToInteger(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
}
extern "C" {
    pub fn DrawPoint3D(position: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCircle3D(
        center: Vector3,
        radius: f32,
        rotationAxis: Vector3,
        rotationAngle: f32,
        color: Color,
    );
}
extern "C" {
    pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
}
extern "C" {
    pub fn DrawTriangleStrip3D(
        points: *mut Vector3,
        pointCount: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
extern "C" {
    pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
}
extern "C" {
    pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);
}
extern "C" {
    pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
}
extern "C" {
    pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color);
}
extern "C" {
    pub fn DrawSphereEx(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawSphereWires(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinder(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: f32,
        endRadius: f32,
        sides: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderWires(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCylinderWiresEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: f32,
        endRadius: f32,
        sides: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCapsule(
        startPos: Vector3,
        endPos: Vector3,
        radius: f32,
        slices: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawCapsuleWires(
        startPos: Vector3,
        endPos: Vector3,
        radius: f32,
        slices: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        color: Color,
    );
}
extern "C" {
    pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
}
extern "C" {
    pub fn DrawRay(ray: Ray, color: Color);
}
extern "C" {
    pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: f32);
}
extern "C" {
    pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;
}
extern "C" {
    pub fn LoadModelFromMesh(mesh: Mesh) -> Model;
}
extern "C" {
    pub fn IsModelReady(model: Model) -> bool;
}
extern "C" {
    pub fn UnloadModel(model: Model);
}
extern "C" {
    pub fn GetModelBoundingBox(model: Model) -> BoundingBox;
}
extern "C" {
    pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color);
}
extern "C" {
    pub fn DrawModelWiresEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBoundingBox(box_: BoundingBox, color: Color);
}
extern "C" {
    pub fn DrawBillboard(
        camera: Camera,
        texture: Texture2D,
        position: Vector3,
        size: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBillboardRec(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    );
}
extern "C" {
    pub fn DrawBillboardPro(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
}
extern "C" {
    pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
}
extern "C" {
    pub fn UpdateMeshBuffer(
        mesh: Mesh,
        index: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        dataSize: ::std::os::raw::c_int,
        offset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn UnloadMesh(mesh: Mesh);
}
extern "C" {
    pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
}
extern "C" {
    pub fn DrawMeshInstanced(
        mesh: Mesh,
        material: Material,
        transforms: *const Matrix,
        instances: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ExportMesh(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;
}
extern "C" {
    pub fn GenMeshTangents(mesh: *mut Mesh);
}
extern "C" {
    pub fn GenMeshPoly(sides: ::std::os::raw::c_int, radius: f32) -> Mesh;
}
extern "C" {
    pub fn GenMeshPlane(
        width: f32,
        length: f32,
        resX: ::std::os::raw::c_int,
        resZ: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh;
}
extern "C" {
    pub fn GenMeshSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshHemiSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshCylinder(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
extern "C" {
    pub fn GenMeshCone(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;
}
extern "C" {
    pub fn GenMeshTorus(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshKnot(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;
}
extern "C" {
    pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;
}
extern "C" {
    pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;
}
extern "C" {
    pub fn LoadMaterials(
        fileName: *const ::std::os::raw::c_char,
        materialCount: *mut ::std::os::raw::c_int,
    ) -> *mut Material;
}
extern "C" {
    pub fn LoadMaterialDefault() -> Material;
}
extern "C" {
    pub fn IsMaterialReady(material: Material) -> bool;
}
extern "C" {
    pub fn UnloadMaterial(material: Material);
}
extern "C" {
    pub fn SetMaterialTexture(
        material: *mut Material,
        mapType: ::std::os::raw::c_int,
        texture: Texture2D,
    );
}
extern "C" {
    pub fn SetModelMeshMaterial(
        model: *mut Model,
        meshId: ::std::os::raw::c_int,
        materialId: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn LoadModelAnimations(
        fileName: *const ::std::os::raw::c_char,
        animCount: *mut ::std::os::raw::c_int,
    ) -> *mut ModelAnimation;
}
extern "C" {
    pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ::std::os::raw::c_int);
}
extern "C" {
    pub fn UnloadModelAnimation(anim: ModelAnimation);
}
extern "C" {
    pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: ::std::os::raw::c_int);
}
extern "C" {
    pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;
}
extern "C" {
    pub fn CheckCollisionSpheres(
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool;
}
extern "C" {
    pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;
}
extern "C" {
    pub fn CheckCollisionBoxSphere(box_: BoundingBox, center: Vector3, radius: f32) -> bool;
}
extern "C" {
    pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: f32) -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionBox(ray: Ray, box_: BoundingBox) -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)
        -> RayCollision;
}
extern "C" {
    pub fn GetRayCollisionQuad(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision;
}
pub type AudioCallback = ::std::option::Option<
    unsafe extern "C" fn(bufferData: *mut ::std::os::raw::c_void, frames: ::std::os::raw::c_uint),
>;
extern "C" {
    pub fn InitAudioDevice();
}
extern "C" {
    pub fn CloseAudioDevice();
}
extern "C" {
    pub fn IsAudioDeviceReady() -> bool;
}
extern "C" {
    pub fn SetMasterVolume(volume: f32);
}
extern "C" {
    pub fn GetMasterVolume() -> f32;
}
extern "C" {
    pub fn LoadWave(fileName: *const ::std::os::raw::c_char) -> Wave;
}
extern "C" {
    pub fn LoadWaveFromMemory(
        fileType: *const ::std::os::raw::c_char,
        fileData: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Wave;
}
extern "C" {
    pub fn IsWaveReady(wave: Wave) -> bool;
}
extern "C" {
    pub fn LoadSound(fileName: *const ::std::os::raw::c_char) -> Sound;
}
extern "C" {
    pub fn LoadSoundFromWave(wave: Wave) -> Sound;
}
extern "C" {
    pub fn LoadSoundAlias(source: Sound) -> Sound;
}
extern "C" {
    pub fn IsSoundReady(sound: Sound) -> bool;
}
extern "C" {
    pub fn UpdateSound(
        sound: Sound,
        data: *const ::std::os::raw::c_void,
        sampleCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn UnloadWave(wave: Wave);
}
extern "C" {
    pub fn UnloadSound(sound: Sound);
}
extern "C" {
    pub fn UnloadSoundAlias(alias: Sound);
}
extern "C" {
    pub fn ExportWave(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn ExportWaveAsCode(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn PlaySound(sound: Sound);
}
extern "C" {
    pub fn StopSound(sound: Sound);
}
extern "C" {
    pub fn PauseSound(sound: Sound);
}
extern "C" {
    pub fn ResumeSound(sound: Sound);
}
extern "C" {
    pub fn IsSoundPlaying(sound: Sound) -> bool;
}
extern "C" {
    pub fn SetSoundVolume(sound: Sound, volume: f32);
}
extern "C" {
    pub fn SetSoundPitch(sound: Sound, pitch: f32);
}
extern "C" {
    pub fn SetSoundPan(sound: Sound, pan: f32);
}
extern "C" {
    pub fn WaveCopy(wave: Wave) -> Wave;
}
extern "C" {
    pub fn WaveCrop(
        wave: *mut Wave,
        initSample: ::std::os::raw::c_int,
        finalSample: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn WaveFormat(
        wave: *mut Wave,
        sampleRate: ::std::os::raw::c_int,
        sampleSize: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn LoadWaveSamples(wave: Wave) -> *mut f32;
}
extern "C" {
    pub fn UnloadWaveSamples(samples: *mut f32);
}
extern "C" {
    pub fn LoadMusicStream(fileName: *const ::std::os::raw::c_char) -> Music;
}
extern "C" {
    pub fn LoadMusicStreamFromMemory(
        fileType: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_uchar,
        dataSize: ::std::os::raw::c_int,
    ) -> Music;
}
extern "C" {
    pub fn IsMusicReady(music: Music) -> bool;
}
extern "C" {
    pub fn UnloadMusicStream(music: Music);
}
extern "C" {
    pub fn PlayMusicStream(music: Music);
}
extern "C" {
    pub fn IsMusicStreamPlaying(music: Music) -> bool;
}
extern "C" {
    pub fn UpdateMusicStream(music: Music);
}
extern "C" {
    pub fn StopMusicStream(music: Music);
}
extern "C" {
    pub fn PauseMusicStream(music: Music);
}
extern "C" {
    pub fn ResumeMusicStream(music: Music);
}
extern "C" {
    pub fn SeekMusicStream(music: Music, position: f32);
}
extern "C" {
    pub fn SetMusicVolume(music: Music, volume: f32);
}
extern "C" {
    pub fn SetMusicPitch(music: Music, pitch: f32);
}
extern "C" {
    pub fn SetMusicPan(music: Music, pan: f32);
}
extern "C" {
    pub fn GetMusicTimeLength(music: Music) -> f32;
}
extern "C" {
    pub fn GetMusicTimePlayed(music: Music) -> f32;
}
extern "C" {
    pub fn LoadAudioStream(
        sampleRate: ::std::os::raw::c_uint,
        sampleSize: ::std::os::raw::c_uint,
        channels: ::std::os::raw::c_uint,
    ) -> AudioStream;
}
extern "C" {
    pub fn IsAudioStreamReady(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn UnloadAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn UpdateAudioStream(
        stream: AudioStream,
        data: *const ::std::os::raw::c_void,
        frameCount: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn IsAudioStreamProcessed(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn PlayAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn PauseAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn ResumeAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool;
}
extern "C" {
    pub fn StopAudioStream(stream: AudioStream);
}
extern "C" {
    pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32);
}
extern "C" {
    pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32);
}
extern "C" {
    pub fn SetAudioStreamPan(stream: AudioStream, pan: f32);
}
extern "C" {
    pub fn SetAudioStreamBufferSizeDefault(size: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback);
}
extern "C" {
    pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
}
extern "C" {
    pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
}
extern "C" {
    pub fn AttachAudioMixedProcessor(processor: AudioCallback);
}
extern "C" {
    pub fn DetachAudioMixedProcessor(processor: AudioCallback);
}

