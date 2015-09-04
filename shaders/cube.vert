#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out VERTEX_OUTPUT
{
    vec3 frag_pos;
    vec3 normal;
    vec3 light_pos;
    vec2 tex_coords;
} vtx_out;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 light_pos;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);

    vtx_out.tex_coords = tex_coords;

    vtx_out.frag_pos = vec3(view * model * vec4(position, 1.0));
    vtx_out.normal = mat3(transpose(inverse(view * model))) * normal;
    vtx_out.light_pos = vec3(view * vec4(light_pos, 1.0));
}
