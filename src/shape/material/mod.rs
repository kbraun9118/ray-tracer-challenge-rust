use std::rc::Rc;

use crate::{
    color::{Color, Colors},
    point_light::PointLight,
    tuple::Tuple,
    util::eq_f64,
};

use self::pattern::{solid::SolidPattern, Pattern};

use super::Shape;

pub mod pattern;

#[derive(Debug, Clone)]
pub struct Material {
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    pattern: Rc<dyn Pattern>,
}

impl Material {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn reflective(&self) -> f64 {
        self.reflective
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.pattern = Rc::new(SolidPattern::new(color));
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn with_pattern<T: Pattern + 'static>(mut self, pattern: T) -> Self {
        self.pattern = Rc::new(pattern);
        self
    }

    /**
       Combine the surface color with the light's color / intensity.

       Find the direction to the light source.

       Compute the ambient contribution.

       light_dot_normal represents the cosine of the angle between the
       light vector and the normal vector. A negative number means the
       light is on the same side of the surface.

       Compute the diffuse contribution.

       reflect_dot_eye represents the cosine of the angle between the
       reflection vector and the eye vector. A negative number means the
       light reflects away from the eye.

       Compute the specular contribution.

       Add the three contributions together to get the final shading.
    */
    pub fn lighting(
        &self,
        shape: &dyn Shape,
        light: PointLight,
        point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.pattern().color_at_object(shape, point) * light.intensity();

        let light_v = (light.position() - point).normalize();

        let ambient = effective_color * self.ambient();

        if in_shadow {
            return ambient;
        }

        let light_dot_normal = light_v * normal_v;

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Colors::Black.into(), Colors::Black.into())
        } else {
            let diffuse = effective_color * self.diffuse() * light_dot_normal;

            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v * eye_v;

            if eq_f64(0.0, reflect_dot_eye) || reflect_dot_eye < 0.0 {
                (diffuse, Colors::Black.into())
            } else {
                let factor = reflect_dot_eye.powf(self.shininess());
                (diffuse, light.intensity() * self.specular() * factor)
            }
        };

        return ambient + diffuse + specular;
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Rc::new(SolidPattern::new(Colors::White.into())),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        eq_f64(self.ambient, other.ambient)
            && eq_f64(self.diffuse, other.diffuse)
            && eq_f64(self.specular, other.specular)
            && eq_f64(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::sphere::Sphere;

    use super::{pattern::stripes::StripePattern, *};

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(
            Color::from(Colors::White),
            m.pattern().color_at(Tuple::origin())
        );
        assert_eq!(0.1, m.ambient());
        assert_eq!(0.9, m.diffuse());
        assert_eq!(0.9, m.specular());
        assert_eq!(200.0, m.shininess());
        assert_eq!(0.0, m.reflective());
        assert_eq!(0.0, m.transparency());
        assert_eq!(1.0, m.refractive_index());
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::origin();
        let sphere = Sphere::new();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Colors::White.into());

        let result = m.lighting(&sphere, light, position, eye_v, normal_v, false);

        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Tuple::origin();
        let sphere = Sphere::new();

        let eye_v = Tuple::vector(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Colors::White.into());

        let result = m.lighting(&sphere, light, position, eye_v, normal_v, false);

        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::origin();
        let sphere = Sphere::new();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Colors::White.into());

        let result = m.lighting(&sphere, light, position, eye_v, normal_v, false);

        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::origin();
        let sphere = Sphere::new();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Colors::White.into());

        let result = m.lighting(&sphere, light, position, eye_v, normal_v, false);

        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_the_surface_with_materials() {
        let m = Material::new();
        let position = Tuple::origin();
        let sphere = Sphere::new();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Colors::White.into());

        let in_shadow = true;

        let result = m.lighting(&sphere, light, position, eye_v, normal_v, in_shadow);

        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let sphere = Sphere::new();
        let material = Material::new()
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0)
            .with_pattern(StripePattern::new(Colors::White.into(), Colors::Black.into()));
        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Colors::White.into());
        let c1 = material.lighting(&sphere, light, Tuple::point(0.9, 0.0, 0.0), eye_v, normal_v, false);
        let c2 = material.lighting(&sphere, light, Tuple::point(1.0, 0.0, 0.0), eye_v, normal_v, false);

        assert_eq!(c1, Colors::White.into());
        assert_eq!(c2, Colors::Black.into());
    }
}
