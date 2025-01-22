mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(x: usize, y: usize, objects: areas_volumes::GeometricalShapes, times: usize, a: usize, b: usize ) -> bool {
    // println!("{:?}", objects);

    println!("{}", rectangle_area(a,b));
    println!("{}", square_area(a));

    let mut acc: f64 = 0.0;


    for _ in 1..= times {
        match objects {
            areas_volumes::GeometricalShapes::Rectangle => acc += rectangle_area(a,b) as f64,
            areas_volumes::GeometricalShapes::Square => acc += square_area(a) as f64,
            areas_volumes::GeometricalShapes::Triangle => acc += triangle_area(a,b) as f64,
            areas_volumes::GeometricalShapes::Circle => acc += circle_area(a) as f64,
        }
    }
    
    let area = x as f64 * y as f64;

    if acc > area {
        return false
    }
    return true
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {

    println!("{}", rectangle_area(a,b));
    println!("{}", square_area(a));

    let mut acc: f64 = 0.0;

    for _ in 1..= times {
        match objects {
            areas_volumes::GeometricalVolumes::Cube => acc += cube_volume(a) as f64,
            areas_volumes::GeometricalVolumes::Sphere => acc += sphere_volume(a) as f64,
            areas_volumes::GeometricalVolumes::Cone => acc += cone_volume(a,b) as f64,
            areas_volumes::GeometricalVolumes::Pyramid => acc += triangular_pyramid_volume(a as f64,b) as f64,
            areas_volumes::GeometricalVolumes::Parallelepiped => acc += parallelepiped_volume(a,b,c) as f64,
        }
    }
    
    let area = x as f64 * y as f64 * z as f64;

    if acc > area {
        return false
    }
    return true

}

//fn main() {
//	println!(
//		"Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
//		area_fit(2, 4, GeometricalShapes::Rectangle, 100, 2, 1)
//	);
//	println!(
//		"Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
//		area_fit(5, 5, GeometricalShapes::Triangle, 3, 5, 3)
//	);
//	println!(
//		"Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
//		volume_fit(5, 5, 5, GeometricalVolumes::Sphere, 3, 2, 0, 0)
//	);
//	println!(
//		"Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
//		volume_fit(5, 7, 5, GeometricalVolumes::Parallelepiped, 1, 6, 7, 4)
//	);
//}
