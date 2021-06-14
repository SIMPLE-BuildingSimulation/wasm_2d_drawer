use wasm_bindgen::prelude::*;

use crate::drawer2d::Drawer2D;
use crate::point2d::{Point2D};


/// A Point2D collection that allows organizing them
/// and connecting them.
#[wasm_bindgen]
pub struct PointCloud2D {
    
    /// All the points in the collection
    points : Vec<Point2D>,

    /// position of each point in the X direction
    positions_x: Vec<usize>,

    /// position of each point in the Y direction
    positions_y:Vec<usize>,

    /// The indexes of the points, sorted in the X axis
    sorted_x: Vec<usize>,

    /// The indexes of the points, sorted in the Y axis
    sorted_y: Vec<usize>,
    
}

impl PointCloud2D {

    /// Borrows the points
    pub fn points(&self)->&[Point2D]{
        &self.points
    }

      /// Finds the position that new point would have in the 
    /// sorted_x.
    /// 
    /// If the tested point is in the same position as an already existing
    /// point, it will be marked as being after
    fn find_point_position_x(&self, new_x:f64)->usize{        
        let found = self.sorted_x.binary_search_by(|i| {
            self.points[*i].x.partial_cmp(&new_x).expect("could not compare!")
        });
        match found {
            Ok(i)=>i+1, // It was there... return the index of the following element
            Err(i)=>i // It was not there... return the index
        }        
    }

    /// Finds the position that new point would have in the 
    /// sorted_y.
    /// 
    /// If the tested point is in the same position as an already existing
    /// point, it will be marked as being after
    fn find_point_position_y(&self, new_y:f64)->usize{        
        let found = self.sorted_y.binary_search_by(|i| {
            self.points[*i].y.partial_cmp(&new_y).expect("could not compare!")
        });
        match found {
            Ok(i)=>i+1, // It was there... return the index of the following element
            Err(i)=>i // It was not there... return the index
        }        
    }


     /// Checks whether the structure is coherent
     #[cfg(debug_assertions)]
     fn check_consistency(&self){        
         
         // Lengths of the structure
         debug_assert_eq!(self.points.len(), self.positions_x.len());
         debug_assert_eq!(self.positions_x.len(), self.positions_y.len());
         debug_assert_eq!(self.positions_y.len(), self.sorted_x.len());
         debug_assert_eq!(self.sorted_x.len(), self.sorted_y.len());
 
         // Ensure that all positions are there
         for i in 0..self.points.len(){
             assert!(self.positions_x.contains(&i));
             assert!(self.positions_y.contains(&i));
             assert!(self.sorted_x.contains(&i));
             assert!(self.sorted_y.contains(&i));
         }
 
         // indexes and positions in X direction
         for i in 0..self.sorted_x.len(){
             
             let index = self.sorted_x[i];
             let current = self.points[index];
             debug_assert_eq!(self.positions_x[index], i);
             
 
             // If not first, check previous
             if i >= 1 {
                 let prev_index = self.sorted_x[i-1];
                 let prev = self.points[prev_index];                
                 if prev.x > current.x {
                     panic!("not true: prev.x [index:{}] <= current.x [index:{}] | next.x = {}, current.x = {}",prev_index, index, prev.x, current.x);
                 }
             }
 
             // if not last, check next
             if i + 1 < self.sorted_x.len() {
                 let next_index = self.sorted_x[i+1];
                 let next = self.points[next_index];
                 if next.x < current.x {
                     panic!("not true: next.x [index:{}] >= current.x [index:{}] | next.x = {}, current.x = {}",next_index, index, next.x, current.x);
                 }                
             }
         }
 
         // indexes and positions in Y direction
         for i in 0..self.sorted_y.len(){
             
             let index = self.sorted_y[i];
             let current = self.points[index];
             debug_assert_eq!(self.positions_y[index], i);
             
 
             // If not first, check previous
             if i >= 1 {
                 let prev_index = self.sorted_y[i-1];
                 let prev = self.points[prev_index];
                 if prev.y > current.y {
                     panic!("not true: prev.y [index:{}] <= current.y [index:{}] | next.y = {}, current.y = {}",prev_index, index, prev.y, current.y);
                 }
             }
 
             // if not last, check next
             if i + 1 < self.sorted_y.len() {
                 let next_index = self.sorted_y[i+1];
                 let next = self.points[next_index];
                 if next.y < current.y {
                     panic!("not true: next.y [index:{}] >= current.y [index:{}] | next.y = {}, current.y = {}",next_index, index, next.y, current.y);
                 }
             }
         }
 
     }
}

