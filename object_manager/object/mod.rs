extern crate rand;
extern crate nalgebra;

use rand::Rng;
#[allow(dead_code)]
pub enum BasicShape{
    Cube([f32;3]),
    Pyramid,
    Sphere(f32),
}
use nalgebra::*;

use super::GJK::Collider;
type V3 = Vector3<f32>; 
type V4 = Vector4<f32>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Object {
    pub p: V3,
    pub old_p: V3,
    v: V3,
    pub o: UnitQuaternion<f32>,
    pub old_o: UnitQuaternion<f32>,
    a: V3,
    pub dim: [f32;3],
    pub data: Vec<V3>,
    pub collider: Collider, 
    f_ext: V3,
    t_ext: V3,

    pub m: f32,
    i_m: f32,
    inertia_tensor_local: Matrix3<f32>,
    i_t: Matrix3<f32>,
    pub ii_t: Matrix3<f32>,
}
fn create_default_object()-> Object{
    Object {m: 20.,inertia_tensor_local: Matrix3::default(), t_ext: V3::default(), f_ext: V3::default(), old_p: V3::new(0.,0.,0.),p: V3::new(0.,0.,0.),collider: Collider{data: vec![]}, dim: [1.,1.,1.], v: V3::new(0.,0.,0.), old_o: UnitQuaternion::default(),o: UnitQuaternion::default(), a: Vector3::<f32>::new(0.,0.,0.), data: vec![], i_m: 0. , ii_t: Matrix3::default(), i_t: Matrix3::default() }
}
fn quaternion_to_rotation_matrix(q: UnitQuaternion<f32>){

}
impl Object {
    pub fn generate_rectangle_tensor(w: f32, h: f32, d: f32, mass: f32)-> Matrix3<f32>{
        
        let w2 = w*w; let h2 = h*h; let d2 = d*d;
        
        let mo12 = mass/12.;
        let x = (w2+d2)*mo12;
        let y = (d2+h2)*mo12;
        let z = (w2+h2)*mo12;
        
        
        let mut m = Matrix3::<f32>::new(x ,0.,0.,
                                        0.,y ,0.,
                                        0.,0.,z );
        
        return m;

    }
    pub fn new(pp: V3, ss: BasicShape )-> Object{
        let mut temp = create_default_object(); 
        match ss{
            BasicShape::Cube(dim) => {
                let it = Object::generate_rectangle_tensor(dim[0], dim[1], dim[2], temp.m);
                temp.inertia_tensor_local = it;
                temp.p = pp;
                temp.m = 500.;
                let mut rng = rand::thread_rng();
                let mut rd= ||{rng.gen_range(-1.0f32..1.0)};
                temp.a = Vector3::new(1.,0.4,0.).normalize()*4.;
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
            BasicShape::Sphere(r) => {},
        };
        temp
    }
    pub fn generate_collider(&mut self){
        self.collider.data.clear();
        for p in self.data.iter() {
            self.collider.data.push(self.localtoglobal(p.clone()));
        }
        self.i_t = self.inertia_tensor_local;// * self.o.to_rotation_matrix();
        let iitopt = self.i_t.try_inverse();
        self.ii_t = iitopt.unwrap(); 
    } 
    pub fn update(&mut self, h: f32){
        self.old_p = self.p;
        self.v += (self.f_ext*self.i_m*h).xyz();
        self.p += self.v* h;

        self.old_o = self.o;
        self.a += self.ii_t * (self.t_ext - (self.a.cross(&(self.i_t*self.a))))* h; 
//        self.o = UnitQuaternion::new_normalize(q3); 
        
        let axisn = UnitVector3::new_normalize(self.a); 
        self.o *= nalgebra::UnitQuaternion::from_axis_angle(&axisn,self.a.norm()*h); 
        self.o = UnitQuaternion::new_normalize(self.o.normalize());
        
    }
    pub fn update_velocities(&mut self, h: f32){
        self.v = (self.p - self.old_p)/h;
        let dq = self.o * self.old_o.inverse(); 
        let (axis,angle) = dq.axis_angle().unwrap(); 

        self.a = 2.*axis.normalize() *angle /h;      
        self.a = if dq.w >=0. {self.a}else{-self.a};


    }

    pub fn localtoglobal(&self, p: V3)-> V3{
        self.o * p + self.p
    }
    pub fn globaltolocal(&self, p: V3)-> V3{
        self.o *  (p - self.p) 
    }
}
