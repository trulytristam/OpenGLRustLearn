mod object;
mod GJK;
extern crate nalgebra;
use nalgebra::*;
mod graphic_debug;
mod physics_manager;
use GJK::*;
use object::Object;
use physics_manager::*;
use object::BasicShape;
use physics_manager::*;
use graphic_debug::GraphicDebug;
type V3 = Vector3<f64>;
pub struct ObjectManager {
    pub screen_dim: (f64,f64),
    pub cam: (V3,UnitQuaternion<f64>),
    pub debug: GraphicDebug, 
    objects: Vec<Object>,
    physics_manager: PhysicsManager,
}

impl ObjectManager {
    pub fn new()-> Self{
        let o: Vec<Object> = vec![];
        let d = GraphicDebug{lines: vec![],line_colors: vec![], dots: vec![]};
        let mut om = ObjectManager{physics_manager: PhysicsManager::new(), objects: o,cam: (V3::new(0.,0.,-1.), UnitQuaternion::default()), debug: d, screen_dim: (0.,0.)};
        om.add_object(V3::new(0.0,2.,4.5),BasicShape::Cube([1.,3.,1.]), true, 10., V3::new(1.,0.,0.));
        om.add_object(V3::new(-2.0,0.,3.5),BasicShape::Cube([3.,1.2,0.2]), false ,1.5, V3::new(-1.,0.,0.));
        
        let desc = ConstraintDesc{
            apoint: V3::new(0.5,-1.5,0.5),
            bpoint: V3::new(1.5,0.6,0.1),
            compliance: 0.0001000,
            dist:  2., 
        };
        let desc2 = ConstraintDesc{
            apoint: V3::new(-0.5,-1.5,-0.5),
            bpoint: V3::new(1.5,-0.6,0.1),
            compliance: 0.0000100,
            dist:  2., 
        };
        
        om.physics_manager.add_constraint(0, 1, desc);
        om.physics_manager.add_constraint(0, 1, desc2);
        om
    }
    pub fn add_object(&mut self, p: V3, s: BasicShape, bstatic: bool, d: f64, a: V3){ match s{
            BasicShape::Cube(dim) => self.objects.push(Object::new(p, BasicShape::Cube(dim), bstatic, d,a) ),
            BasicShape::Pyramid => {},
            BasicShape::Sphere(r) => {} ,
        } 
    }
    pub fn get_len(&self) -> f32{
        self.objects.len() as f32
    }
    pub fn update(&mut self, dt: f64, ct: f64, dim: (f64,f64)){
        self.screen_dim = dim;
        self.debug.clear();
        self.physics_manager.update_physics(&mut self.objects, dt,ct);

//        self.add_debug_lines_for_cube(0, V3::new(1.,1.,0.)); 
//        self.add_debug_lines_for_cube(1,V3::new(1.,0.,1.)); 
        
        self.debug.debug_constraint(&self.physics_manager.constraints[0], &self.objects);
        self.debug.debug_constraint(&self.physics_manager.constraints[1], &self.objects);
    }
    

    fn add_debug_lines_for_cube(&mut self,  id: usize, color: V3) {

            if id >= self.objects.len() {return}
            let out = vec![
                self.objects[id].collider.data[0],
                self.objects[id].collider.data[1],
                self.objects[id].collider.data[3],
                self.objects[id].collider.data[2],
                self.objects[id].collider.data[0],
                self.objects[id].collider.data[4],
                self.objects[id].collider.data[6],
                self.objects[id].collider.data[7],
                self.objects[id].collider.data[5],
                self.objects[id].collider.data[4],
            ];
            self.debug.add_set_of_points_as_connected_lines(out, color); 
            self.debug.addline(self.objects[id].collider.data[2],self.objects[id].collider.data[6], color);
            self.debug.addline(self.objects[id].collider.data[3],self.objects[id].collider.data[7], color);
            self.debug.addline(self.objects[id].collider.data[1],self.objects[id].collider.data[5], color);
    }
    pub fn get_object_position(&self)->[f32;1024]{
        let mut vp  = [0.;1024]; 
        for oi in 0..self.objects.len(){
            let o = self.cam.1.inverse() * (&self.objects[oi].p- self.cam.0);
            vp[oi*3]= o.x as f32;
            vp[oi*3+1]= o.y as f32;
            vp[oi*3+2]= o.z as f32;
        }   
        vp
    }
    pub fn get_object_dims(&self)->[f32;1024]{
        let mut vp = [0.;1024];
        for oi in 0..self.objects.len(){
            vp[oi*3] = self.objects[oi].dim[0] as f32;
            vp[oi*3+1] = self.objects[oi].dim[1] as f32;
            vp[oi*3+2] = self.objects[oi].dim[2] as f32;
        }
        vp
    }
    pub fn get_object_orientations(&self)->[f32;4096]{
        let mut vpout  = [0.;4096];
        let mut i = 0;
        for o in self.objects.iter(){
            let or =(self.cam.1.inverse() * o.o).to_rotation_matrix();
            for j in 0..3{
                for k in 0..3{
                    vpout[i*9+(j*3+k)]= or[(j,k)] as f32; 
                }
            }
            i+=1;
        }   

        // println!("mat: {:?}", vpout[0]); 
        vpout
    }

}
