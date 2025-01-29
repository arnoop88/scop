use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct ObjData {
    pub vertices: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<Face>,
}

#[derive(Debug)]
pub struct Face {
    pub vertex_indices: [u32; 3],
}

impl ObjData {
    fn calculate_face_normals(vertices: &[[f32; 3]], faces: &[Face]) -> Vec<[f32; 3]> {
        let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];
        let mut normal_counts = vec![0; vertices.len()];

        for face in faces {
            let v0 = vertices[face.vertex_indices[0] as usize];
            let v1 = vertices[face.vertex_indices[1] as usize];
            let v2 = vertices[face.vertex_indices[2] as usize];

            // Calculate face normal using cross product
            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            // Cross product
            let normal = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            // Normalize
            let length =
                (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            let normalized = if length > 0.0 {
                [normal[0] / length, normal[1] / length, normal[2] / length]
            } else {
                // Fallback for degenerate triangles
                [0.0, 1.0, 0.0]
            };

            // Accumulate normals for each vertex
            for &vertex_idx in &face.vertex_indices {
                let idx = vertex_idx as usize;
                normals[idx][0] += normalized[0];
                normals[idx][1] += normalized[1];
                normals[idx][2] += normalized[2];
                normal_counts[idx] += 1;
            }
        }

        // Average and renormalize the normals
        for i in 0..normals.len() {
            if normal_counts[i] > 0 {
                let count = normal_counts[i] as f32;
                normals[i][0] /= count;
                normals[i][1] /= count;
                normals[i][2] /= count;

                // Renormalize
                let length = (normals[i][0] * normals[i][0]
                    + normals[i][1] * normals[i][1]
                    + normals[i][2] * normals[i][2])
                    .sqrt();
                if length > 0.0 {
                    normals[i][0] /= length;
                    normals[i][1] /= length;
                    normals[i][2] /= length;
                }
            }
        }

        normals
    }

    pub fn parse(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut vertices = Vec::new();
        let mut tex_coords = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();

        // Generate default texture coordinates based on vertex position
        fn generate_tex_coords(vertex: &[f32; 3]) -> [f32; 2] {
            [
                (vertex[0] + 1.0) * 0.5, // U coordinate
                (vertex[1] + 1.0) * 0.5, // V coordinate
            ]
        }

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["v", x, y, z] => {
                    let vertex = [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()];
                    vertices.push(vertex);
                    // Generate texture coordinates for each vertex
                    tex_coords.push(generate_tex_coords(&vertex));
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
                    });
                }
                ["f", v1, v2, v3, v4] => {
                    // First triangle
                    faces.push(Face {
                        vertex_indices: [
                            v1.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v2.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v3.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                        ],
                    });
                    // Second triangle
                    faces.push(Face {
                        vertex_indices: [
                            v1.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v3.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                            v4.split('/').next().unwrap().parse::<u32>().unwrap() - 1,
                        ],
                    });
                }
                _ => {}
            }
        }

        // If no normals were provided in the file, calculate them
        if normals.is_empty() {
            normals = Self::calculate_face_normals(&vertices, &faces);
        }

        Ok(ObjData {
            vertices,
            tex_coords,
            normals,
            faces,
        })
    }
}
