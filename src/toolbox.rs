use crate::drawer2d::Drawer2D;
use crate::tool_trait::ToolTrait;


pub struct ToolBox<T>{
    tools: Vec<Box<dyn ToolTrait<T>>>,
    active_tool: Option<usize>,
}


impl <T>ToolBox<T>{

    /// Creates a new empty ToolBox with no Tool selected
    pub fn new()->Self{
        
        let tools : Vec<Box<dyn ToolTrait<T>>> = Vec::new();

        Self{
            tools,
            active_tool: None
        }
    }

    /// Gets the Toolbox's active tool. If None is selected,
    /// it returns the first one. Panics if the Toolbox has 
    /// no tools
    fn get_active_tool(&self)->usize{
        match self.active_tool {
            Some(i)=>i,
            None => {
                if self.tools.is_empty() {
                    panic!("ToolBox has no tools!");
                }else{
                    0
                }
            }
        }
    }


    /// Calls the onmousemove event on the selected Tool. 
    /// 
    /// If None is selected, calls it on the first one. Panics if Toolbox is empty.
    pub fn onmousemove(&mut self, drawable : &T, drawer: &mut Drawer2D, x: u32, y: u32){                
        let i = self.get_active_tool();
        self.tools[i].onmousemove(drawable, drawer, x, y);
    }
    
    /// Calls the onmouseup event on the selected Tool. 
    /// 
    /// If None is selected, calls it on the first one. Panics if Toolbox is empty.
    pub fn onmouseup(&mut self, drawable : &T, drawer: &mut Drawer2D, x: u32, y: u32){
        let i = self.get_active_tool();
        self.tools[i].onmouseup(drawable, drawer, x, y);
    }

    /// Calls the onmousedown event on the selected Tool. 
    /// 
    /// If None is selected, calls it on the first one. Panics if Toolbox is empty.
    pub fn onmousedown(&mut self, drawable : &T, drawer: &mut Drawer2D, x: u32, y: u32){
        let i = self.get_active_tool();
        self.tools[i].onmousedown(drawable, drawer, x, y);
    }

    /// Calls the onwheel event on the selected Tool. 
    /// 
    /// If None is selected, calls it on the first one. Panics if Toolbox is empty.
    pub fn onwheel(&mut self, drawable : &T, drawer: &mut Drawer2D, dy: f64, x: u32, y: u32){
        let i = self.get_active_tool();
        self.tools[i].onwheel(drawable, drawer, dy, x, y);
    }
}
