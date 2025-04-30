use std::f32::consts::PI;

pub(crate) enum Formas {
    Quadrado(f32),
    Circulo(f32),
    Ellipse(f32, f32),
    Triangle(f32, f32, f32),
    Cubo(f32),
    Cilindro(f32, f32),
    Esfera(f32),
}


impl Formas {
    
    pub(crate) fn perimeter(&self) -> Result<f32, &'static str> {
        match self {
            Formas::Quadrado(l) => Ok(4f32 **l), 
            Formas::Circulo(r) => Ok(PI*2f32**r),
            Formas::Ellipse(a, b) => Ok(PI*(3f32*(*a+*b) - ((3f32**a+b)*(*a*3f32**b)).sqrt())),
            Formas::Triangle(x, y, z) => Ok(*x+*y+*z),
            Formas::Cubo(l) => Ok(12f32**l),
            _ => Err("Cannot calculate perimeter of objects")
        }
    }
    pub(crate) fn area(&self) -> Result<f32, &'static str> {
        match self {
            Formas::Quadrado(x) => Ok(*x**x),
            Formas::Circulo(r) => Ok(PI**r**r),
            Formas::Ellipse(a, b) => Ok(PI**a**b),
            Formas::Triangle(x, y, z) => {
                let s = (*x+*y+*z)/2f32;
                Ok((s*(s-x)*(s-y)*(s-z)).sqrt())
            },
            _ => Err("Cannot calculate area of 3D objects")
        }
    }
    
    pub(crate) fn volume(&mut self) -> Result<f32, &'static str> {
        match self {
            Formas::Cubo(a) => Ok(*a**a**a),
            Formas::Cilindro(r, h) => Ok(PI**r**r**h),
            Formas::Esfera(r) => Ok((4f32/3f32)**r**r**r),
            _ => Err("Cannot calculate volume of 2D objects"),
        }
    }
}

fn main() {
    todo!()
}
