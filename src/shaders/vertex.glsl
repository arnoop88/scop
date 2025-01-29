#version 330 core

layout(location = 0) in vec3 aPos;       // Position
layout(location = 1) in vec2 aTexCoord;  // Texture coordinates
layout(location = 2) in vec3 aNormal;    // Normal

uniform mat4 model;      // Will only contain rotation
uniform float posOffset;  // Camera distance
uniform vec3 modelCenter;
uniform vec3 modelPosition;

out vec3 FragPos;     // Pass world position to fragment shader
out vec3 Normal;      // Pass transformed normal to fragment shader
out vec2 TexCoord;    // Add this

void main() {
    // Center the model
    vec3 centered = aPos - modelCenter;
    
    // Apply rotation
    vec4 rotated = model * vec4(centered, 1.0);
    
    // Move back and apply translation
    vec3 finalPos = rotated.xyz + modelCenter + modelPosition;
    
    // Apply camera offset
    finalPos.z -= posOffset;
    
    // Adjusted perspective parameters
    float fov = radians(60.0);  // Wider field of view
    float aspect = 1024.0/768.0;
    float near = 0.1;
    float far = 1000.0;  // Increased far plane
    
    // Calculate perspective matrix components
    float scale = tan(fov * 0.5);
    float r = aspect * scale;
    float l = -r;
    float t = scale;
    float b = -t;
    
    // Create perspective matrix with adjusted values
    mat4 perspective = mat4(
        2.0 / (r - l), 0.0, 0.0, 0.0,
        0.0, 2.0 / (t - b), 0.0, 0.0,
        0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
        0.0, 0.0, 0.0, 1.0
    );
    
    // Apply perspective projection
    vec4 clipPos = perspective * vec4(finalPos, 1.0);
    
    // Adjust final position
    gl_Position = clipPos;
    
    // Pass values to fragment shader
    FragPos = finalPos;
    Normal = aNormal;
    TexCoord = aTexCoord;
}
