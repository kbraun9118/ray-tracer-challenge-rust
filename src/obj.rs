use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    error::RayTraceResult,
    shape::{
        group::{Group, GroupContainer},
        triangle::Triangle,
    },
    tuple::Tuple,
};

pub struct OBJParser {
    groups: HashMap<String, Vec<Triangle>>,
    default_group: GroupContainer,
}

fn fan_triangulation(verticies: Vec<Tuple>) -> Vec<Triangle> {
    let mut triangles = vec![];

    for i in 1..(verticies.len() - 1) {
        let tri = Triangle::new(verticies[0], verticies[i], verticies[i + 1]);
        triangles.push(tri);
    }
    triangles
}

impl OBJParser {
    pub fn parse_file<T: AsRef<Path>>(path: T) -> RayTraceResult<Self> {
        let file_string = fs::read_to_string(path)?;
        let mut verticies = vec![];
        // let default_group = GroupContainer::from(Group::new());
        let mut current_group: Option<String> = None;
        let mut groups: HashMap<String, Vec<Triangle>> = HashMap::new();
        let lines = file_string.lines().collect::<Vec<_>>();
        let pb = ProgressBar::new(lines.len() as u64);
        pb.set_style(ProgressStyle::with_template("{wide_bar} {percent}% {eta} {msg}").unwrap());

        for line in file_string.lines() {
            pb.inc(1);
            if line.is_empty() {
                continue;
            }
            match &line[..1] {
                "v" => {
                    let input: Vec<_> = line[2..].split_whitespace().collect();
                    let vertex =
                        Tuple::point(input[0].parse()?, input[1].parse()?, input[2].parse()?);
                    verticies.push(vertex);
                }
                "f" => {
                    let verticies: Vec<_> = line[2..]
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().expect("Cannot parse"))
                        .map(|i| verticies[i - 1])
                        .collect();
                    let mut triangles = fan_triangulation(verticies);
                    if let Some(ref current_group) = current_group {
                        groups
                            .entry(current_group.clone())
                            .and_modify(|e| e.append(&mut triangles))
                            .or_insert(triangles);
                    } else {
                        for triangle in triangles {
                            default_group.add_child(triangle.into());
                        }
                    }
                }
                "g" => {
                    current_group = Some(line[2..].to_string());
                }
                _ => {}
            }
        }
        pb.finish_with_message(format!("Finished importing"));
        Ok(Self {
            // verticies,
            groups,
            default_group,
        })
    }

    pub fn default_group(&self) -> GroupContainer {
        self.default_group.clone()
    }

    pub fn get_group(&self, key: String) -> &Vec<Triangle> {
        &self.groups[&key]
    }

    pub fn as_group(self) -> GroupContainer {
        for (_, triangles) in self.groups {
            let inner_group = GroupContainer::from(Group::new());
            for triangle in triangles {
                inner_group.add_child(triangle.into());
            }
            self.default_group.add_child(inner_group.into());
        }
        self.default_group
    }
}

#[cfg(test)]
mod tests {

    use crate::shape::Shape;

    use super::*;

    #[test]
    fn parsing_triangle_faces() {
        let parser = OBJParser::parse_file("./test/triangle_faces.obj").unwrap();
        let g = parser.default_group();
        let t1 = g.borrow().children()[0].clone();
        let t2 = g.borrow().children()[1].clone();
        let v1 = Tuple::point(-1.0, 1.0, 0.0);
        let v2 = Tuple::point(-1.0, 0.0, 0.0);
        let v3 = Tuple::point(1.0, 0.0, 0.0);
        let v4 = Tuple::point(1.0, 1.0, 0.0);
        let t1_triangle = Triangle::new(v1, v2, v3);
        let t2_triangle = Triangle::new(v1, v3, v4);
        assert_eq!(
            t1.borrow().normal_at(t1.id(), Tuple::origin()),
            t1_triangle.local_normal_at(t1_triangle.id(), Tuple::origin())
        );
        assert_eq!(
            t2.borrow().normal_at(t2.id(), Tuple::origin()),
            t2_triangle.local_normal_at(t2_triangle.id(), Tuple::origin())
        );
    }

    #[test]
    fn triangulating_polygons() {
        let parser = OBJParser::parse_file("./test/triangulating_polygons.obj").unwrap();
        let v1 = Tuple::point(-1.0, 1.0, 0.0);
        let v2 = Tuple::point(-1.0, 0.0, 0.0);
        let v3 = Tuple::point(1.0, 0.0, 0.0);
        let v4 = Tuple::point(1.0, 1.0, 0.0);
        let v5 = Tuple::point(0.0, 2.0, 0.0);
        let triangles = parser.get_group("Poly".to_string());
        let t1 = &triangles[0];
        let t2 = &triangles[1];
        let t3 = triangles[2].clone();

        assert_eq!(t1.p1(), v1);
        assert_eq!(t1.p2(), v2);
        assert_eq!(t1.p3(), v3);
        assert_eq!(t2.p1(), v1);
        assert_eq!(t2.p2(), v3);
        assert_eq!(t2.p3(), v4);
        assert_eq!(t3.p1(), v1);
        assert_eq!(t3.p2(), v4);
        assert_eq!(t3.p3(), v5);
    }
}
