use std::{rc::Rc, vec};

use crate::{
    color::{Color, Colors},
    intersection::{
        precomputation::PreComputations,
        ray::Ray,
        shape::{material::Material, sphere::Sphere, Shape},
        IntersectionHeap,
    },
    point_light::PointLight,
    transformation::Transformation,
    tuple::Tuple,
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
        if let Some(light) = self.light {
            comps.object().material().lighting(
                light,
                comps.point(),
                comps.eye_v(),
                comps.normal_v(),
            )
        } else {
            Colors::Black.into()
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let mut intersections = self.intersects(ray);

        if let Some(hit) = intersections.hit() {
            let comps = PreComputations::new(hit, ray.clone());
            self.shade_hit(&comps)
        } else {
            Colors::Black.into()
        }
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

        let light = PointLight::new(Tuple::point(-10.0, -10.0, -10.0), Colors::White.into());
        Self {
            shapes: vec![Rc::new(s1), Rc::new(s2)],
            light: Some(light),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::Intersection;

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

        let light = PointLight::new(Tuple::point(-10.0, -10.0, -10.0), Colors::White.into());

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

        let comps = PreComputations::new(i, r);

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

        let comps = PreComputations::new(i, r);

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
        assert_eq!(c, w.shapes()[1].material().color())
    }
}
