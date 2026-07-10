//! Shared render helpers for both backends (tty + winit).
//!
//! Right now this is only the mouse cursor. Both backends draw the same scene:
//! the window `Space` plus a set of "custom" elements layered on top. The cursor
//! is one such custom element — it's not a wayland surface, it's the xcursor
//! theme image uploaded as a texture and placed at the pointer.
//!
//! Both backends use `GlesRenderer`, so this needs no generics — no
//! `WindowElement`, no `CustomRenderElements` enum, none of anvil's shell types.

use std::time::Duration;

use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::{
                Kind,
                memory::{MemoryRenderBuffer, MemoryRenderBufferRenderElement},
            },
            gles::GlesRenderer,
        },
    },
    utils::{Logical, Point, Transform},
};

use crate::cursor::Cursor;

/// Build the default-theme cursor element at `pointer_location`, ready to drop
/// into `render_output`'s custom-elements slice. Returns a `Vec` (empty on
/// failure) so the call site is just `&cursor_elements(...)`.
///
/// This always draws the theme arrow; honoring client-set cursors
/// (`CursorImageStatus::Surface`) / hiding is a later refinement.
pub fn cursor_elements(
    renderer: &mut GlesRenderer,
    cursor: &Cursor,
    pointer_location: Point<f64, Logical>,
    time: Duration,
) -> Vec<MemoryRenderBufferRenderElement<GlesRenderer>> {
    let image = cursor.get_image(1, time);

    let buffer = MemoryRenderBuffer::from_slice(
        &image.pixels_rgba,
        Fourcc::Argb8888,
        (image.width as i32, image.height as i32),
        1,
        Transform::Normal,
        None,
    );

    // Put the cursor's hotspot at the pointer, then convert to physical space
    // (scale 1 until we do HiDPI).
    let location = (pointer_location
        - Point::<f64, Logical>::from((image.xhot as f64, image.yhot as f64)))
    .to_physical(1.0);

    match MemoryRenderBufferRenderElement::from_buffer(
        renderer,
        location,
        &buffer,
        None,
        None,
        None,
        Kind::Cursor,
    ) {
        Ok(element) => vec![element],
        Err(err) => {
            tracing::warn!("failed to build cursor element: {err:?}");
            Vec::new()
        }
    }
}
