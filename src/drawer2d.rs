use crate::Float;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::point2d::{CanvasPoint2D, Point2D};
use crate::utils;

#[wasm_bindgen]
pub struct Drawer2D {
    /// THe actual context to draw in
    context: web_sys::CanvasRenderingContext2d,

    /// The Canvas element in HTML
    canvas: web_sys::HtmlCanvasElement,

    /// Center of the viewport in meters
    center: Point2D,

    /// Real world width in meters
    width: Float,
}

impl Drawer2D {
    /// Returns the (height, width) of the viewport in meters
    pub fn viewport_size(&self) -> (Float, Float) {
        let canvas_width = self.canvas.width() as Float;
        let canvas_height = self.canvas.height() as Float;
        let r = canvas_width / canvas_height;

        // height = self.width/r
        (self.width / r, self.width)
    }

    /// Clears the canvas
    pub fn clear(&self) {
        let height = self.canvas.height() as Float;
        let width = self.canvas.width() as Float;
        self.context.clear_rect(0.0, 0.0, width.into(), height.into());
    }

    /// Borrows the canvas
    pub fn canvas(&self) -> &web_sys::HtmlCanvasElement {
        &self.canvas
    }

    /// Borrows the context
    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }

    /// Calculates the position of a Point2D in meters within the
    /// canvas (in pixels). Returns a tuple with the CanvasPoint2D and
    /// a boolean stating whether the point is visible or not
    ///
    /// The result can be OUT of the canvas (e.g., negative values or
    /// out of the (width,height) tuple)
    pub fn as_canvas_point(&self, p: &Point2D) -> (CanvasPoint2D, bool) {
        // Viewport size
        let (vp_height, vp_width) = self.viewport_size();

        // Canvas/World Aspect ratio
        let r = self.canvas.width() as Float / self.width;

        // find origin of the viewport reference system
        let ocx = self.center.x - vp_width / 2.;
        let ocy = -(self.center.y + vp_height / 2.);

        // return
        let pt = CanvasPoint2D {
            x: r * (p.x - ocx),
            y: -r * (p.y + ocy),
        };

        let is_visible = pt.x >= 0.0
            && pt.x <= self.canvas.width() as Float
            && pt.y >= 0.0 as Float
            && pt.y <= self.canvas.height() as Float;

        (pt, is_visible)
    }
}

#[wasm_bindgen]
impl Drawer2D {
    /// Creates a new drawer
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let canvas = document.get_element_by_id("wasm-canvas").unwrap();

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self {
            context,
            canvas,
            center: Point2D { x: 0.0, y: 0.0 },
            width: 10.,
        }
    }

    /// Transforms a canvas point into a world point
    pub fn as_world_point(&self, p: &CanvasPoint2D) -> Point2D {
        // Viewport size
        let (vp_height, vp_width) = self.viewport_size();

        // Canvas/World Aspect ratio
        let r = self.canvas.width() as Float / self.width;

        // find origin of the viewport reference system
        let ocx = self.center.x - vp_width / 2.;
        let ocy = -(self.center.y + vp_height / 2.);

        // return
        Point2D {
            x: ocx + p.x / r,
            y: -ocy - p.y / r,
        }
    }

    /// Sets up the size of the canvas and
    /// draws the building
    pub fn setup_canvas(&mut self, height: u32, width: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    /// Retreives the width of the viewport in World dimensions
    pub fn width(&self) -> Float {
        let (_height, width) = self.viewport_size();
        width
    }

    /// Retreives the height of the viewport in World dimensions
    pub fn height(&self) -> Float {
        let (height, _width) = self.viewport_size();
        height
    }

    /// Translates the center
    pub fn translate_viewport(&mut self, x: Float, y: Float) {
        self.center.x += x;
        self.center.y += y;
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

}
