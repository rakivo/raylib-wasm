use crate::{Rectangle, KeyboardKey, Vector2, Color};

use std::ptr::addr_of;

pub mod ffi {
    use super::*;

    extern {
        pub fn GetMousePositionX() -> f32;
        pub fn GetMousePositionY() -> f32;
        pub fn DrawCircle(_: i32, _: i32, _: f32, _: *const Color);
        pub fn DrawLine(_: i32, _: i32, _: i32, _: i32, _: *const Color);
        pub fn DrawRectangleV(_: *const Vector2, _: *const Vector2, _: *const Color);
        pub fn DrawRectangleRec(_: *const Rectangle, _: *const Color);
        pub fn DrawRectangle(_: i32, _: i32, _: i32, _: i32, _: *const Color);
        pub fn SetTargetFPS(_: i32);
        pub fn InitWindow(_: i32, _: i32, _: *const i8);
        pub fn BeginDrawing();
        pub fn ClearBackground(_: *const Color);
        pub fn GetFrameTime() -> f32;
        pub fn DrawFPS(_: i32, _: i32);
        pub fn DrawText(_: *const i8, _: i32, _: i32, _: i32, _: *const Color);
        pub fn EndDrawing();
        pub fn SetExitKey(_: KeyboardKey);
        pub fn CloseWindow();
        pub fn IsKeyDown(_: KeyboardKey) -> bool;
        pub fn GetFPS() -> i32;
        pub fn GetMousePosition() -> Vector2;
        pub fn MeasureText(_: *const i8, _: i32) -> i32;
    }
}

// Functions that do not require passing a structure
pub use ffi::{
    DrawFPS,
    MeasureText,
    BeginDrawing,
    EndDrawing,
    SetTargetFPS,
    IsKeyDown,
    InitWindow,
    CloseWindow,
    GetFrameTime,
    GetMousePositionX,
    GetMousePositionY
};

// This functions are mandatory because without them the whole thing won't work.
// You can't just pass a structure into a function and get it fields in JS via memory buffer,
// To this thing to work we've got create this layer of abstraction.

#[inline(always)]
pub unsafe fn DrawLine(sx: i32, sy: i32, ex: i32, ey: i32, color: Color) {
    ffi::DrawLine(sx, sy, ex, ey, addr_of!(color))
}

#[inline(always)]
pub unsafe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color) {
    ffi::DrawRectangleV(addr_of!(position), addr_of!(size), addr_of!(color));
}

#[inline(always)]
pub unsafe fn WindowShouldClose() -> bool {
    false
}

#[inline(always)]
pub unsafe fn ClearBackground(color: Color) {
    ffi::ClearBackground(addr_of!(color));
}

#[inline(always)]
pub unsafe fn DrawText(text: *const i8, x: i32, y: i32, size: i32, color: Color) {
    ffi::DrawText(text, x, y, size, addr_of!(color));
}

#[inline(always)]
pub unsafe fn DrawRectangle(x: i32, y: i32, w: i32, h: i32, color: Color) {
    ffi::DrawRectangle(x, y, w, h, addr_of!(color));
}

#[inline(always)]
pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color) {
    ffi::DrawRectangleRec(addr_of!(rec), addr_of!(color));
}

#[inline(always)]
pub unsafe fn DrawCircle(cx: i32, cy: i32, radius: f32, color: Color) {
    ffi::DrawCircle(cx, cy, radius, addr_of!(color));
}

#[inline(always)]
pub unsafe fn GetMousePosition() -> Vector2 {
    Vector2 {
        x: ffi::GetMousePositionX(),
        y: ffi::GetMousePositionY()
    }
}
