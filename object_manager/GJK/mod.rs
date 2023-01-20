use nalgebra::Vector3;
use rand::Rng;
use super::object::Object;
type V3 = Vector3<f64>;

#[derive(Clone)]
pub struct Collider {
    pub data: Vec<V3>,
}
fn clamp(x:f64,a:f64,b:f64)->f64{
    f64::min(b,f64::max(a,x))
}
impl Collider {
    pub fn get_support(&self, dir: V3)-> V3{
        let mut out = V3::default();
        let mut maxv = -std::f64::MAX;
        for p in self.data.iter(){
            let dist = p.dot(&dir);
            if dist > maxv {
                out = p.clone();
                maxv = dist; 
            }
        } 
        out
    }
    fn get_cso_support(&self, other: &Self, dir: V3)-> V3{
        let a = self.get_support(dir); 
        let b = other.get_support(-dir);
        a-b
    }
}
#[derive (Debug)]
pub enum Simplex{
    One(V3),
    Two(V3,V3),
    Thr(V3,V3,V3),
    Fou(V3,V3,V3,V3),
}

#[derive (Debug)]
pub enum SType {
    Vert(V3),
    Line(V3,V3),
    Face(V3,V3,V3),
}
impl SType{
    pub fn to_simplex(&self)->Simplex{
        match self {
            SType::Vert(p) => Simplex::One(p.clone()),
            SType::Line(p,p1) => Simplex::Two(p.clone(),p1.clone()),
            SType::Face(p,p1,p2) => Simplex::Thr(p.clone(),p1.clone(),p2.clone()),
        }
    }
}
fn line_closest(p:& V3, d: V3, dn: V3)->(V3,SType){
    let dist = -p.dot(&dn);
    let dlen = d.norm();
    let distc = clamp(dist, 0., dlen);
    let areas = (dist < 0., dist > dlen);
    let a = p; 
    let b = p + dlen*dn;
    let pout = p + distc*dn;
    match areas {
        (false,false) => (pout,SType::Line(a.clone(), b.clone())),
        (true,false) => (a.clone(),SType::Vert(a.clone())),
        (false,true) => (b,SType::Vert(b)),
        (true,true) => {
            println!("{:?}","from Gjk line_closes");
            (p.clone(),SType::Vert(a.clone()))
        },
    }
}
pub fn triangle_closest(i:& V3, j:& V3, k:& V3)->(V3,SType){
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
            (v*i + w*j + u*k, SType::Face(i.clone(),j.clone(),k.clone()))
        },
        (true,true,false) => {
            line_closest(k, ki,kin)
        },
        (true,false,false) => {
            (k.clone(), SType::Vert(k.clone()))
        },
        (true,false,true) => {
            line_closest(j, jk,jkn)
        },
        (false,true,true) => {
            line_closest(i, ij, ijn)
        },
        (false,true,false) => {
            (i.clone(), SType::Vert(i.clone()))
        },
        (false,false,false) => {
            (j.clone(), SType::Vert(j.clone()))
        },
        (false,false,true) => {
            (j.clone(), SType::Vert(j.clone()))
        },
    }

}
fn order3dsimplex(a: & V3,b: & V3,c: & V3,d: & V3, ab: V3, ac: V3)-> (V3,V3,V3,V3){
    let insidep = a*0.25 + b*0.25 + c*0.25 + d*0.25;
    if (insidep - a).dot(&ac.cross(&ab)) > 0. {
        return (b.clone(),a.clone(),c.clone(),d.clone());
    }
    (a.clone(),b.clone(),c.clone(),d.clone())
}

pub fn simplex_closest(ai:& V3, bi:& V3, ci:& V3, di:& V3) -> (V3,SType) {
    let (a,b,c,d) = order3dsimplex(ai,bi,ci,di,bi-ai,ci-ai);

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
        (true,true,true,true)  => {(V3::new(0.,0.,0.),SType::Vert(V3::new(0.,0.,0.)))},
        (true,true,true,false) => {triangle_closest(&b,&c,&d)},
        (true,true,false,true) => {triangle_closest(&a,&d,&b)},
        (true,false,true,true) => {triangle_closest(&a,&c,&d)},
        (false,true,true,true) => {triangle_closest(&a,&b,&c)},

        (true,true,false,false) => {
            let p = bd;
            line_closest(&b, p, p.normalize())
        },
        (true,false,false,true) => {
            let p = d-a;
            line_closest(&a, p, p.normalize())
        },
        (false,false,true,true) => {
            let p = c-a;
            line_closest(&a, p, p.normalize())
        },
        (false,true,false,true) => {
            let p = b-a;
            line_closest(&a, p, p.normalize())
        },
        (true,false,true,false) => {
            let p = d-c;
            line_closest(&c, p, p.normalize())
        },
        (false,true,true,false) => {
            let p = b-c;
            line_closest(&c, p, p.normalize())
        },

        (true,false,false,false) => {(d.clone(),SType::Vert(d.clone()))},
        (false,true,false,false) => {(b.clone(),SType::Vert(b.clone()))},
        (false,false,true,false) => {(c.clone(),SType::Vert(c.clone()))},
        (false,false,false,true) => {(a.clone(),SType::Vert(a.clone()))},

        (false,false,false,false) => {(a.clone(),SType::Vert(a.clone()))},
    }
}

