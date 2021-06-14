use crate::drawer2d::Drawer2D;

/// A trait that the Tools in the Toolbox need to comply with.
/// 
/// This Trait contains the general mouse interactions... can be extended in the future.
pub trait ToolTrait<T> {

    fn onmousemove(&mut self, drawable: &T, drawer: &mut Drawer2D,  x: u32, y: u32);
    fn onmousedown(&mut self, drawable: &T, drawer: &mut Drawer2D,  x: u32, y: u32);
    fn onmouseup(&mut self, drawable: &T, drawer: &mut Drawer2D,  x: u32, y: u32);
    fn onwheel(&mut self, drawable: &T, drawer: &mut Drawer2D,  dy: f64, x: u32, y:u32);
    
}