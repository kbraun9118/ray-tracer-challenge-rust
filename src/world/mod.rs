use std::vec;

use crate::{
    color::{Color, Colors},
    intersection::{prepcomputation::PrepComputations, ray::Ray, IntersectionHeap},
    point_light::PointLight,
    shape::{material::Material, sphere::Sphere, Shape, ShapeContainer},
    transformation::Transformation,
    tuple::Tuple,
    util::eq_f64,
};

#[derive(Debug)]
pub struct World {
    shapes: Vec<ShapeContainer>,
    light: Option<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            light: None,
        }
    }

    pub fn shapes(&self) -> &Vec<ShapeContainer> {
        &self.shapes
    }

    pub fn add_shape(&mut self, shape: ShapeContainer) {
        self.shapes.push(shape);
    }

    pub fn shapes_mut(&mut self) -> &mut Vec<ShapeContainer> {
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

    pub fn shade_hit(&self, comps: &PrepComputations) -> Color {
        self.shade_hit_recursive(comps, 5)
    }

    pub fn shade_hit_recursive(&self, comps: &PrepComputations, remaining: usize) -> Color {
        let shadowed = self.is_shadowed(comps.over_point());

        if let Some(light) = self.light {
            let surface = comps
                .object()
                .borrow()
                .material(comps.object_id())
                .unwrap_or_default()
                .lighting(
                    comps.object().clone(),
                    light,
                    comps.over_point(),
                    comps.eye_v(),
                    comps.normal_v(),
                    shadowed,
                );

            let reflected = self.reflected_color(comps, remaining);
            let refracted = self.refracted_color(comps, remaining);

            let material = comps.object().borrow().material(comps.object_id()).unwrap();
            if material.reflective() > 0.0 && material.transparency() > 0.0 {
                let reflectance = comps.schlick();
                return surface + reflected * reflectance + refracted * (1.0 - reflectance);
            }

            surface + reflected + refracted
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
            let comps = PrepComputations::new(hit, ray.clone(), &intersections);
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

    fn reflected_color(&self, comps: &PrepComputations, remaining: usize) -> Color {
        if remaining <= 0
            || eq_f64(
                comps
                    .object()
                    .borrow()
                    .material(comps.object_id())
                    .unwrap()
                    .reflective(),
                0.0,
            )
        {
            return Colors::Black.into();
        }

        let reflect_ray = Ray::new(comps.over_point(), comps.reflect_v());
        let color = self.color_at_recursive(reflect_ray, remaining - 1);

        color
            * comps
                .object()
                .borrow()
                .material(comps.object_id())
                .unwrap()
                .reflective()
    }

    fn refracted_color(&self, comps: &PrepComputations, remaining: usize) -> Color {
        if remaining == 0
            || eq_f64(
                comps
                    .object()
                    .borrow()
                    .material(comps.object_id())
                    .unwrap()
                    .transparency(),
                0.0,
            )
        {
            return Colors::Black.into();
        }
        let n_ratio = comps.n1() / comps.n2();
        let cos_i = comps.eye_v() * comps.normal_v();
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return Colors::Black.into();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normal_v() * (n_ratio * cos_i - cos_t) - comps.eye_v() * n_ratio;
        let refract_ray = Ray::new(comps.under_point(), direction);
        self.color_at_recursive(refract_ray, remaining - 1)
            * comps
                .object()
                .borrow()
                .material(comps.object_id())
                .unwrap()
                .transparency()
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
            shapes: vec![s1.into(), s2.into()],
            light: Some(light),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        intersection::ShapeIntersection,
        intersections,
        shape::{material::pattern::TestPattern, plane::Plane},
    };

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
            .any(|i| i.borrow().transformation() == s1_transformation));
        assert!(world.shapes().iter().any(|i| i
            .borrow()
            .material(world.shapes()[0].id())
            .unwrap()
            == s2_material));
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
        let i = ShapeIntersection::new(4.0, shape.clone(), shape.id());

        let comps = PrepComputations::new(i, r, &IntersectionHeap::new());

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
        let i = ShapeIntersection::new(0.5, shape.clone(), shape.id());

        let comps = PrepComputations::new(i, r, &IntersectionHeap::new());

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
        let w = World::default();
        w.shapes()
            .get(0)
            .unwrap()
            .borrow_mut()
            .set_material(Material::default().with_ambient(1.0));
        w.shapes()
            .get(1)
            .unwrap()
            .borrow_mut()
            .set_material(Material::default().with_ambient(1.0));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);
        assert_eq!(
            c,
            w.shapes()[1]
                .clone()
                .borrow()
                .material(w.shapes()[1].id())
                .unwrap()
                .pattern()
                .color_at(Tuple::origin())
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
        w.add_shape(s1.into());

        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::identity().translation(0.0, 0.0, 10.0));
        w.add_shape(s2.into());

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

        let i = ShapeIntersection::new(4.0, w.shapes()[1].clone(), w.shapes()[1].id());

        let comps = PrepComputations::new(i, r, &IntersectionHeap::new());
        let c = w.shade_hit(&comps);

        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        w.shapes_mut()
            .get_mut(1)
            .unwrap()
            .borrow_mut()
            .set_material(Material::new().with_ambient(1.0));
        let i = ShapeIntersection::new(1.0, w.shapes()[1].clone(), w.shapes()[1].id());
        let comps = PrepComputations::new(i, r, &IntersectionHeap::new());
        let color = w.reflected_color(&comps, 5);

        assert_eq!(color, Colors::Black.into());
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.set_material(Material::new().with_reflective(0.5));
        shape.set_transformation(Transformation::identity().translation(0.0, -1.0, 0.0));
        let shape = ShapeContainer::from(shape);
        w.shapes_mut().push(shape.clone());

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = ShapeIntersection::new(2f64.sqrt(), shape.clone(), shape.id());
        let comps = PrepComputations::new(i, r, &IntersectionHeap::new());
        let color = w.reflected_color(&comps, 5);

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
        w.add_shape(lower.into());

        let mut upper = Plane::new();
        upper.set_material(Material::new().with_reflective(1.0));
        upper.set_transformation(Transformation::identity().translation(0.0, 1.0, 0.0));
        w.add_shape(upper.into());

        let r = Ray::new(Tuple::origin(), Tuple::vector(0.0, 1.0, 0.0));

        let _color = w.color_at(r);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.set_material(Material::new().with_reflective(0.5));
        shape.set_transformation(Transformation::identity().translation(0.0, -1.0, 0.0));
        let shape = ShapeContainer::from(shape);
        w.shapes_mut().push(shape.clone());

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = ShapeIntersection::new(2f64.sqrt(), shape.clone(), shape.id());
        let comps = PrepComputations::new(i, r, &IntersectionHeap::new());
        let color = w.reflected_color(&comps, 0);

        assert_eq!(color, Colors::Black.into());
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.shapes()[0].clone();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersections!(
            ShapeIntersection::new(4.0, shape.clone(), shape.id()),
            ShapeIntersection::new(6.0, shape.clone(), shape.id())
        );
        let comps = PrepComputations::new(xs[0].clone(), r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Colors::Black.into());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let shape = w.shapes_mut().get_mut(0).unwrap();

        shape.borrow_mut().set_material(
            Material::default()
                .with_transparency(1.0)
                .with_reflective(1.5),
        );

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersections!(
            ShapeIntersection::new(4.0, shape.clone(), shape.id()),
            ShapeIntersection::new(6.0, shape.clone(), shape.id())
        );
        let comps = PrepComputations::new(xs[0].clone(), r, &xs);
        let c = w.refracted_color(&comps, 0);

        assert_eq!(c, Colors::Black.into());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        let shape = w.shapes_mut().get_mut(0).unwrap();
        shape.borrow_mut().set_material(
            Material::default()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );

        let r = Ray::new(
            Tuple::point(0.0, 0.0, 2f64.sqrt() / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        let xs = intersections!(
            ShapeIntersection::new(-(2f64.sqrt()) / 2.0, shape.clone(), shape.id()),
            ShapeIntersection::new(2f64.sqrt() / 2.0, shape.clone(), shape.id())
        );
        let comps = PrepComputations::new(xs[1].clone(), r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Colors::Black.into());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let w = World::default();
        w.shapes().get(0).unwrap().borrow_mut().set_material(
            Material::new()
                .with_ambient(1.0)
                .with_pattern(TestPattern::default()),
        );
        w.shapes().get(1).unwrap().borrow_mut().set_material(
            Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));
        let a = w.shapes().get(0).unwrap();
        let b = w.shapes().get(1).unwrap();
        let xs = intersections!(
            ShapeIntersection::new(-0.9899, a.clone(), a.id()),
            ShapeIntersection::new(-0.4899, b.clone(), b.id()),
            ShapeIntersection::new(0.4899, b.clone(), b.id()),
            ShapeIntersection::new(0.9899, a.clone(), a.id())
        );

        let comps = PrepComputations::new(xs[2].clone(), r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.9988745506795582, 0.04721898034382347));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::default();
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -(2f64.sqrt()) / 2.0, 2f64.sqrt() / 2.0),
        );
        let mut floor = Plane::new();
        let floor_id = floor.id();
        floor.set_transformation(Transformation::default().translation(0.0, -1.0, 0.0));
        floor.set_material(
            Material::new()
                .with_reflective(0.5)
                .with_transparency(0.5)
                .with_refractive_index(1.5),
        );
        w.add_shape(floor.into());

        let mut ball = Sphere::new();
        ball.set_material(
            Material::new()
                .with_color(Color::new(1.0, 0.0, 0.0))
                .with_ambient(0.5),
        );
        ball.set_transformation(Transformation::default().translation(0.0, -3.5, -0.5));
        w.add_shape(ball.into());
        let xs = intersections!(ShapeIntersection::new(
            2f64.sqrt(),
            w.shapes()
                .iter()
                .find(|s| s.borrow().id() == floor_id)
                .unwrap()
                .clone(),
            floor_id
        ));
        let comps = PrepComputations::new(xs[0].clone(), r, &xs);
        let color = w.shade_hit(&comps);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