#[wasm_bindgen]
impl PointCloud2D{
    
    /// Creates a new empty PointCloud2D
    #[wasm_bindgen(constructor)]
    pub fn new()->Self{
        Self{
            points: Vec::new(),
            positions_x:Vec::new(),
            positions_y:Vec::new(),
            sorted_x: Vec::new(),
            sorted_y: Vec::new(),
        }
    }

    /// Cleans the canvas and then redraws
    pub fn redraw(&self,drawer:&Drawer2D){
        drawer.clear();
        self.draw(drawer)
    }

    /// Draws the Cloud
    pub fn draw(&self, drawer:&Drawer2D){
        
        const RADIUS : f64 = 5.;
        let context = drawer.context();

        for p in &self.points{            
            let (canvas_p, is_visible) = drawer.as_canvas_point(p);
            if is_visible {

                context.begin_path();
                context.arc(canvas_p.x, canvas_p.y, RADIUS, 0., 2.0*std::f64::consts::PI).unwrap();

                let fill_style = wasm_bindgen::JsValue::from_str("green");
                context.set_fill_style(&fill_style);
                context.fill();

                context.set_line_width(3.);
                let stroke_style = wasm_bindgen::JsValue::from_str("#003300");
                context.set_stroke_style(&stroke_style);
                context.stroke();
            }
        }

    }
   

