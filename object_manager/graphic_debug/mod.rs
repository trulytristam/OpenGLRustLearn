extern crate nalgebra;
use nalgebra::*;
pub struct GraphicDebug {
    pub lines: Vec<(Vector3<f32>,Vector3<f32>)>,
    pub lineColors: Vec<Vector3<f32>>,
    pub dots: Vec<Vector3<f32>>,
}

fn planeinter(p: Vector3<f32> , zplane: f32)->Option<Vector3<f32>>{
    let rd = p.normalize();
    let r = 1./rd.dot(&Vector3::<f32>::new(0.,0.,1.)); 
    if r > 0. {
        return Some(rd * zplane * r);
    }
    None
}
impl GraphicDebug {
    pub fn clear(&mut self){
        self.lines.clear();
        self.lineColors.clear();
        self.dots.clear();
    }
    pub fn getlines(&self, dim: (f32,f32), cam: (Vector3<f32>, UnitQuaternion<f32>))-> [f32;1024]{
        let mut out = [ 0.;1024];
        let mut j =0;
        let mut i = 0;
        for L in self.lines.iter(){
            let Lm0 = cam.1.inverse() * (L.0 - cam.0); 
            let Lm1 = cam.1.inverse() * (L.1 - cam.0); 
            let L2 = planeinter(Lm0, 0.7); 
            let L3 = planeinter(Lm1, 0.7);
            
            if !(L2.is_none()||L3.is_none()){
                out[j] = L2.unwrap().x;
                out[j+1] = L2.unwrap().y * (dim.0/dim.1);
                out[j+2] = L3.unwrap().x;
                out[j+3] = L3.unwrap().y* (dim.0/dim.1);
                j+=4;
            }
            i+=1;
        }
        out
    }
    pub fn getLineColors(&self)->[f32;1024]{
        let mut out = [0.;1024];
        let mut i = 0;
        // /rintln!("{:?}", self.lineColors.len());
        for l in self.lineColors.iter(){
            out[i] = l.x;
            out[i+1] = l.y;
            out[i+2] = l.z;
            i+=3;
        } 
        out
    }
    pub fn getpoints(&self)-> [f32;1024]{
        let out = [0.;1024];
        for P in self.dots.iter(){

        }
        out
    }
    pub fn addline(&mut self, a: Vector3<f32>, b: Vector3<f32>, color: Vector3<f32>){
        self.lineColors.push(color);
        self.lines.push((a,b));
       
    }
    pub fn addSetOf3DPoints_asConnectedLines(&mut self, p: Vec<Vector3<f32>>, color: Vector3<f32>){
        for i in 0..p.len()-1{
            self.addline(p[i], p[i+1],color);
        }
    }
    fn adddot(&mut self, a: Vector3<f32>){
        self.dots.push(a);
    }

}
