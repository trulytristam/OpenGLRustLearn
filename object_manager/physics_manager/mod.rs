use super::object::*;
extern crate nalgebra;
use nalgebra::*;
type V3 = Vector3<f32>; 
pub struct ConstraintDesc {
    pub apoint: V3,
    pub bpoint: V3,
    pub compliance: f32,
    pub dist: f32,
}
struct Constraint{
    a: u32,
    b: u32,
    c_desc: ConstraintDesc,
    lagrange: f32,
}
impl Constraint {
    fn solve_constraint(&mut self, objects: &mut Vec<Object>, dt:f32){
        let o1: Object = objects[(self.a as usize)].clone(); 
        let o2: Object = objects[(self.b as usize)].clone(); 
        let r1_global = o1.localtoglobal(self.c_desc.apoint);
        let r2_global = o2.localtoglobal(self.c_desc.bpoint);

        let between = r2_global-r1_global;
        let n = between.normalize();
        let r1 = r1_global - o1.p; 
        let r2 = r2_global - o2.p;
        let c = between.norm();

        let r1n1 = r1.cross(&n);
        let r2n2 = r2.cross(&n);
        let w1 = (1./o1.m) + (r1n1.transpose()*o1.ii_t*r1n1).norm(); 
        let w2 = (1./o2.m) + (r2n2.transpose()*o2.ii_t*r2n2).norm(); 
        
        let a = self.c_desc.compliance / (dt*dt);
        let dy = (-c-a* self.lagrange)/
                        (w1 + w2 + a);

        let p = n*dy;
        
        //update pos
        objects[(self.a as usize)].p += p/o1.m;
        objects[(self.b as usize)].p -= p/o2.m;

        let aq = o1.ii_t* (r1.cross(&p));
        let bq = o2.ii_t* (r2.cross(&p));
        objects[(self.a as usize)].o *=  nalgebra::UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(aq),aq.norm());
        objects[(self.b as usize)].o *= nalgebra::UnitQuaternion::from_axis_angle(&UnitVector3::new_normalize(bq),bq.norm());

        
        self.lagrange += dy;
    }
    fn initialize(&mut self){
        self.lagrange = 0.;
    }
}
pub struct PhysicsManager {
    constraints: Vec<Constraint>,
}


impl PhysicsManager {
    pub fn new()-> Self{
        PhysicsManager { constraints: vec![]}
    }
    pub fn update_physics(&mut self, objects: &mut Vec<Object>, dt: f32,ct: f32){
        let n_sub = 20;
        self.init_contraints();
        let h = dt/(n_sub as f32);
        for _ in 0..n_sub{
            for o in objects.iter_mut(){
                o.generate_collider();
                o.update(h);
            }        
            //self.solve_positions(objects,h);
            for o in objects.iter_mut(){
                o.update_velocities(h);
            }
        }
    }
    fn init_contraints(&mut self){
        for c in self.constraints.iter_mut(){
            c.initialize();
        }
    }
    pub fn add_constraint(&mut self, ai:u32, bi:u32, desc: ConstraintDesc){
        let c = Constraint {a:ai, b:bi, c_desc: desc, lagrange: 0.};
        self.constraints.push(c);
    }
    fn solve_positions(&mut self, objects: &mut Vec<Object>,h: f32){
        for c in self.constraints.iter_mut() {
            c.solve_constraint(objects, h);
        }
    }
}
