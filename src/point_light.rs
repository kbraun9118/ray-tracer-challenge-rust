use crate::{color::Color, tuple::Tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> Tuple {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Colors;

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intisity() {
        let intensity = Colors::White.into();
        let position = Tuple::origin();

        let light = PointLight::new(position, intensity);

        assert_eq!(position, light.position());
        assert_eq!(intensity, light.intensity());
    }
}
