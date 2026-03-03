// ------------------------------------ Geometry ------------------------------------

pub trait CircleCircumference {
    fn h_circle_circumference(&self) -> f64;
}

impl<T> CircleCircumference for T
where
    T: Copy + Into<f64>,
{
    fn h_circle_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * (*self).into()
    }
}

pub trait CircleArea {
    fn h_circle_area(&self) -> f64;
}

impl<T> CircleArea for T
where
    T: Copy + Into<f64>,
{
    fn h_circle_area(&self) -> f64 {
        let r = (*self).into();
        std::f64::consts::PI * r * r
    }
}

pub fn h_pythagorean_theorem<A, B>(a: A, b: B) -> f64
where
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
{
    let a_f = a.into();
    let b_f = b.into();
    (a_f.powf(2.0) + b_f.powf(2.0)).sqrt()
}

pub fn h_reverse_pythagorean_theorem<K, H>(x: K, h: H) -> f64
where 
    K: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    let xfc: f64 = x.into().powf(2.0);
    let hfc: f64 = h.into().powf(2.0);
    (hfc - xfc).powf(0.5)
}

pub fn h_find_equal_legs_from_hypotenuse<H>(h: H) -> f64 
where 
    H: Copy + Into<f64>,
{
    let hfc: f64 = h.into().powf(2.0);
    (hfc / 2.0).sqrt()
}

pub trait ShortFromLongLeg30_60_90{
    fn h_short_from_long_leg_30_60_90(&self) -> f64;
}

impl<T> ShortFromLongLeg30_60_90 for T
where
    T: Copy + Into<f64>,
{
    fn h_short_from_long_leg_30_60_90(&self) -> f64 {
        (*self).into() / (3.0f64).sqrt()
    }
}





// ------------------------------------ 3d -------------------------------------------------


/// Calculates the volume of a sphere given its radius.
/// Formula: V = (4/3) * π * r^3
pub trait SphereVolume {
    fn h_sphere_volume(&self) -> f64;
}

impl<T> SphereVolume for T
where
    T: Copy + Into<f64>,
{
    fn h_sphere_volume(&self) -> f64 {
        let r: f64 = (*self).into();
        (4.0 / 3.0) * std::f64::consts::PI * r * r * r
    }
}

/// Calculates the surface area of a sphere given its radius.
/// Formula: A = 4 * π * r^2
pub trait SphereSurfaceArea {
    fn h_sphere_surface_area(&self) -> f64;
}

impl<T> SphereSurfaceArea for T
where
    T: Copy + Into<f64>,
{
    fn h_sphere_surface_area(&self) -> f64 {
        let r: f64 = (*self).into();
        4.0 * std::f64::consts::PI * r * r
    }
}

/// Calculates the volume of a cylinder given its radius and height.
/// Formula: V = π * r^2 * h
pub trait CylinderVolume<H> {
    fn h_cylinder_volume(&self, height: H) -> f64;
}

impl<R, H> CylinderVolume<H> for R
where
    R: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_cylinder_volume(&self, height: H) -> f64 {
        let r: f64 = (*self).into();
        let h: f64 = height.into();
        std::f64::consts::PI * r * r * h
    }
}

/// Calculates the surface area of a cylinder given its radius and height.
/// Formula: A = 2 * π * r^2 + 2 * π * r * h
pub trait CylinderSurfaceArea<H> {
    fn h_cylinder_surface_area(&self, height: H) -> f64;
}

impl<R, H> CylinderSurfaceArea<H> for R
where
    R: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_cylinder_surface_area(&self, height: H) -> f64 {
        let r: f64 = (*self).into();
        let h: f64 = height.into();
        2.0 * std::f64::consts::PI * r * r + 2.0 * std::f64::consts::PI * r * h
    }
}

/// Calculates the volume of a cube given the length of its side.
/// Formula: V = s^3
pub trait CubeVolume {
    fn h_cube_volume(&self) -> f64;
}

