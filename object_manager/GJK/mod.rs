extern crate nalgebra;
extern crate rand;
use nalgebra::*;
use rand::Rng;
type V3 = Vector3<f32>;


pub struct Collider {
    data: Vec<V3>,
}
fn clamp(x:f32,a:f32,b:f32)->f32{
    f32::min(b,f32::max(a,x))
}
impl Collider {
    fn getSupport(&self, dir: V3)-> Option<V3>{
        let mut out : Option<V3> = None;
        let mut max = std::f32::MAX;
        for p in self.data.iter(){
            let dist = p.dot(&dir);
            if dist < max && dist > 0. {
                out = Some(p.clone());
                max = dist; 
            }
        } 
        out
    }

    fn getCSOSupport(&self, other: &Self, dir: V3)-> Option<V3>{
        let a = self.getSupport(dir); 
        let b = self.getSupport(-dir);
        if a.is_some() && b.is_some() {
            Some(b.unwrap()-a.unwrap())
        }
        else{
            None
        }
    }
}

enum Simplex{
    One(V3),
    Two(V3,V3),
    Thr(V3,V3,V3),
    Fou(V3,V3,V3,V3),
}
pub fn barycentric3(i: V3, j: V3, k: V3)->V3{
    let ij = (j-i);
    let jk = (k-j);
    let ki = (i-k);
    let ijn = ij.normalize();
    let jkn = jk.normalize();
    let kin = ki.normalize();

    let ijp = -ki.dot(&ijn);
    let jkp = -ij.dot(&jkn);
    let kip = -jk.dot(&kin);

     
    let im =  (i+ijn*ijp);
    let jm =  (j+jkn*jkp);
    let km =  (k+kin*kip);

    let iv = (k-im); 
    let jv = (i-jm); 
    let kv = (j-km); 

    let ivn = iv.normalize();
    let jvn = jv.normalize();
    let kvn = kv.normalize();

    let io = clamp(-i.dot(&jvn)/jv.norm(), 0., 1.); 
    let jo = clamp(-j.dot(&kvn)/kv.norm(), 0., 1.); 
    let ko = clamp(-k.dot(&ivn)/iv.norm(), 0., 1.); 

    (io)*i + (jo) * j + (ko) * k


}
impl Simplex {
    pub fn Add(&self, p: V3){
    }
    fn Reduce(&self){

    }
    fn Closest(&self)->V3{
        match self {
            Simplex::One(p)=> {p.clone()},
            Simplex::Two(p, p2)=> {
                let ab = (p2-p).normalize();
                let d = ab.dot(p);
                ab * clamp(d,0.,ab.norm())
            },
            Simplex::Thr(p,p2,p3)=> {
                V3::new(0.,0.,0.)
            },
            Simplex::Fou(p1,p2,p3,p4)=>{
                V3::new(0.,0.,0.)
            }
        }
    }
} 
pub fn GJK(a:& Collider, b:& Collider){

    let mut rng = rand::thread_rng();
    let mut rd= ||{rng.gen_range(-1.0f32..1.0)};
    let mut dir = V3::new(rd(),rd(),rd()).normalize();
    let simp = Simplex::One(a.getCSOSupport(b, dir).unwrap());
     
    loop {

        //4
        let c = simp.Closest();
        //5
        if  c.norm()<0.001 {return;}
        //6
        simp.Reduce();



    } 

}
