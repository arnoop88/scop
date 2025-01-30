#version 330 core

layout(location = 0) in vec3 aPos;       // Position
layout(location = 1) in vec2 aTexCoord;  // Texture coordinates
layout(location = 2) in vec3 aNormal;    // Normal

uniform mat4 projection;
uniform mat4 model;
uniform float posOffset;
uniform vec3 modelCenter;
uniform vec3 modelPosition;

out vec3 FragPos;     // Pass world position to fragment shader
out vec3 Normal;      // Pass transformed normal to fragment shader
out vec2 TexCoord;

void main() {
    // Center the model
    vec3 centered = aPos - modelCenter;
    
    // Apply rotation
    vec4 rotated = model * vec4(centered, 1.0);
    
    // Move back and apply translation
    vec3 finalPos = rotated.xyz + modelCenter + modelPosition;
    
    // Apply camera offset
    finalPos.z -= posOffset;
    
    // Use the projection matrix from camera
    vec4 clipPos = projection * vec4(finalPos, 1.0);
    
    // Adjust final position
    gl_Position = clipPos;
    
    // Pass values to fragment shader
    FragPos = finalPos;
    Normal = aNormal;
    TexCoord = aTexCoord;
}
