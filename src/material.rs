use crate::canvas::Color;
use crate::F;
use crate::fuzzy_eq::*;

pub trait Illuminated {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Phong(Phong),
}

impl From<Phong> for Material {
    fn from(phong: Phong) -> Self {
        Material::Phong(phong)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::from(Phong::default())
    }
}

impl FuzzyEq<Material> for Material {
  fn fuzzy_eq(&self, other: Material) -> bool {
    match (self, other) {
      (Material::Phong(ref phong), Material::Phong(other)) => phong.fuzzy_eq(other),
      // Add default case (different types) to return false, once more than one
      // Material exists
      // _ => false,
    }
  }
}

impl Illuminated for Material {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phong {
    pub color: Color,
    pub ambient: F,
    pub diffuse: F,
    pub specular: F,
    pub shininess: F,
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Phong {
    pub fn new(color: Color, ambient: F, diffuse: F, specular: F, shininess: F) -> Self {
        Phong {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl FuzzyEq<Phong> for Phong {
    fn fuzzy_eq(&self, other: Phong) -> bool {
        self.color.fuzzy_eq(other.color) 
            && self.ambient.fuzzy_eq(other.ambient) 
            && self.diffuse.fuzzy_eq(other.diffuse) 
            && self.specular.fuzzy_eq(other.specular) 
            && self.shininess.fuzzy_eq(other.shininess)
    }
}

impl Illuminated for Phong {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_phong_material() {
        let materail = Phong::default();

        assert_fuzzy_eq!(materail.color, Color::new(1.0, 1.0, 1.0));
        assert_fuzzy_eq!(materail.ambient, 0.1);
        assert_fuzzy_eq!(materail.diffuse, 0.9);
        assert_fuzzy_eq!(materail.specular, 0.9);
        assert_fuzzy_eq!(materail.shininess, 200.0);
    }

    #[test]
    fn phong_material_can_be_constructed_with_properties() {
        let color = Color::new(1.0, 1.0, 1.0);
        let ambient = 0.05;
        let diffuse = 0.7;
        let specular = 0.9;
        let shininess = 400.0;

        let material = Phong::new(color, ambient, diffuse, specular, shininess);

        assert_fuzzy_eq!(material.color, color);
        assert_fuzzy_eq!(material.ambient, ambient);
        assert_fuzzy_eq!(material.diffuse, diffuse);
        assert_fuzzy_eq!(material.specular, specular);
        assert_fuzzy_eq!(material.shininess, shininess);
    }
}
