use crate::vec3::Vec3f;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Face {
    pub verts: [usize; 3],
    tex_coords: [usize; 3],
    normals: [usize; 3]
}

pub struct Object {
    pub verticies: Vec<Vec3f>,
    pub faces: Vec<Face>
}

impl Object {
    pub fn new<P>(filename: P) -> Object 
    where P: AsRef<Path>, {
        let mut obj = Object { verticies: vec![Vec3f::zero()], faces: vec![]};
        let mut faces: Vec<Face> = vec![];
        
        // TODO: Error checking
        if let Ok(lines) = Object::read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    match ip.get(..2) {
                        Some("v ") => {
                            let line_split: Vec<&str> = ip.split_whitespace().collect();
                            obj.verticies.push(Vec3f::new(line_split[1].parse().unwrap(),
                                                          line_split[2].parse().unwrap(),
                                                          line_split[3].parse().unwrap()));
                        },
                        Some("f ") => {
                            let mut line_split: Vec<&str> = ip.split_whitespace().collect();
                            line_split.drain(0..1);
                            //println!("{:?}", line_split);
                            let mut face_indicies: [usize; 3] = [0, 0, 0];
                            let mut i = 0;

                            for group in line_split {
                                let group_split: Vec<&str> = group.split("/").collect();
                                //println!("{}", group_split[0]);
                                face_indicies[i] = group_split[0].parse().unwrap();
                                i += 1;
                            }

                            faces.push(Face {
                                verts: face_indicies,
                                tex_coords: [0, 0, 0],
                                normals: [0, 0, 0]
                            });
                        },
                        Some(_)   => (),
                        None      => (),
                    };
                }
            }
        }
        
        obj.faces = faces;
        obj
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn get_face_verts(&self, face_idx: usize) -> Vec<Vec3f> {
        //println!("face_idx {} faces.len() {}", face_idx, self.faces.len());
        vec![self.verticies[self.faces[face_idx].verts[0]],
             self.verticies[self.faces[face_idx].verts[1]],
             self.verticies[self.faces[face_idx].verts[2]]]
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }
}