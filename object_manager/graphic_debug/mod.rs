extern crate nalgebra;
use nalgebra::*;
use super::Constraint;
use super::Object;
type V3 = Vector3<f64>;

pub struct GraphicDebug {
    pub lines: Vec<(Vector3<f64>,Vector3<f64>)>,
    pub line_colors: Vec<Vector3<f64>>,
    pub dots: Vec<Vector3<f64>>,
}

fn planeinter(p: Vector3<f64> , zplane: f64)->Option<Vector3<f64>>{
    let rd = p.normalize();
    let r = 1./rd.dot(&Vector3::<f64>::new(0.,0.,1.)); 
    if r > 0. {
        return Some(rd * zplane * r);
    }
    None
}
impl GraphicDebug {
    pub fn clear(&mut self){
        self.lines.clear();
        self.line_colors.clear();
        self.dots.clear();
    }
    pub fn getlines(&self, dim: (f64,f64), cam: (Vector3<f64>, UnitQuaternion<f64>))-> [f64;1024]{
        let mut out = [ 0.;1024];
        let mut j =0;
        for L in self.lines.iter(){
            let lm0 = cam.1.inverse() * (L.0 - cam.0); 
            let lm1 = cam.1.inverse() * (L.1 - cam.0); 
            let l2 = planeinter(lm0, 0.7); 
            let l3 = planeinter(lm1, 0.7);
            
            if !(l2.is_none()||l3.is_none()){
                out[j] = l2.unwrap().x;
                out[j+1] = l2.unwrap().y * (dim.0/dim.1);
                out[j+2] = l3.unwrap().x;
                out[j+3] = l3.unwrap().y* (dim.0/dim.1);
                j+=4;
            }
        }
        out
    }
    pub fn debug_constraint(&mut self, c: &Constraint, objects: &Vec<Object>){
        if c.a >= objects.len() as u32 || c.b >= objects.len() as u32 {return;} 
        let loca = c.c_desc.apoint;
        let locb = c.c_desc.bpoint;
        
        self.addline(objects[c.a as usize].localtoglobal(loca),objects[c.b as usize].localtoglobal(locb),V3::new(0.,1.,1.));
    }
    pub fn get_line_colors(&self)->[f32;1024]{

        let mut out = [0.;1024];
        let mut i = 0;
        // /rintln!("{:?}", self.lineColors.len());
        for l in self.line_colors.iter(){
            out[i] = l.x as f32;
            out[i+1] = l.y as f32;
            out[i+2] = l.z as f32;
            i+=3;
        } 
        out
    }
    pub fn getpoints(&self)-> [f64;1024]{
        let out = [0.;1024];
        for P in self.dots.iter(){

        }
        out
    }
    pub fn addline(&mut self, a: Vector3<f64>, b: Vector3<f64>, color: Vector3<f64>){
        self.line_colors.push(color);
        self.lines.push((a,b));
       
    }
    pub fn add_set_of_points_as_connected_lines(&mut self, p: Vec<Vector3<f64>>, color: Vector3<f64>){
        for i in 0..p.len()-1{
            self.addline(p[i], p[i+1],color);
        }
    }
    fn adddot(&mut self, a: Vector3<f64>){
        self.dots.push(a);
    }

}
