use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ObjData {
    pub vertices: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<Face>,
    pub materials: HashMap<String, Material>,
}

#[derive(Debug)]
pub struct Face {
    pub vertex_indices: [u32; 3],
    pub material_name: Option<String>,
}

#[derive(Debug)]
pub struct Material {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
}

impl ObjData {
    pub fn parse(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut vertices = Vec::new();
        let mut tex_coords = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();
        let mut materials = HashMap::new();
        let mut current_material = None;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["v", x, y, z] => {
                    vertices.push([x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]);
                }
                ["vt", u, v] => {
                    tex_coords.push([u.parse().unwrap(), v.parse().unwrap()]);
                }
                ["vn", x, y, z] => {
                    normals.push([x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]);
                }
                ["f", v1, v2, v3] => {
                    faces.push(Face {
                        vertex_indices: [
                            v1.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v2.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v3.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                        ],
                        material_name: current_material.clone(),
                    });
                }
				["f", v1, v2, v3, v4] => {
                    faces.push(Face {
                        vertex_indices: [
                            v1.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v2.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v3.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                        ],
                        material_name: current_material.clone(),
                    });
					faces.push(Face {
                        vertex_indices: [
                            v1.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v3.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v4.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                        ],
                        material_name: current_material.clone(),
                    });
                }
                ["usemtl", material_name] => {
                    current_material = Some(material_name.to_string());
                }
                ["mtllib", mtl_file] => {
                    let mtl_path = format!("{}/{}", std::path::Path::new(file_path).parent().unwrap().display(), mtl_file);
                    materials = Self::parse_mtl(&mtl_path)?;
                }
                _ => {}
            }
        }

        Ok(ObjData {
            vertices,
            tex_coords,
            normals,
            faces,
            materials,
        })
    }

    fn parse_mtl(file_path: &str) -> io::Result<HashMap<String, Material>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut materials = HashMap::new();
        let mut current_material = None;
        let mut ambient = [0.0; 3];
        let mut diffuse = [0.0; 3];
        let mut specular = [0.0; 3];
        let mut shininess = 0.0;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["newmtl", name] => {
                    if let Some(mat_name) = current_material.take() {
                        materials.insert(mat_name, Material { ambient, diffuse, specular, shininess });
                    }
                    current_material = Some(name.to_string());
                }
                ["Ka", r, g, b] => {
                    ambient = [r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap()];
                }
                ["Kd", r, g, b] => {
                    diffuse = [r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap()];
                }
                ["Ks", r, g, b] => {
                    specular = [r.parse().unwrap(), g.parse().unwrap(), b.parse().unwrap()];
                }
                ["Ns", value] => {
                    shininess = value.parse().unwrap();
                }
                _ => {}
            }
        }

        if let Some(mat_name) = current_material {
            materials.insert(mat_name, Material { ambient, diffuse, specular, shininess });
        }

        Ok(materials)
    }
}
