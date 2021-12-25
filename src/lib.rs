mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;
extern crate nalgebra as na;
use na::{Vector2,vector};
use rand::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static ZERO: Vector2<f64> = vector!(0.0,0.0);

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! logf {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Copy,Clone)]
pub enum BodyType {
    Particle = 0,
    Comet = 1,
    Star = 2,
    Planet = 3,
}


#[wasm_bindgen]
#[derive(Copy,Clone)]
pub struct Body {
    id: u64,
    k: BodyType,
    lifetime: u64,
    m: f64,
    q: Vector2<f64>,
    qp: Vector2<f64>,
    a: Vector2<f64>,
}

#[wasm_bindgen]
pub struct BodyBuilder {
    body: Body
}

#[wasm_bindgen]
pub struct System {
    keygen: u64,
    arr: Vec<Body>
}


#[wasm_bindgen]
impl Body {
    pub fn star() -> Body {
        Body {
            id: 0,
            k: BodyType::Star,
            lifetime: 0,
            m: 1000.0,
            q: ZERO,
            qp: ZERO,
            a: ZERO,
        }
    }

    pub fn comet() -> BodyBuilder {
        BodyBuilder{ body: Body {
            id: 0,
            k: BodyType::Comet,
            lifetime: 0,
            m: 1.0,
            q: ZERO,
            qp: ZERO,
            a: ZERO,
        }}
    }

    pub fn dust(parent: u64) -> BodyBuilder {
        BodyBuilder{ body: Body {
            id: parent,
            k: BodyType::Particle,
            lifetime: 0,
            m: 0.00001,
            q: ZERO,
            qp: ZERO,
            a: ZERO,
        }}
    }
    
    pub fn qx(&self) -> f64 {
        self.q[0]
    }
    
    pub fn qy(&self) -> f64 {
        self.q[1]
    }
    
    pub fn mass(&self) -> f64 {
        self.m
    }
    
    pub fn kind(&self) -> u8 {
        self.k as u8
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.q[0], self.q[1])
    }
}

#[wasm_bindgen]
impl BodyBuilder {
    pub fn mass(mut self,m: f64) -> BodyBuilder {
        self.body.m = m;
        self
    }
    
    pub fn position(mut self,x: f64, y: f64) -> BodyBuilder {
        self.body.q = vector!(x,y);
        self
    }
    
    pub fn velocity(mut self,x: f64, y: f64) -> BodyBuilder {
        //interpret this field as velocity for the first iteration
        self.body.qp = vector!(x,y);
        self
    }
    
    pub fn build(self) -> Body {
        self.body
    }
}

#[wasm_bindgen]
impl System {
    pub fn new() -> System {
        System {
            keygen: 0,
            arr: vec!()
        }
    }
    
    pub fn add(&mut self,mut body: Body) {
        body.id = self.keygen;
        self.keygen += 1;
        self.arr.push(body);
    }
    
    pub fn count(&self) -> u32 {
        self.arr.len() as u32
    }
    
    pub fn body(&self,index: usize) -> Body {
        self.arr[index]
    }

    pub fn tick(&mut self){
        let dt = 0.01;//0.0167;
        let g = 0.00001;
        let mut rng = rand::thread_rng();
        
        let mut i = 0;
        while i < self.arr.len() {
            let particle = matches!(self.arr[i].k,BodyType::Particle);
            
            if particle && self.arr[i].lifetime > 100 {
                self.arr.swap_remove(i);
                continue;
            }
            
            if self.arr[i].q.norm() > 2.0 {
                self.arr.swap_remove(i);
                continue;
            }
            
            i += 1;
        }
        
        let n = self.arr.len();
        
        for i in 0..(n-1) {
            for j in (i+1)..n {
                let qi = &self.arr[i].q;
                let qj = &self.arr[j].q;
                let mi = self.arr[i].m;
                let mj = self.arr[j].m;
                
                let u = qj - qi;
                let r = u.norm();
                let u = u/r;
                let r2 = r*r;
                let f = u*g/r2;
                
                self.arr[i].a +=  mj*f;
                self.arr[j].a += -mi*f;
                
                if i == 0 {
                    let comet = matches!(self.arr[j].k,BodyType::Comet);
                    if comet && rng.gen::<f64>() < (dt/r2) {
                        let q = self.arr[j].q;
                        let qp = self.arr[j].qp;
                        let v = (q - qp)/dt;
                        let dq = q + u*dt;
                        self.add(
                            Body::dust(self.arr[j].id)
                            .mass(0.00001)
                            .position(dq[0],dq[1])
                            .velocity(v[0],v[1])
                            .build()
                        );
                    }
                    
                    //Solar radiation pressure
                    self.arr[j].a += 0.001*u/(r2*mj.sqrt());
                }
            }
        }
        
        for i in 0..n {
            let p = &mut self.arr[i];
            let qn = if p.lifetime == 0 {
                let v = p.qp; //interpret qp as velocity for the first iteration
                p.q + v*dt + 0.5*p.a*dt*dt
            } else {
                2.0*p.q - p.qp + p.a*dt*dt
            };
            
            p.qp = p.q;
            p.q = qn;
            p.a = ZERO;
            p.lifetime += 1;
        }
    }
}