    /// Checks if the PointCloud2D is empty
    pub fn is_empty(&self)->bool{        
        #[cfg(debug_assertions)]
        self.check_consistency();
        
        self.points.is_empty()
    }

  
    /// Adds a point to the cloud, identifying its position
    /// and updating the whole structure
    pub fn push(&mut self, p:Point2D){
        
        // Get the index of the new point
        let new_index = self.points.len();
        
        // Push the point
        self.points.push(p);

        // Insert in X
        let index_x = self.find_point_position_x(p.x);
        for e in self.positions_x.iter_mut(){
            if *e >= index_x {
                *e += 1
            }
        }
        self.positions_x.push(index_x);
        self.sorted_x.insert(index_x, new_index);

        // Insert in Y
        let index_y = self.find_point_position_y(p.y);
        for e in self.positions_y.iter_mut(){
            if *e >= index_y {
                *e += 1
            }
        }
        self.positions_y.push(index_y);
        self.sorted_y.insert(index_y, new_index); 
        
        #[cfg(debug_assertions)]
        self.check_consistency();
    }

    
    /// Updates point in index i, replacing it by P
    pub fn update_point(&mut self, i:usize, p:Point2D){
        
        /* X AXIS */
        
        let old_x_position = self.positions_x[i];
        let mut new_x_position = self.find_point_position_x(p.x);
        // Since find_point_position_y can return an index out of bounds 
        // (e.g., the point isn't there and thus it would be position at the
        // end of the array), we need to do this.
        if new_x_position >= self.positions_x.len(){
            new_x_position -=1;
        }
        
        if old_x_position > new_x_position{
            // moving left
        
            // update positions_x
            for e in self.positions_x.iter_mut(){
                if *e == old_x_position{
                    *e = new_x_position
                }else if *e >= new_x_position && *e < old_x_position {
                    *e += 1
                }
            }            

            // update sorted_x
            for i in (new_x_position+1..old_x_position+1).rev(){
                self.sorted_x[i] = self.sorted_x[i-1];
            }
            self.sorted_x[new_x_position]=i;
            
        }else if old_x_position < new_x_position {
            // Moving right

            // update positions_x
            for e in self.positions_x.iter_mut(){
                if *e == old_x_position{
                    *e = new_x_position
                }else if *e <= new_x_position && *e > old_x_position {
                    *e -= 1
                }
            }
            
            // update sorted_x
            for i in old_x_position..new_x_position{
                self.sorted_x[i] = self.sorted_x[i+1];
            }
            self.sorted_x[new_x_position]=i;

        }// else, do nothing... does not move.
        
        /* Y AXIS */
        let old_y_position = self.positions_y[i];
        let mut new_y_position = self.find_point_position_y(p.y);

        // Since find_point_position_y can return an index out of bounds 
        // (e.g., the point isn't there and thus it would be position at the
        // end of the array), we need to do this.
        if new_y_position >= self.positions_y.len(){
            new_y_position -= 1;
        }
        
        if old_y_position > new_y_position{
            // moving down
        
            // update positions_y
            for e in self.positions_y.iter_mut(){
                if *e == old_y_position{
                    *e = new_y_position
                }else if *e >= new_y_position && *e < old_y_position {
                    *e += 1
                }
            }            

            // update sorted_y
            for i in (new_y_position+1..old_y_position+1).rev(){
                self.sorted_y[i] = self.sorted_y[i-1];
            }
            self.sorted_y[new_y_position]=i;
            
        }else if old_y_position < new_y_position {
            // Moving up

            // update positions_y
            for e in self.positions_y.iter_mut(){
                if *e == old_y_position{
                    *e = new_y_position
                }else if *e <= new_y_position && *e > old_y_position {
                    *e -= 1
                }
            }
            // update sorted_y
            for i in old_y_position..new_y_position{
                self.sorted_y[i] = self.sorted_y[i+1];
            }
            self.sorted_y[new_y_position]=i;            
        }// else, do nothing... does not move.
        
    
        
        // Update point
        self.points[i] = p;

        /* VERIFY */
        #[cfg(debug_assertions)]
        self.check_consistency();
    }

    /// Checks whether a point P is very close to 
    /// another point in the Cloud
    /// 
    /// The way this works is as follows:
    /// 1. Find the points that might be close enough (i.e., within the p +- MAX_DISTANCE square)
    /// 2. Check which direction contains less points (i.e., X or Y)
    /// 3. Iterate the candidate points, checking the distance. If smallest so far, mark for return
    pub fn test_world_point(&self, p: &Point2D)->Option<usize>{
        
        const MAX_DISTANCE : f64 = 0.25;
        const MAX_DISTANCE_SQ : f64 = MAX_DISTANCE * MAX_DISTANCE;
        
        // 1. Find the points that might be close enough (i.e., within the p +- MAX_DISTANCE square)
        // Points outside of this rectangle cannot be "close enough"
        let min_index_x = self.find_point_position_x(p.x-MAX_DISTANCE);
        let max_index_x = self.find_point_position_x(p.x+MAX_DISTANCE);
        let d_index_x = max_index_x - min_index_x;

        let min_index_y = self.find_point_position_y(p.y-MAX_DISTANCE);
        let max_index_y = self.find_point_position_y(p.y+MAX_DISTANCE);
        let d_index_y = max_index_y - min_index_y;
                
        
        // 2. Check which direction contains less points (i.e., X or Y)        
        let (candidate_point_positions, sorted) = if d_index_x <= d_index_y {
            // there are less points to test in the X axis... iterate them all
            (min_index_x..max_index_x, &self.sorted_x)
        }else{
            // there are less points to test in the Y axis
            (min_index_y..max_index_y, &self.sorted_y)
        };
        
        // initialize return
        let mut ret : Option<usize> = None;
        let mut min_squared_distance = MAX_DISTANCE_SQ;
        
        // 3. Iterate the candidate points, checking the distance. If smallest so far, mark for return
        for other_position in candidate_point_positions {
            // Get the point
            let other_index = sorted[other_position];
            let other_p = &self.points[other_index];
            // Check distance... would this be really more efficient if searched in squares as opposed to circles?
            // That is, instead of calculating the ACTUAL square distance, to calculate the 
            // vertical/horizontal distance?
            let sq_d = p.squared_distance_to(other_p);
            if sq_d < min_squared_distance {                    
                ret = Some(other_index);
                min_squared_distance = sq_d;                
            }
        }

        // return 
        ret
    }

