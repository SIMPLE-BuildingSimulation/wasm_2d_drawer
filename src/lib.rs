

/// The kind of Floating point number used in the
/// library... the `"float"` feature means it becomes `f32`
/// and `f64` is used otherwise.
#[cfg(feature = "float")]
type Float = f32;

#[cfg(not(feature = "float"))]
type Float = f64;

#[cfg(feature = "float")]
const PI : Float = std::f32::consts::PI;

#[cfg(not(feature = "float"))]
const PI : Float = std::f64::consts::PI;



pub mod drawer2d;
pub mod point2d;
pub mod pointcloud2d;
pub mod tool_trait;
pub mod toolbox;

mod utils;
