#[cfg(feature = "web")]
use crate::web_fns::*;
#[cfg(feature = "native")]
use crate::native_fns::*;
use crate::structs::*;
use crate::small_c_string::run_with_cstr;

#[inline]
pub fn init_window(w: i32, h: i32, title: &str) {
    run_with_cstr(title.as_bytes(), |ctitle| unsafe {
        InitWindow(w, h, ctitle.as_ptr());
    })
}

#[inline]
pub fn measure_text(text: &str, font_size: usize) {
    run_with_cstr(text.as_bytes(), |text| unsafe {
        MeasureText(text.as_ptr(), font_size as i32);
    })
}

#[inline]
pub fn draw_text(text: &str, x: i32, y: i32, font_size: usize, color: Color) {
    run_with_cstr(text.as_bytes(), |text| unsafe {
        DrawText(text.as_ptr(), x, y, font_size as i32, color);
    })
}