impl Simplex {
    fn add(& self, sup: V3)-> Option<Simplex>{
        match self {
            Simplex::One(p) =>        {Some(Simplex::Two(p.clone(),sup))},    
            Simplex::Two(p,p2)=>      {Some(Simplex::Thr(p.clone(),p2.clone(),sup))},    
            Simplex::Thr(p,p2,p3)=>   {Some(Simplex::Fou(p.clone(),p2.clone(), p3.clone(), sup))},    
            Simplex::Fou(p,p2,p3,p4)=>{
                None
            }
        }
    }
    fn reduce(vs: & Vec<V3>, p: V3, dir: V3)->Simplex{
        let mut vout: Vec<V3> = vec![];
        let mut acc = 0;
        for v in vs.iter(){
            let vec = v-p;
            if vec.dot(&dir) < 0.0 {
                vout.push(v.clone());
            }
            else{
                acc+=1;
            }

        }
        println!("removed count: {:?}",acc);
        match vout.len() {
            0 => Simplex::One(p.clone()),
            1 => Simplex::One(vout[0]),
            2 => Simplex::Two(vout[0],vout[1]),
            3 => Simplex::Thr(vout[0],vout[1],vout[2]),
            4 => Simplex::Fou(vout[0],vout[1],vout[2],vout[3]),
            _ => {
                println!("{:?}", "from gjk simplex reduce, should be inacessible");
                Simplex::One(vout[0])
            },
        }
    }
    fn reduce_2(t: SType)->Simplex{
        t.to_simplex()
    }
    pub fn closest(&self)->(V3,SType){
        match self {
            Simplex::One(p)=> {
                (p.clone(),SType::Vert(p.clone()))
            },
            Simplex::Two(p, p2)=> {
                let ab = p2-p;
                let abn = ab.normalize();
                line_closest(p, ab, abn)
            },
            Simplex::Thr(p,p2,p3)=> {
                triangle_closest(p,p2,p3) 
            },
            Simplex::Fou(p1,p2,p3,p4)=>{
                simplex_closest(p1,p2,p3,p4)
            }
        }
    }
    fn to_vec(&self)-> Vec<V3> {
        match self {
            Simplex::One(p) => vec![p.clone()],
            Simplex::Two(p,p2) => vec![p.clone(),p2.clone()],
            Simplex::Thr(p,p2,p3) => vec![p.clone(),p2.clone(),p3.clone()],
            Simplex::Fou(p,p2,p3,p4) => vec![p.clone(),p2.clone(),p3.clone(),p4.clone()],
        }
    }
    fn count(&self)-> i32 {
        match self {
            Simplex::One(_) => 1,
            Simplex::Two(_,_) => 2,
            Simplex::Thr(_,_,_) => 3,
            Simplex::Fou(_,_,_,_) => 4,
        }
    }
} 
pub fn gjk(a:& Object, b:& Object)->bool{
    let mut rng = rand::thread_rng();
    let mut rd= || {rng.gen_range(-1.0f64..1.0)};
    let dir = V3::new(rd(),rd(),rd()).normalize();
    let mut sup = a.collider.get_cso_support(&b.collider, dir);
    let mut simp = Simplex::One(sup);
    let mut n_iter = 0;
    loop {
        //4
        let c = simp.closest();
        let dist_from_origin = c.0.norm(); 
        //5
        if dist_from_origin <0.1 {
            println!("return true after {:?} {}", n_iter, " iterations");
            return true;
        }
        println!("closest: {:?}", c.0);
        println!("dist from origin: {:?}",dist_from_origin);
        println!("sType: {:?}", c.1);

        //6
        let newdir = -c.0.normalize(); 
        println!("direction: {:?}",newdir);

//        simp = Simplex::Reduce2(&simp.toVec(), c.0,-newdir );
        simp = Simplex::reduce_2(c.1);

        //7
        println!("simplex before: {:?}", simp);
        sup = a.collider.get_cso_support(&b.collider, newdir );

        println!("simplex after: {:?}", simp);

        println!("closest: {:?} newsup: {}", c.0, sup);

        if sup.dot(&newdir) <  0.100{
            println!("false from gjk func after {:?} {}", n_iter, " iteration");
            return false;
        }
        let newsimp = simp.add(sup);

        match newsimp {
            Some(s) => {simp = s;},
            None => {panic!("Tried to add to a full simplex");},
        };
        println!("simplex after add: {:?}", simp);
        println!("new sup dist: {:?}", sup.dot(&newdir));

        if n_iter > 400 {
            panic!("GJK stuck");
        }
        n_iter += 1;
    }
}
