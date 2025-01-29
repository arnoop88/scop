#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

uniform sampler2D textureSampler;
uniform float textureBlend;  // 0.0 = vertex/face, 1.0 = texture

out vec4 FragColor;

// Function to create face-based colors
vec3 getFaceColor(vec3 normal) {
    vec3 absNormal = abs(normal);
    float threshold = 0.95; // Higher threshold for more precise face detection
    
    // Normalize the normal again to ensure unit length
    normal = normalize(normal);
    absNormal = abs(normal);
    
    if (absNormal.x > threshold) {
        // X-facing faces (red)
        return normal.x > 0.0 ? vec3(0.8, 0.0, 0.0) : vec3(0.6, 0.0, 0.0);
    } else if (absNormal.y > threshold) {
        // Y-facing faces (green)
        return normal.y > 0.0 ? vec3(0.0, 0.8, 0.0) : vec3(0.0, 0.6, 0.0);
    } else if (absNormal.z > threshold) {
        // Z-facing faces (blue)
        return normal.z > 0.0 ? vec3(0.0, 0.0, 0.8) : vec3(0.0, 0.0, 0.6);
    } else {
        // For faces that aren't clearly aligned with an axis,
        // interpolate between the closest two directions
        vec3 color = vec3(0.0);
        if (absNormal.x > 0.5) color += vec3(0.4, 0.0, 0.0) * absNormal.x;
        if (absNormal.y > 0.5) color += vec3(0.0, 0.4, 0.0) * absNormal.y;
        if (absNormal.z > 0.5) color += vec3(0.0, 0.0, 0.4) * absNormal.z;
        return color;
    }
}

void main() {
    // Compute all three color modes
    vec4 vertexColor = vec4(
        0.5 + (FragPos.x + 1.0) * 0.2,
        0.5 + (FragPos.y + 1.0) * 0.2,
        0.8 + (FragPos.z + 1.0) * 0.2,
        1.0
    );
    vec4 faceColor = vec4(getFaceColor(normalize(Normal)), 1.0);
    vec4 textureColor = texture(textureSampler, TexCoord);

    vec4 finalColor;

    // Smooth transitions between modes
    if (textureBlend <= 0.5) {
        // Blend Vertex -> Face
        float t = textureBlend / 0.5;
        finalColor = mix(vertexColor, faceColor, t);
    } else {
        // Blend Face -> Texture
        float t = (textureBlend - 0.5) / 0.5;
        finalColor = mix(faceColor, textureColor, t);
    }

    FragColor = finalColor;
}