    /// Highlights a point by showing it on a different colour    
    pub fn highlight_point(&self, drawer:&Drawer2D, i:usize){
        let (p, is_visible) = drawer.as_canvas_point(&self.points[i]);
        if !is_visible {
            return
        }

        const RADIUS : f64 = 8.;

        drawer.context().begin_path();
        drawer.context().arc(p.x, p.y, RADIUS, 0., 2.0*std::f64::consts::PI).unwrap();

        let fill_style = wasm_bindgen::JsValue::from_str("red");
        drawer.context().set_fill_style(&fill_style);
        drawer.context().fill();

        drawer.context().set_line_width(3.);
        let stroke_style = wasm_bindgen::JsValue::from_str("#330000");
        drawer.context().set_stroke_style(&stroke_style);
        drawer.context().stroke();
    }

}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_point_position_x() {

        /******************************** */
        /* Empty case... should return 0. */
        /******************************** */
        let cloud = PointCloud2D::new();
        cloud.check_consistency();
        assert_eq!(cloud.find_point_position_x(1.),0);


        /******************************** */
        /* 1 point case. */
        /******************************** */
        
        // X axis is | ----- O ------
        //                 (0,0)
        let cloud = PointCloud2D {
            points: vec![Point2D{x:0.0, y:0.0}],
            positions_x:vec![0],
            positions_y:vec![0],
            sorted_x:vec![0],
            sorted_y:vec![0],
        };
        cloud.check_consistency();

        // Testing case:
        // X axis is   - P ---------- O ------
        //            (-1,0)        (0,0)
        assert_eq!(cloud.find_point_position_x(-1.),0);

        // Testing case:
        // X axis is   ---- O ------ P
        //                (0,0)    (1,0)
        assert_eq!(cloud.find_point_position_x(1.), 1);

        // Testing case:
        // X axis is   ---- OP ------ 
        //                (0,0)
        //                (0,0)   
        assert_eq!(cloud.find_point_position_x(0.), 1);

        /******************************** */
        /*  2 point cases */
        /******************************** */
        // X axis is | ----- O --------- O --
        //                 (0,0)        (1,0)
        let cloud = PointCloud2D {
            points: vec![Point2D{x:0.0, y:0.0}, Point2D{x:1., y:0.}],
            positions_x:vec![0,1],
            positions_y:vec![0,1],
            sorted_x:vec![0, 1],
            sorted_y:vec![0, 1],
        };
        cloud.check_consistency();

        // Test case |   P --------- O --------- O --
        //             (-1,0)      (0,0)        (1,0)
        assert_eq!(cloud.find_point_position_x(-1.),0);

        // Test case |   -- OP --------- O --
        //                (0,0)        (1,0)
        //                (0,0)
        assert_eq!(cloud.find_point_position_x(0.), 1);

        // Test case |   ----- O ----- P --------- O --
        //                   (0,0)    (0.5,0)    (1,0)
        assert_eq!(cloud.find_point_position_x(0.5),1);

        // Test case |   -- O --------- OP--
        //                (0,0)        (1,0)
        //                             (1,0)
        assert_eq!(cloud.find_point_position_x(1.), 2);

        // Test case |   -- O --------- O ------- P
        //                (0,0)        (1,0)     (2,0)        
        assert_eq!(cloud.find_point_position_x(2.), 2);
    }


    #[test]
    fn test_find_point_position_y() {

        /******************************** */
        /* Empty case... should return 0. */
        /******************************** */
        let cloud = PointCloud2D::new();
        cloud.check_consistency();
        assert_eq!(cloud.find_point_position_y(1.),0);


        /******************************** */
        /* 1 point case. */
        /******************************** */
        
        // Y axis is | ----- O ------
        //                 (0,0)
        let cloud = PointCloud2D {
            points: vec![Point2D{x:0.0, y:0.0}],
            positions_x:vec![0],
            positions_y:vec![0],
            sorted_x:vec![0],
            sorted_y:vec![0],
        };
        cloud.check_consistency();

        // Testing case:
        // Y axis is   - P ---------- O ------
        //            (0,-1)        (0,0)
        assert_eq!(cloud.find_point_position_y(-1.),0);

        // Testing case:
        // Y axis is   ---- O ------ P
        //                (0,0)    (0,1)
        assert_eq!(cloud.find_point_position_y(1.), 1);

        // Testing case:
        // Y axis is   ---- OP ------ 
        //                (0,0)
        //                (0,0)   
        assert_eq!(cloud.find_point_position_y(0.), 1);

        /******************************** */
        /*  2 point cases */
        /******************************** */
        // X axis is | ----- O --------- O --
        //                 (0,0)        (0,1)
        let cloud = PointCloud2D {
            points: vec![Point2D{x:0.0, y:0.0}, Point2D{x:0., y:1.}],
            positions_x:vec![0,1],
            positions_y:vec![0,1],
            sorted_x:vec![0, 1],
            sorted_y:vec![0, 1],
        };
        cloud.check_consistency();

        // Test case |   P --------- O --------- O --
        //             (0,-1)      (0,0)        (0,1)
        assert_eq!(cloud.find_point_position_y(-1.),0);

        // Test case |   -- OP --------- O --
        //                (0,0)        (0,1)
        //                (0,0)
        assert_eq!(cloud.find_point_position_y(0.), 1);

        // Test case |   ----- O ----- P --------- O --
        //                   (0,0)  (0,0.5)      (0,1)
        assert_eq!(cloud.find_point_position_y(0.5),1);

        // Test case |   -- O --------- OP--
        //                (0,0)        (0,1)
        //                             (0,1)
        assert_eq!(cloud.find_point_position_y(1.), 2);

        // Test case |   -- O --------- O ------- P
        //                (0,0)       (0,1)     (0,2)        
        assert_eq!(cloud.find_point_position_y(2.), 2);
    }


    #[test]
    fn test_push() {

        /******************************** */
        /* Empty case... should return 0. */
        /******************************** */
        let mut cloud = PointCloud2D::new();
        cloud.check_consistency();
        let p = Point2D{x:0., y:0.};
        cloud.push(p);
        assert_eq!(cloud.sorted_x, vec![0]);
        assert_eq!(cloud.sorted_y, vec![0]);
        assert_eq!(cloud.points, vec![p]);
        cloud.check_consistency();


        /******************************** */
        /* 1 point case. */
        /******************************** */
        
        // X axis is | ----- O ------
        //                 (0,0)
        let mut cloud = PointCloud2D {
            points: vec![Point2D{x:0.0, y:0.0}],
            positions_x:vec![0],
            positions_y:vec![0],
            sorted_x:vec![0],
            sorted_y:vec![0],
        };
        cloud.check_consistency();
        
        // Testing case:
        // X axis is   - P ---------- O ------
        //            (-1,0)        (0,0)
        let p = Point2D{x:-1.0, y:0.0};
        cloud.push(p);
        assert_eq!(cloud.sorted_x, vec![1,0]);
        assert_eq!(cloud.sorted_y, vec![0,1]);
        assert_eq!(cloud.points, vec![Point2D{x:0.0, y:0.0},p]);
        cloud.check_consistency();
        
    }

    #[test]
    fn test_update_point(){
        let a = Point2D{x:0.0, y:0.0};
        let b = Point2D{x:1.0, y:0.0}; 
        let c = Point2D{x:2.0, y:0.0};

         /******************************** */
        /*  3 points cases */
        /******************************** */
        // X axis is | ----- O --------- O -------- O
        //                 A(0,0)        B(1,0)     C(2,0)
        let mut cloud = PointCloud2D {
            points: vec![a,b,c],
            positions_x:vec![0,1, 2],
            positions_y:vec![0,1,2],
            sorted_x:vec![0, 1,2],
            sorted_y:vec![0, 1,2],
        };
        cloud.check_consistency();

        // Move A to the left (nothing should happen)
        let new_a = Point2D{x:-1., y:0.};
        cloud.update_point(0, new_a);        
        assert_eq!(cloud.positions_x, vec![0,1,2]);
        assert_eq!(cloud.sorted_x, vec![0,1,2]);        
        // this changes because, being in equal Y to the other points, it ends up third.
        assert_eq!(cloud.sorted_y, vec![1,2,0]); 
        assert_eq!(cloud.positions_y, vec![2,0,1]);
        assert_eq!(cloud.points[0], new_a);
        
        // Move A to the very right... new order is [b,c,a]
        let new_a = Point2D{x:12., y:0.};
        cloud.update_point(0, new_a);        
        assert_eq!(cloud.positions_x, vec![2,0,1]);
        assert_eq!(cloud.sorted_x, vec![1,2,0]);        
        // this changes because, being in equal Y to the other points, it ends up third.
        assert_eq!(cloud.sorted_y, vec![1,2,0]); 
        assert_eq!(cloud.positions_y, vec![2,0,1]);
        assert_eq!(cloud.points[0], new_a);
        
    }

    #[test]
    fn test_test_point(){
        
        let n_points = 20;

        /* ************** */
        /* ALL HORIZONTAL */
        /* ************** */
        let mut cloud = PointCloud2D::new();
        for i in 0..n_points {
            cloud.push(Point2D::new(i as f64,0.0));
        }
        // These are out of the clould altogether
        let p = Point2D::new(-10.0,0.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(100.0,0.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(0.0,-1.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(0.0,1.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(30.0,2.0);
        assert_eq!(cloud.test_world_point(&p), None);

        // These are in
        for i in 0..n_points{
            let p = Point2D::new(i as f64,0.0);
            assert_eq!(cloud.test_world_point(&p), Some(i));
        }

        /* ************** */
        /* ALL VERTICAL */
        /* ************** */
        let mut cloud = PointCloud2D::new();
        for i in 0..n_points {
            cloud.push(Point2D::new(0.0,i as f64));
        }
        // These are out of the clould altogether
        let p = Point2D::new(0.0,-10.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(0.0,110.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(1.0,0.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(-1.0,0.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(30.0,2.0);
        assert_eq!(cloud.test_world_point(&p), None);

        // These are in
        for i in 0..n_points{
            let p = Point2D::new(0.0, i as f64);
            assert_eq!(cloud.test_world_point(&p), Some(i));
        }


        /* ************** */
        /* 45 DEGREES */
        /* ************** */

        
        let mut cloud = PointCloud2D::new();
        for i in 0..n_points {
            cloud.push(Point2D::new(i as f64,i as f64));
        }
        
        // These are out of the clould altogether
        let p = Point2D::new(0.0,-10.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(0.0,110.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(1.0,0.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(-1.0,0.0);
        assert_eq!(cloud.test_world_point(&p), None);

        let p = Point2D::new(30.0,2.0);
        assert_eq!(cloud.test_world_point(&p), None);
        
        // These are in
        for i in 0..n_points{
            let p = Point2D::new(i as f64,i as f64);
            assert_eq!(cloud.test_world_point(&p), Some(i));
        }

    }

}
