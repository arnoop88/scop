#version 330 core

layout(location = 0) in vec3 aPos;       // Position
layout(location = 1) in vec2 aTexCoord;  // Texture coordinates
layout(location = 2) in vec3 aNormal;    // Normal

uniform mat4 model;      // Model matrix (includes rotation)
uniform mat4 view;       // Camera view matrix
uniform mat4 projection; // Perspective projection matrix
uniform float posOffset;  // Add this line at the top with other uniforms

out vec3 FragPos;     // Pass world position to fragment shader
out vec3 Normal;      // Pass transformed normal to fragment shader

void main() {
    vec4 worldPos = model * vec4(aPos, 1.0);
    FragPos = worldPos.xyz;
    Normal = aNormal;
    
    vec3 pos = worldPos.xyz;
    pos.z -= posOffset;
    
    // Scale inversely with distance to maintain apparent size
    float distance = posOffset;
    float scale = 0.3 * (3.0 / distance);  // Base scale * distance adjustment
    
    // Apply different scale to Z for better depth handling when close
    float z_scale = scale * 0.1;  // Reduce Z scale to prevent clipping
    
    // Scale X and Y normally, but handle Z differently
    gl_Position = vec4(pos.x * scale, pos.y * scale, pos.z * z_scale, 1.0);
}
