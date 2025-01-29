# scop

This project is a simple 3D viewer built using Rust and OpenGL. It allows you to load and display 3D models in OBJ format, with support for textures and different rendering modes.

## Features

- **Model Loading**: Load 3D models in OBJ format.
- **Texture Support**: Apply textures to models.
- **Multiple Rendering Modes**:
  - **Mode 1**: Colorful vertex-based rendering.
  - **Mode 2**: Face-based color rendering, where each face is colored based on its normal direction.
  - **Mode 3**: Textured rendering using a specified texture image.
- **Smooth Transition**: Smoothly transition between rendering modes.
- **Camera Control**: Zoom in and out, and rotate the model.
- **Command-Line Arguments**: Specify the model and texture files when running the program.


## Usage

1. **Build the Project**: Use Cargo to build the project.
   ```bash
   cargo build
   ```

2. **Run the Project**: You can run the project with optional command-line arguments to specify the model and texture files.
   ```bash
   cargo run -- path/to/model.obj path/to/texture.bmp
   ```

   If no arguments are provided, it will default to loading `models/42.obj` and `textures/sigma_cat.bmp`.

3. **Controls**:
   - **E**: Cycle through rendering modes (Vertex, Face, Texture).
   - **Arrow Keys**: Move the model in the respective direction.
   - **W/S**: Rotate the model up and down.
   - **A/D**: Rotate the model left and right.
   - **Z/X**: Zoom in and out.

## Dependencies

- `gl` for OpenGL bindings.
- `sdl2` for window management and input handling.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.