use super::object::*;
extern crate nalgebra;
use nalgebra::*;
type V3 = Vector3<f64>; 
pub struct ConstraintDesc {
    pub apoint: V3,
    pub bpoint: V3,
    pub compliance: f64,
    pub dist: f64,
}
pub struct Constraint{
    pub a: u32,
    pub b: u32,
    pub c_desc: ConstraintDesc,
    lagrange: f64,
}
impl Constraint {
    fn solve_constraint(&mut self, objects: &mut Vec<Object>, h:f64){
        let o1: Object = objects[(self.a as usize)].clone(); 
        let o2: Object = objects[(self.b as usize)].clone(); 
        let r1_global = o1.localtoglobal(self.c_desc.apoint);
        let r2_global = o2.localtoglobal(self.c_desc.bpoint);
        

        let ii_ta = o1.ii_t;
        let ii_tb = o2.ii_t;
        let between = r2_global-r1_global;
        let n = between.normalize();
        let r1 = r1_global - o1.p; 
        let r2 = r2_global - o2.p;
        let c = self.c_desc.dist - between.norm()  ;

        let r1n1 = r1.cross(&n);
        let r2n2 = r2.cross(&n);
        let w1 = (1./o1.m) + (r1n1.transpose()*ii_ta*r1n1).x; 
        let w2 = (1./o2.m) + (r2n2.transpose()*ii_tb*r2n2).x; 
        
        let a = self.c_desc.compliance / (h*h);
        let dy = (-c-a* self.lagrange) /
                        (w1 + w2 + a);
        let p = n*dy;
        
        //update pos
        objects[(self.a as usize)].p += p/o1.m;
        objects[(self.b as usize)].p -= p/o2.m;

        let aq = ii_ta * (r1.cross(&p));
        let bq = ii_tb * (r2.cross(&p));
//        let bq = ii_tb * (r2.cross(&p));

        let o1on = o1.o.normalize();
        let o2on = o2.o.normalize();
        
        objects[(self.a as usize)].o = UnitQuaternion::new_normalize(o1on + 0.5* Quaternion::<f64>::new(0.,aq.x,aq.y,aq.z)*o1on); 
        objects[(self.b as usize)].o = UnitQuaternion::new_normalize(o2on - 0.5* Quaternion::<f64>::new(0.,bq.x,bq.y,bq.z)*o2on); 


        self.lagrange += dy;
    }
    fn initialize(&mut self){
        self.lagrange = 0.;
    }
}
pub struct PhysicsManager {
    pub constraints: Vec<Constraint>,
}


impl PhysicsManager {
    pub fn new()-> Self{
        PhysicsManager { constraints: vec![]}
    }
    pub fn update_physics(&mut self, objects: &mut Vec<Object>, dt: f64,ct: f64){
        let n_sub = 50;
        let h = dt/(n_sub as f64);
        self.init_contraints();
        for _ in 0..n_sub{
            for o in objects.iter_mut(){
                o.generate_collider();
                if !o.is_static {
                    o.update(h);
                }
                o.orient_ii_t();
            }        

            self.solve_positions(objects,h);
            for o in objects.iter_mut(){
                if !o.is_static {
                    o.update_velocities(h);
                }
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
    fn solve_positions(&mut self, objects: &mut Vec<Object>,h: f64){
        for c in self.constraints.iter_mut() {
            c.solve_constraint(objects, h);
        }
    }
}
