use crate::math::{check_vector_type, normalize, Vec3D};

mod math;

fn main() {

    // println!();
    //
    // add_vec();
    //
    // println!();
    // sub_vec();
    //
    // println!();
    // scale_vec();
    //
    // println!();
    // vec_len();
    //
    // println!();
    // normalize_vec()

    // println!();
    // calc_dot_prod();
    //
    // println!();
    // calc_angles();

    //check_vector_types();

    println!();

    projecting_vectors();
}

fn add_vec() {

    //Sum
    let vec_a = math::Vec3D { x: 8.218, y: -9.341, z: 0.0 };
    let vec_b = math::Vec3D { x: -1.129, y: 2.111, z: 0.0 };

    println!("Calculating Addition of Vectors ( {} ) and ( {} )", vec_a, vec_b);

    let vec_sum = math::add_vec(&vec_a, &vec_b);

    println!("Result is {}", vec_sum);
}

fn sub_vec() {
    let vec_a = math::Vec3D { x: 7.119, y: 8.215, z: 0.0 };
    let vec_b = math::Vec3D { x: -8.223, y: 0.878, z: 0.0 };

    println!("Calculating Subtraction of Vectors ( {} ) and ( {} )", vec_a, vec_b);

    let vec_sub = math::sub_vec(&vec_a, &vec_b);

    println!("Result is {}", vec_sub);
}

fn scale_vec() {
    let vec_a = math::Vec3D { x: 1.671, y: -1.012, z: -0.318 };
    let scalar = 7.41;

    println!("Scaling Vector {} by {}", vec_a, scalar);

    let vec_scaled = math::scale_vec(&vec_a, scalar);

    println!("ScaledVec:  {}", vec_scaled);
}

fn vec_len() {
    let vec_a = math::Vec3D { x: -0.221, y: 7.437, z: 0.0 };

    let vec_b = math::Vec3D { x: 8.813, y: -1.331, z: -6.247 };

    println!("Calculating magnitude of Vector a {}", vec_a);

    let magnitude = math::vec_length(&vec_a);

    println!("Magnitude of Vector A is {}", magnitude);

    println!("Calculating magnitude of Vector B {}", vec_b);

    let magnitude = math::vec_length(&vec_b);

    println!("Magnitude of Vector B is {}", magnitude);
}

fn normalize_vec() {
    let vec_a = Vec3D { x: 5.581, y: -2.136, z: 0.0 };
    let vec_b = Vec3D { x: 1.996, y: 3.108, z: -4.554 };

    println!("Vector A: {}", vec_a);
    println!("Normalized Vector A {}", math::normalize(&vec_a));

    println!("Vector B: {}", vec_b);
    println!("Normalized Vector B {}", math::normalize(&vec_b));
}

fn calc_dot_prod() {
    let vec_a = Vec3D { x: 7.887, y: 4.138, z: 0.0 };
    let vec_b = Vec3D { x: -8.802, y: 6.776, z: 0.0 };

    println!("Dot Product of A {} and B {} is {}", vec_a, vec_b, math::dot_product(&vec_a, &vec_b));

    let vec_a = Vec3D { x: -5.955, y: -4.904, z: -1.874 };
    let vec_b = Vec3D { x: -4.496, y: -8.755, z: 7.103 };

    println!("Dot Product of A {} and B {} is {}", vec_a, vec_b, math::dot_product(&vec_a, &vec_b));
}

fn calc_angles() {
    let vec_a = Vec3D { x: 3.183, y: -7.627, z: 0.0 };
    let vec_b = Vec3D { x: -2.668, y: 5.319, z: 0.0 };

    println!("Angle between A {} and B {} is {}", vec_a, vec_b, math::arccos(&vec_a, &vec_b));

    let vec_a = Vec3D { x: 7.35, y: 0.221, z: 5.188 };
    let vec_b = Vec3D { x: 2.751, y: 8.259, z: 3.985 };

    println!("Angle (degress) between A {} and B {} is {}", vec_a, vec_b, math::degress(math::arccos(&vec_a, &vec_b)));

    let vec_a = Vec3D { x: 0.0, y: 5.0, z: 0.0 };
    let vec_b = Vec3D { x: 5.0, y: 0.0, z: 0.0 };

    println!("Angle (degress) between A {} and B {} is {}", vec_a, vec_b, math::degress(math::arccos(&vec_a, &vec_b)));
}

fn check_vector_types() {
    let vec_a = Vec3D { x: -7.579, y: -7.88, z: 0.0 };
    let vec_b = Vec3D { x: 22.737, y: 23.64, z: 0.0 };

    let vec_type = check_vector_type(&vec_a, &vec_b);

    println!("Vector {} and {} are {:?}", vec_a, vec_b, vec_type);

    let vec_a = Vec3D { x: -2.029, y: 9.97, z: 4.172 };
    let vec_b = Vec3D { x: -9.231, y: -6.639, z: -7.245 };

    let vec_type = check_vector_type(&vec_a, &vec_b);

    println!("Vector {} and {} are {:?}", vec_a, vec_b, vec_type);

    let vec_a = Vec3D { x: -2.328, y: -7.284, z: -1.214 };
    let vec_b = Vec3D { x: -1.821, y: 1.072, z: -2.94 };

    let vec_type = check_vector_type(&vec_a, &vec_b);

    println!("Vector {} and {} are {:?}", vec_a, vec_b, vec_type);
    //
    // let vec_a = Vec3D { x: 2.118, y: 4.827, z: 0.0 };
    // let vec_b = Vec3D { x: 0.0, y: 0.0, z: 0.0 };
    //
    // let vec_type = check_vector_type(&vec_a, &vec_b);
    //
    // println!("Vector {} and {} are {:?}", vec_a, vec_b, vec_type);
}

fn projecting_vectors() {
    let vec_v = Vec3D {
        x: 3.039,
        y: 1.879,
        z: 0.0,
    };

    let vec_b = Vec3D {
        x: 0.825,
        y: 2.036,
        z: 0.0,
    };

    println!("Proj V {} onto B {} is {}", vec_v, vec_b, math::projection(&vec_b, &vec_v));

    let vec_v = Vec3D {
        x: -9.88,
        y: -3.264,
        z: -8.159,
    };

    let vec_b = Vec3D {
        x: -2.155,
        y: -9.353,
        z: -9.473,
    };

    println!("Perp vector of V {} is {}",
             vec_v,
             math::get_perp_vector(
                 &vec_v,
                 &math::projection(
                     &vec_b,
                     &vec_v,
                 ),
             )
    );

    let vec_v = Vec3D {
        x: 3.009,
        y: -6.172,
        z: 3.692,
    };

    let vec_b = Vec3D {
        x: 6.404,
        y: -9.144,
        z: 2.759,
    };

    let proj_v_onto_b = math::projection(&vec_b, &vec_v);
    println!("Projecting V {} onto B {} we can separate V in V Perp {} and V Parallel {}",
             vec_v,
             vec_b,
             &proj_v_onto_b,
             math::get_perp_vector(&vec_v, &proj_v_onto_b),
    );
}