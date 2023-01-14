mod object;
mod GJK;
extern crate nalgebra;
use nalgebra::*;
mod graphic_debug;
use GJK::*;
use object::Object;
use object::BasicShape;
use graphic_debug::GraphicDebug;
type V3 = Vector3<f32>;
pub struct ObjectManager {

    pub screenDim: (f32,f32),
    pub cam: (V3,UnitQuaternion<f32>),
    pub debug: GraphicDebug, 
    objects: Vec<Object>,


}

impl ObjectManager {
    pub fn new()-> Self{
        let o: Vec<Object> = vec![];
        let d = GraphicDebug{lines: vec![],lineColors: vec![], dots: vec![]};
        let mut om = ObjectManager{ objects: o,cam: (V3::new(0.,0.,0.), UnitQuaternion::default()), debug: d, screenDim: (0.,0.)};
        om.AddObject(V3::new(1.,0.,2.),BasicShape::Cube([2.,1.,0.4]));
        om.AddObject(V3::new(-1.,0.,2.),BasicShape::Cube([1.,1.2,0.4]));
        om
    }
    pub fn AddObject(&mut self,p: V3, s: BasicShape){ match s{
            BasicShape::Cube(dim) => self.objects.push(Object::new(p, BasicShape::Cube(dim)) ),
            BasicShape::Pyramid => {},
            BasicShape::Sphere => {} ,
        } 
    }
    fn GenerateColliders(&mut self){
        for o in self.objects.iter_mut(){
            o.generateCollider();
        }
    }
    pub fn getLen(&self) -> f32{
        (self.objects.len() as f32)
    }
    pub fn update(&mut self, dt: f32, ct: f32, dim: (f32,f32)){
        self.screenDim = dim;
        self.debug.clear();
        for o in &mut self.objects.iter_mut(){
            o.update(dt, ct);
        }
        //adding lines
        // self.addDebugLinesForCube(0, V3::new(1.,0.,0.)); 
        // self.addDebugLinesForCube(1,V3::new(0.,1.,0.)); 

        self.GenerateColliders();
        let o = V3::new(0.,0.,0.);
        let a = self.objects[1].localtoglobal(self.objects[1].data[0]); 
        let b = self.objects[1].localtoglobal(self.objects[1].data[2]); 
        let c = self.objects[1].localtoglobal(self.objects[1].data[6]); 
        let d = barycentric3(a, b, c);
        self.debug.addline(a,b,V3::new(0.,1.,0.));
        self.debug.addline(b,c,V3::new(0.,1.,0.));
        self.debug.addline(c,a,V3::new(0.,1.,0.));

        // self.debug.addline(d, d+V3::new(0.01,0.,0.), V3::new(1.,0.,0.));
        // self.debug.addline(o, o+V3::new(0.01,0.,0.), V3::new(1.,0.,0.));
        self.debug.addline(o, d, V3::new(0.5,0.,0.5));
    }
    

    fn addDebugLinesForCube(&mut self,  id: usize, color: V3) {

            let dx = self.objects[id].dim[0]/2.;
            let dy = self.objects[id].dim[1]/2.;
            let dz = self.objects[id].dim[2]/2.;
            if id >= self.objects.len() {return}
            let out = vec![
                self.objects[id].localtoglobal(V3::new(dx,dy,dz)),
                self.objects[id].localtoglobal(V3::new(-dx,dy,dz)),
                self.objects[id].localtoglobal(V3::new(-dx,dy,-dz)),
                self.objects[id].localtoglobal(V3::new(dx,dy,-dz)),
                self.objects[id].localtoglobal(V3::new(dx,dy,dz)),
                self.objects[id].localtoglobal(V3::new(dx,-dy,dz)),
                self.objects[id].localtoglobal(V3::new(-dx,-dy,dz)),
                self.objects[id].localtoglobal(V3::new(-dx,-dy,-dz)),
                self.objects[id].localtoglobal(V3::new(dx,-dy,-dz)),
                self.objects[id].localtoglobal(V3::new(dx,-dy,dz)),
            ];
            self.debug.addSetOf3DPoints_asConnectedLines(out, color); 
            self.debug.addline(self.objects[id].localtoglobal(V3::new(-dx,dy,-dz)), self.objects[id].localtoglobal(V3::new(-dx,-dy,-dz)), color);
            self.debug.addline(self.objects[id].localtoglobal(V3::new(-dx,dy,dz)), self.objects[id].localtoglobal(V3::new(-dx,-dy,dz)), color);
            self.debug.addline(self.objects[id].localtoglobal(V3::new(dx,dy,-dz)), self.objects[id].localtoglobal(V3::new(dx,-dy,-dz)), color);
    }
    pub fn getObjectPositions(&self)->[f32;1024]{
        let mut vp  = [0.;1024]; 
        for oi in 0..self.objects.len(){
            let o = self.cam.1.inverse() * (&self.objects[oi].p- self.cam.0);
            vp[oi*3]= o.x;
            vp[oi*3+1]= o.y;
            vp[oi*3+2]= o.z;
        }   
        vp
    }
    pub fn getObjectDims(&self)->[f32;1024]{
        let mut vp = [0.;1024];
        for oi in 0..self.objects.len(){
            vp[oi*3] = self.objects[oi].dim[0];
            vp[oi*3+1] = self.objects[oi].dim[1];
            vp[oi*3+2] = self.objects[oi].dim[2];
        }
        vp
    }
    pub fn getObjectOrientations(&self)->[f32;4096]{
        let mut vpout  = [0.;4096];
        let mut i = 0;
        for o in self.objects.iter(){
            let mut or =(self.cam.1.inverse() * o.o).to_rotation_matrix();
            for j in 0..3{
                for k in 0..3{
                    vpout[i*9+(j*3+k)]= or[(j,k)]; 
                }
            }
            i+=1;
        }   

        // println!("mat: {:?}", vpout[0]); 
        vpout
    }

}