extern crate rand;
extern crate nalgebra;

use rand::Rng;
pub enum BasicShape{
    Cube([f32;3]),
    Pyramid,
    Sphere,
}
use nalgebra::*;
type V3 = Vector3<f32>; 
pub struct Object {
    pub p: V3,
    v: V3,
    pub o: UnitQuaternion<f32>,
    a: Vector4<f32>,
    pub dim: [f32;3],
    pub data: Vec<V3>,
    pub collider: Vec<V3>, 
    iM: f32,
    iI: Matrix3<f32>,
}
fn CreateDefaultObject()-> Object{
    Object { p: V3::new(0.,0.,0.),collider: vec![], dim: [1.,1.,1.], v: V3::new(0.,0.,0.), o: UnitQuaternion::default(), a: Vector4::<f32>::new(0.,0.,0.,0.), data: vec![], iM: 0. , iI: Matrix3::default() }
}
impl Object {
    pub fn new(pp: V3, ss: BasicShape )-> Object{
        let mut temp = CreateDefaultObject(); 
        match ss{
            BasicShape::Cube(dim) => {
                temp.p = pp;
                let mut rng = rand::thread_rng();
                let mut rd= ||{rng.gen_range(-1.0f32..1.0)};
                temp.a = Vector4::new(rd(),rd(),rd(),rd()*0.4).normalize();
                // temp.a = Vector4::new(0.,1.,0.,0.4).normalize();
                temp.dim = dim;
                let dir:[f32;2] = [0.5,-0.5];
                for i in 0..2{
                    for j in 0..2{
                        for k in 0..2{
                            temp.data.push(V3::new(dir[i]*dim[0],dir[j]*dim[1],dir[k]*dim[2]))
                        }
                    }
                }
            },

            BasicShape::Pyramid => {},
            BasicShape::Sphere => {},

        };
        temp
    }

    pub fn generateCollider(&mut self){
        self.collider.clear();
        for p in self.data.iter() {
            self.collider.push(self.localtoglobal(p.clone()));
        }
    } 
    pub fn update(& mut self, dt: f32, ct: f32){
        self.p+= self.v* dt;
        // let mot = ct.sin()*3.;
        // self.p = V3::new(mot,0.,2.);
        // println!("pos : {:?} ", self.p);
        let axis = self.a.xyz();
        let axisn = UnitVector3::new_normalize(axis); 
        // println!("axis from object update: {:?} ", axisn);
        self.o = self.o * nalgebra::UnitQuaternion::from_axis_angle(&axisn,self.a.w*dt); 
        // println!("quat: {:?}", self.o);
    }

    pub fn localtoglobal(&self, p: V3)-> V3{
        self.o * p + self.p
    }
    pub fn globaltolocal(&self, p: V3)-> V3{
        self.o *  (p - self.p) 
    }
}