impl<S> CubeVolume for S
where
    S: Copy + Into<f64>,
{
    fn h_cube_volume(&self) -> f64 {
        let side: f64 = (*self).into();
        side * side * side
    }
}


/// Calculates the surface area of a cube given the length of its side.
/// Formula: A = 6 * s^2
pub trait CubeSurfaceArea {
    fn h_cube_surface_area(&self) -> f64;
}

impl<S> CubeSurfaceArea for S
where
    S: Copy + Into<f64>,
{
    fn h_cube_surface_area(&self) -> f64 {
        let side: f64 = (*self).into();
        6.0 * side * side
    }
}


/// Calculates the volume of a cone given its radius and height.
/// Formula: V = (1/3) * π * r^2 * h
pub trait ConeVolume<H> {
    fn h_cone_volume(&self, height: H) -> f64;
}

impl<R, H> ConeVolume<H> for R
where
    R: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_cone_volume(&self, height: H) -> f64 {
        let r: f64 = (*self).into();
        let h: f64 = height.into();
        (1.0 / 3.0) * std::f64::consts::PI * r * r * h
    }
}


/// Calculates the surface area of a cone given its radius and height.
/// Formula: A = π * r * (r + √(r^2 + h^2))
pub trait ConeSurfaceArea<H> {
    fn h_cone_surface_area(&self, height: H) -> f64;
}

impl<R, H> ConeSurfaceArea<H> for R
where
    R: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_cone_surface_area(&self, height: H) -> f64 {
        let r: f64 = (*self).into();
        let h: f64 = height.into();
        let slant_height: f64 = (r * r + h * h).sqrt();
        std::f64::consts::PI * r * (r + slant_height)
    }
}

/// Calculates the volume of a rectangular prism given its length, width, and height.
/// Formula: V = l * w * h
pub trait RectangularPrismVolume<H> {
    fn h_rectangular_prism_volume(&self, height: H) -> f64;
}

impl<L, H> RectangularPrismVolume<H> for L
where
    L: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_rectangular_prism_volume(&self, height: H) -> f64 {
        let length: f64 = (*self).into();
        let width: f64 = length;
        let height: f64 = height.into();
        length * width * height
    }
}

/// Calculates the surface area of a rectangular prism given its length, width, and height.
/// Formula: A = 2 * (l * w + l * h + w * h)
pub trait RectangularPrismSurfaceArea<H> {
    fn h_rectangular_prism_surface_area(&self, height: H) -> f64;
}

impl<L, H> RectangularPrismSurfaceArea<H> for L
where
    L: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_rectangular_prism_surface_area(&self, height: H) -> f64 {
        let length: f64 = (*self).into();
        let width: f64 = length;
        let height: f64 = height.into();
        2.0 * (length * width + length * height + width * height)
    }
}

/// Calculates the volume of a pyramid given its base area and height.
/// Formula: V = (1/3) * base_area * height
pub trait PyramidVolume<H> {
    fn h_pyramid_volume(&self, height: H) -> f64;
}

impl<B, H> PyramidVolume<H> for B
where
    B: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_pyramid_volume(&self, height: H) -> f64 {
        let base_area: f64 = (*self).into();
        let height: f64 = height.into();
        (1.0 / 3.0) * base_area * height
    }
}

/// Calculates the surface area of a square pyramid given the length of its base and height.
/// Formula: A = base_area + 2 * (base_area / 4)^0
pub trait SquarePyramidSurfaceArea<B, H> {
    fn h_square_pyramid_surface_area(&self, height: H) -> f64;
}

impl<B, H> SquarePyramidSurfaceArea<B, H> for B
where
    B: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    fn h_square_pyramid_surface_area(&self, height: H) -> f64 {
        let base_area: f64 = (*self).into();
        let height: f64 = height.into();
        let slant_height: f64 = (height * height + (base_area / 4.0).sqrt() * (base_area / 4.0).sqrt()).sqrt();
        base_area + 2.0 * (base_area / 4.0).sqrt() * slant_height
    }
}



