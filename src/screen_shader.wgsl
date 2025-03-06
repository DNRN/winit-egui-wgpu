struct Uniforms {
    // time: f32,
    resolution: vec2<f32>,
    mouse_pos: vec2<f32>,
    base_color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Renders a full-screen triangle without vertex data
@vertex
fn vs_main(@builtin(vertex_index) vert_index: u32) -> @builtin(position) vec4<f32> {
    // Generate positions for a full-screen triangle
    let pos = array(
        vec2(-1.0, -1.0),
        vec2(3.0, -1.0),
        vec2(-1.0, 3.0),
    );
    return vec4(pos[vert_index], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) frag_position: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_position.xy / uniforms.resolution;
    let dist = length(uv - uniforms.mouse_pos);
    
    // // Use uniform values
    // let color = mix(
    //     uniforms.base_color,
    //     vec3(1.0, 0.0, 0.0),
    //     smoothstep(0.2, 0.1, dist)
    // );
    
    // return vec4(color, 1.0);
    return vec4<f32>(
        uniforms.base_color.b,  // Blue channel
        uniforms.base_color.g,  // Green channel
        uniforms.base_color.r,  // Red channel
        uniforms.base_color.a   // Alpha
    );
}

// @fragment
// fn fs_main(
//     @builtin(position) frag_position: vec4<f32>
// ) -> @location(0) vec4<f32> {
//     // Pixel coordinates (0,0 at top-left)
//     let x = u32(frag_position.x);
//     let y = u32(frag_position.y);
    
//     // Example: Draw a red pixel at (100,100)
//     // if x == 100 && y == 100 {
//     //     return vec4(1.0, 0.0, 0.0, 1.0);
//     // }
    
//     // Default: Blue gradient background
//     // let blue = frag_position.x / 800.0; // Assuming 800px width
//     // return vec4(0.0, 0.0, blue, 1.0);

//     // Checkerboard pattern
//     let pattern = (x / 20 + y / 20) % 2;
//     return vec4(vec3(f32(pattern)), 1.0);


//     // Move a white pixel
//     // let time = f32(/* pass time via uniform */);
//     // let moving_x = u32(200.0 + 100.0 * sin(time));
//     // if x == moving_x && y == 300 {
//     //     return vec4(1.0);
//     // }
// }