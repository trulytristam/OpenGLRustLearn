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

fn project(p: V3, d: V3, dn: V3)->V3{
    p + clamp(-p.dot(&dn),0.,d.norm())*dn  

}
pub fn barycentric3(i: V3, j: V3, k: V3)->V3{
    let ij = j-i;
    let jk = k-j;
    let ki = i-k;
    
    let ijn = ij.normalize();
    let jkn = jk.normalize();
    let kin = ki.normalize();

    let mut normal = ij.cross(&jk).normalize();
    let ivn = ijn.cross(&normal).normalize();
    let jvn = jkn.cross(&normal).normalize();
    let kvn = kin.cross(&normal).normalize();
    
    let kdist = jk.dot(&ivn);
    let area = ij.norm() * kdist/2.;

    let mut a = -ivn.dot(&i);
    let mut b = -jvn.dot(&j);
    let mut c = -kvn.dot(&k);
    
    
    let mut u = a * ij.norm()/2./area;
    let mut v = b * jk.norm()/2./area;
    let mut w = 1.-u-v;

    let areas = (a < 0.,b < 0.,c < 0.);
    
    match areas {
        (true,true,true) => {
            v*i + w*j + u*k
        },
        (true,true,false) => {
            project(k, ki,kin)
        },
        (true,false,false) => {
            k
        },
        (true,false,true) => {
            project(j, jk,jkn)
        },
        (false,true,true) => {
            project(i, ij, ijn)
        },
        (false,true,false) => {
            i
        },
        (false,false,false) => {
            j
        },
        (false,false,true) => {
            j
        },
    }
     
}
pub fn simplex_closest(a: V3, b: V3, c: V3, d: V3) -> V3 {
    let ab = b-a;
    let ac = c-a;
    let ad = d-a;
    let bc = c-b;
    let bd = d-b; 

    let abn = ac.cross(&ab).normalize();
    let acn = ad.cross(&ac).normalize();
    let adn = ab.cross(&ad).normalize();
    let bdn = bc.cross(&bd).normalize();
    
    let a_s = abn.dot(&a);
    let b_s = acn.dot(&a);
    let c_s = adn.dot(&a);
    let d_s = bdn.dot(&d);

    let areas = (a_s > 0., b_s > 0., c_s >0., d_s > 0.);
//    println!("areas from gjk: {:?}",areas);
    match areas {
        (true,true,true,true)  => {V3::new(0.,0.,0.)},
        (true,true,true,false) => {barycentric3(b,c,d)},
        (true,true,false,true) => {barycentric3(a,d,b)},
        (true,false,true,true) => {barycentric3(a,c,d)},
        (false,true,true,true) => {barycentric3(a,b,c)},

        (true,true,false,false) => {
            let p = bd;
            project(b, p, p.normalize())
        },
        (true,false,false,true) => {
            let p = (d-a);
            project(a, p, p.normalize())
         },
        (false,false,true,true) => {
            let p = (c-a);
            project(a, p, p.normalize())
         },
        (false,true,false,true) => {
            let p = (b-a);
            project(a, p, p.normalize())
         },
        (true,false,true,false) => {
            let p = (d-c);
            project(c, p, p.normalize())
        },
        (false,true,true,false) => {
            let p = (b-c);
            project(c, p, p.normalize())
        },

        (true,false,false,false) => {d},
        (false,true,false,false) => {b},
        (false,false,true,false) => {c},
        (false,false,false,true) => {a},

        (false,false,false,false) => {a},
    }
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
                barycentric3(p.clone(),p2.clone(),p3.clone()) 
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
