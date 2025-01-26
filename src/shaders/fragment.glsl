#version 330 core

in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

void main() {
    // Create a color based on position for better visualization
    vec3 color = vec3(
        (FragPos.x + 1.0) * 0.5,  // Red varies with x
        (FragPos.y + 1.0) * 0.5,  // Green varies with y
        (FragPos.z + 1.0) * 0.5   // Blue varies with z
    );
    FragColor = vec4(color, 1.0);
}
