use std::fmt;

#[derive(Debug)]
pub enum VectorType {
    orthogonal,
    parallel,
    neitherOrthogonalOrParallel,
}

#[derive(Clone)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}, Y: {}, Z: {}", self.x, self.y, self.z)
    }
}

///
///
/// # Arguments
///
/// * `a`:
/// * `b`:
///
/// returns: Vec2D<T>
///
/// # Examples
///
/// ```
///
/// ```
pub fn add_vec(a: &Vec3D, b: &Vec3D) -> Vec3D {
    Vec3D {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

///
///
/// # Arguments
///
/// * `a`:
/// * `b`:
///
/// returns: Vec2D<T>
///
/// # Examples
///
/// ```
///
/// ```
pub fn sub_vec(a: &Vec3D, b: &Vec3D) -> Vec3D {
    Vec3D {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

///
///
/// # Arguments
///
/// * `a`:
/// * `scale`:
///
/// returns: Vec2D<T>
///
/// # Examples
///
/// ```
///
/// ```
pub fn scale_vec(a: &Vec3D, scale: f64) -> Vec3D {
    let new_x = a.x * scale;
    let new_y = a.y * scale;
    let new_z = a.z * scale;

    Vec3D { x: new_x, y: new_y, z: new_z }
}

///
///
/// # Arguments
///
/// * `a`:
///
/// returns: f64
///
/// # Examples
///
/// ```
///
/// ```
pub fn vec_length(a: &Vec3D) -> f64 {
    f64::sqrt((a.x * a.x) + (a.y * a.y) + (a.z * a.z))
}

///
///
/// # Arguments
///
/// * `a`:
///
/// returns: Vec3D
///
/// # Examples
///
/// ```
///
/// ```
pub fn normalize(a: &Vec3D) -> Vec3D {
    Vec3D {
        x: 1 as f64 / vec_length(&a) * a.x,
        y: 1 as f64 / vec_length(&a) * a.y,
        z: 1 as f64 / vec_length(&a) * a.z,

    }
}

///
///
/// # Arguments
///
/// * `a`:
/// * `b`:
///
/// returns: f64
///
/// # Examples
///
/// ```
///
/// ```
pub fn dot_product(a: &Vec3D, b: &Vec3D) -> f64 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

///
///
/// # Arguments
///
/// * `a`:
/// * `b`:
///
/// returns: f64
///
/// # Examples
///
/// ```
///
/// ```
pub fn arccos(a: &Vec3D, b: &Vec3D) -> f64 {
    f64::acos(
        dot_product(
            &normalize(&a),
            &normalize(&b),
        )
    )
}

pub fn degress(a: f64) -> f64 {
    a * (180.0 / std::f64::consts::PI)
}

pub fn is_zero_vector(a: &Vec3D) -> bool {
    return f64::abs(vec_length(a)) < 10.0;
}


///
///
/// # Arguments
///
///  return wether Vec3D is orthogonal or parallel.
/// * `a`: Vec3D to be checked
/// * `b`: Vec3D to be compared to.
///
/// returns: VectorType
///
pub fn check_vector_type(a: &Vec3D, b: &Vec3D) -> VectorType {
    let dot_product = dot_product(a, b);

    println!("Dot product {}", dot_product);

    if dot_product == 0.0 {
        return VectorType::orthogonal;
    }

    println!("A is zero vector {}", is_zero_vector(&a));
    println!("B is zero vector {}", is_zero_vector(&b));
    println!("angle between A and B is {}", arccos(&a, &b));

    if is_zero_vector(&a) ||
        is_zero_vector(&b) ||
        arccos(&a, &b) == 0.0 ||
        arccos(&a, &b) == std::f64::consts::PI {
        return VectorType::parallel;
    }

    return VectorType::neitherOrthogonalOrParallel;
}