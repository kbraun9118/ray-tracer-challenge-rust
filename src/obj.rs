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
        smooth_triangle::SmoothTriangle,
        triangle::Triangle,
        ShapeContainer,
    },
    tuple::Tuple,
};

pub struct OBJParser {
    groups: HashMap<String, Vec<ShapeContainer>>,
    default_group: GroupContainer,
}

fn fan_triangulation(verticies: Vec<Tuple>, normals: Vec<Tuple>) -> Vec<ShapeContainer> {
    let mut triangles = vec![];

    if normals.is_empty() {
        for i in 1..(verticies.len() - 1) {
            let tri = Triangle::new(verticies[0], verticies[i], verticies[i + 1]);
            triangles.push(tri.into());
        }
    } else {
        for i in 1..(verticies.len() - 1) {
            let tri = SmoothTriangle::new(
                verticies[0],
                verticies[i],
                verticies[i + 1],
                normals[0],
                normals[i],
                normals[i + 1],
            );
            triangles.push(tri.into());
        }
    }
    triangles
}

impl OBJParser {
    pub fn parse_file<T: AsRef<Path> + Clone>(path: T) -> RayTraceResult<Self> {
        let file_string = fs::read_to_string(path.clone())?;
        let mut verticies = vec![];
        let mut normals = vec![];
        let default_group = GroupContainer::from(Group::new());
        let mut current_group: Option<String> = None;
        let mut groups: HashMap<String, Vec<ShapeContainer>> = HashMap::new();
        let lines = file_string.lines().collect::<Vec<_>>();
        let pb = ProgressBar::new(lines.len() as u64);
        pb.set_style(ProgressStyle::with_template("{wide_bar} {percent}% {eta} {msg}").unwrap());

        for line in file_string.lines() {
            pb.inc(1);
            if line.len() < 2 {
                continue;
            }
            match &line[..2] {
                "v " => {
                    let input: Vec<_> = line[2..].split_whitespace().collect();
                    let vertex =
                        Tuple::point(input[0].parse()?, input[1].parse()?, input[2].parse()?);
                    verticies.push(vertex);
                }
                "vn" => {
                    let input: Vec<_> = line[3..].split_whitespace().collect();
                    let vertex =
                        Tuple::vector(input[0].parse()?, input[1].parse()?, input[2].parse()?);
                    normals.push(vertex);
                }
                "f " => {
                    let mut triangles = if line.contains("/") {
                        let (verticies, normals) = line[2..]
                            .split_whitespace()
                            .map(|l| {
                                l.split("/")
                                    .map(|s| s.parse::<usize>().unwrap_or_default())
                                    .collect::<Vec<_>>()
                            })
                            .map(|i| (verticies[i[0] - 1], normals[i[2] - 1]))
                            .unzip();

                        fan_triangulation(verticies, normals)
                    } else {
                        let verticies: Vec<_> = line[2..]
                            .split_whitespace()
                            .map(|l| l.parse::<usize>().unwrap_or_default())
                            .map(|i| verticies[i - 1])
                            .collect();
                        fan_triangulation(verticies, vec![])
                    };
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
                "g " => {
                    current_group = Some(line[2..].to_string());
                }
                _ => {}
            }
        }
        pb.finish_with_message(format!(
            "Finished importing {}",
            path.as_ref().to_string_lossy()
        ));
        Ok(Self {
            // verticies,
            groups,
            default_group,
        })
    }

    pub fn default_group(&self) -> GroupContainer {
        self.default_group.clone()
    }

    pub fn get_group(&self, key: String) -> &Vec<ShapeContainer> {
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

    // Testing this is quite difficult

    // use crate::shape::Shape;
    //
    // use super::*;

    // #[test]
    // fn parsing_triangle_faces() {
    //     let parser = OBJParser::parse_file("./test/triangle_faces.obj").unwrap();
    //     let g = parser.default_group();
    //     let t1 = g.borrow().children()[0].clone();
    //     let t2 = g.borrow().children()[1].clone();
    //     let v1 = Tuple::point(-1.0, 1.0, 0.0);
    //     let v2 = Tuple::point(-1.0, 0.0, 0.0);
    //     let v3 = Tuple::point(1.0, 0.0, 0.0);
    //     let v4 = Tuple::point(1.0, 1.0, 0.0);
    //     let t1_triangle = Triangle::new(v1, v2, v3);
    //     let t2_triangle = Triangle::new(v1, v3, v4);
    //     assert_eq!(
    //         t1.borrow().normal_at(t1.id(), Tuple::origin()),
    //         t1_triangle.local_normal_at(t1_triangle.id(), Tuple::origin())
    //     );
    //     assert_eq!(
    //         t2.borrow().normal_at(t2.id(), Tuple::origin()),
    //         t2_triangle.local_normal_at(t2_triangle.id(), Tuple::origin())
    //     );
    // }
    //
    // #[test]
    // fn triangulating_polygons() {
    //     let parser = OBJParser::parse_file("./test/triangulating_polygons.obj").unwrap();
    //     let v1 = Tuple::point(-1.0, 1.0, 0.0);
    //     let v2 = Tuple::point(-1.0, 0.0, 0.0);
    //     let v3 = Tuple::point(1.0, 0.0, 0.0);
    //     let v4 = Tuple::point(1.0, 1.0, 0.0);
    //     let v5 = Tuple::point(0.0, 2.0, 0.0);
    //     let triangles = parser.get_group("Poly".to_string());
    //     let t1 = &triangles[0];
    //     let t2 = &triangles[1];
    //     let t3 = triangles[2].clone();
    //
    //     assert_eq!(t1.p1(), v1);
    //     assert_eq!(t1.p2(), v2);
    //     assert_eq!(t1.p3(), v3);
    //     assert_eq!(t2.p1(), v1);
    //     assert_eq!(t2.p2(), v3);
    //     assert_eq!(t2.p3(), v4);
    //     assert_eq!(t3.p1(), v1);
    //     assert_eq!(t3.p2(), v4);
    //     assert_eq!(t3.p3(), v5);
    // }
}
