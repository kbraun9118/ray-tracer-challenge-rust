use std::{rc::Rc, vec};

use crate::{
    color::{Color, Colors},
    intersection::{precomputation::PreComputations, ray::Ray, IntersectionHeap},
    point_light::PointLight,
    shape::{material::Material, sphere::Sphere, Shape},
    transformation::Transformation,
    tuple::Tuple,
    util::eq_f64,
};

#[derive(Debug)]
pub struct World {
    shapes: Vec<Rc<dyn Shape>>,
    light: Option<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            light: None,
        }
    }

    pub fn shapes(&self) -> &Vec<Rc<dyn Shape>> {
        &self.shapes
    }

    pub fn add_shape<T: Shape + 'static>(&mut self, shape: T) {
        self.shapes.push(Rc::new(shape));
    }

    pub fn shapes_mut(&mut self) -> &mut Vec<Rc<dyn Shape>> {
        &mut self.shapes
    }

    pub fn light(&self) -> &Option<PointLight> {
        &self.light
    }

    pub fn set_light(&mut self, point_light: PointLight) -> &Self {
        self.light = Some(point_light);
        self
    }

    pub fn intersects(&self, r: Ray) -> IntersectionHeap {
        let mut heap = IntersectionHeap::new();

        for s in self.shapes() {
            let intersections = r.intersections(s.clone());
            for i in intersections {
                heap.push(i);
            }
        }

        heap
    }

    pub fn shade_hit(&self, comps: &PreComputations) -> Color {
        self.shade_hit_recursive(comps, 5)
    }

    pub fn shade_hit_recursive(&self, comps: &PreComputations, remaining: usize) -> Color {
        let shadowed = self.is_shadowed(comps.over_point());

        if let Some(light) = self.light {
            let surface = comps.object().material().lighting(
                comps.object().as_ref(),
                light,
                comps.over_point(),
                comps.eye_v(),
                comps.normal_v(),
                shadowed,
            );

            let reflected = self.reflected_color_recursive(comps, remaining);

            surface + reflected
        } else {
            Colors::Black.into()
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        self.color_at_recursive(ray, 5)
    }

    pub fn color_at_recursive(&self, ray: Ray, remaining: usize) -> Color {
        let intersections = self.intersects(ray);

        if let Some(hit) = intersections.hit() {
            let comps = PreComputations::new(hit, ray.clone(), &intersections);
            self.shade_hit_recursive(&comps, remaining)
        } else {
            Colors::Black.into()
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        if let Some(l) = self.light {
            let v = l.position() - point;

            let distance = v.magnitude();
            let direction = v.normalize();

            let r = Ray::new(point, direction);

            let h = self.intersects(r).hit();

            match h {
                Some(h) if h.t() < distance => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn reflected_color(&self, comps: &PreComputations) -> Color {
        self.reflected_color_recursive(comps, 5)
    }

    fn reflected_color_recursive(&self, comps: &PreComputations, remaining: usize) -> Color {
        if remaining <= 0 || eq_f64(comps.object().material().reflective(), 0.0) {
            return Colors::Black.into();
        }

        let reflect_ray = Ray::new(comps.over_point(), comps.reflect_v());
        let color = self.color_at_recursive(reflect_ray, remaining - 1);

        color * comps.object().material().reflective()
    }

    fn refracted_color(&self, comps: PreComputations, arg: f64) -> Color {
        if eq_f64(comps.object().material().transparency(), 0.0);
        0.0 == 1.0
    }
}

impl Default for World {
    fn default() -> Self {
        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::identity().scale(0.5, 0.5, 0.5));

        let mut s1 = Sphere::new();

        s1.set_material(
            Material::new()
                .with_color(Color::new(0.8, 1.0, 0.6))
                .with_diffuse(0.7)
                .with_specular(0.2),
        );

        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Colors::White.into());
        Self {
            shapes: vec![Rc::new(s1), Rc::new(s2)],
            light: Some(light),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersection::Intersection, shape::plane::Plane, intersections};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert_eq!(0, w.shapes().len());
        assert_eq!(&None, w.light());
    }

    #[test]
    fn the_default_world() {
        let s1_transformation = Transformation::identity().scale(0.5, 0.5, 0.5);

        let s2_material = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);

        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Colors::White.into());

        let world = World::default();

        assert!(world.light.is_some());

        assert_eq!(light, world.light().unwrap());
        assert!(world
            .shapes()
            .iter()
            .any(|i| i.transformation() == s1_transformation));
        assert!(world.shapes().iter().any(|i| i.material() == s2_material));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersects(r);

        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t());
        assert_eq!(4.5, xs[1].t());
        assert_eq!(5.5, xs[2].t());
        assert_eq!(6.0, xs[3].t());
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.shapes()[0].clone();
        let i = Intersection::new(4.0, shape);

        let comps = PreComputations::new(i, r, &IntersectionHeap::new());

        let c = w.shade_hit(&comps);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(
            Tuple::point(0.0, 0.25, 0.0),
            Colors::White.into(),
        ));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.shapes()[1].clone();
        let i = Intersection::new(0.5, shape);

        let comps = PreComputations::new(i, r, &IntersectionHeap::new());

        let c = w.shade_hit(&comps);

        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);

        assert_eq!(Color::from(Colors::Black), c);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        std::rc::Rc::<_>::get_mut(&mut w.shapes_mut().get_mut(0).unwrap())
            .unwrap()
            .set_material(Material::default().with_ambient(1.0));
        std::rc::Rc::<_>::get_mut(&mut w.shapes_mut().get_mut(1).unwrap())
            .unwrap()
            .set_material(Material::default().with_ambient(1.0));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);
        assert_eq!(
            c,
            w.shapes()[1].material().pattern().color_at(Tuple::origin())
        )
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Tuple::point(0.0, 0.0, -10.0),
            Colors::White.into(),
        ));

        let s1 = Sphere::new();
        w.add_shape(s1);

        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::identity().translation(0.0, 0.0, 10.0));
        w.add_shape(s2);

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

        let i = Intersection::new(4.0, w.shapes()[1].clone());

        let comps = PreComputations::new(i, r, &IntersectionHeap::new());
        let c = w.shade_hit(&comps);

        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        Rc::get_mut(w.shapes_mut().get_mut(1).unwrap())
            .unwrap()
            .set_material(Material::new().with_ambient(1.0));
        let i = Intersection::new(1.0, w.shapes()[1].clone());
        let comps = PreComputations::new(i, r, &IntersectionHeap::new());
        let color = w.reflected_color(&comps);

        assert_eq!(color, Colors::Black.into());
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.set_material(Material::new().with_reflective(0.5));
        shape.set_transformation(Transformation::identity().translation(0.0, -1.0, 0.0));
        let shape = Rc::new(shape);
        w.shapes_mut().push(shape.clone());

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), shape);
        let comps = PreComputations::new(i, r, &IntersectionHeap::new());
        let color = w.reflected_color(&comps);

        assert_eq!(Color::new(0.19033, 0.23791, 0.14274), color);

        let color = w.shade_hit(&comps);

        assert_eq!(Color::new(0.87675, 0.92434, 0.82918), color)
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.set_light(PointLight::new(Tuple::origin(), Colors::White.into()));

        let mut lower = Plane::new();
        lower.set_material(Material::new().with_reflective(1.0));
        lower.set_transformation(Transformation::identity().translation(0.0, -1.0, 0.0));
        w.add_shape(lower);

        let mut upper = Plane::new();
        upper.set_material(Material::new().with_reflective(1.0));
        upper.set_transformation(Transformation::identity().translation(0.0, 1.0, 0.0));
        w.add_shape(upper);

        let r = Ray::new(Tuple::origin(), Tuple::vector(0.0, 1.0, 0.0));

        let _color = w.color_at(r);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.set_material(Material::new().with_reflective(0.5));
        shape.set_transformation(Transformation::identity().translation(0.0, -1.0, 0.0));
        let shape = Rc::new(shape);
        w.shapes_mut().push(shape.clone());

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), shape);
        let comps = PreComputations::new(i, r, &IntersectionHeap::new());
        let color = w.reflected_color_recursive(&comps, 0);

        assert_eq!(color, Colors::Black.into());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.shapes()[0].clone();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersections!(Intersection::new(4.0, shape.clone()), Intersection::new(6.0, shape.clone()));
        let comps = PreComputations::new(xs[0].clone(), r, &xs);
        let c = w.refracted_color(comps, 5.0);

        assert_eq!(c, Colors::Black.into());
    }
}
