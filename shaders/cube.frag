#version 330 core

in VERTEX_OUTPUT
{
    vec3 frag_pos;
    vec3 normal;
    vec3 light_pos;
    vec2 tex_coords;
} vtx_in;

out vec4 color;

uniform vec3 light_color;
uniform vec3 light_ambient;
uniform vec3 light_diffuse;
uniform vec3 light_specular;
uniform float shine;
uniform sampler2D diffuse_map;
uniform sampler2D specular_map;

void main() {
    // ambient
    vec3 ambient = light_ambient * vec3(texture(diffuse_map, vtx_in.tex_coords));

    // diffuse
    vec3 norm = normalize(vtx_in.normal);
    vec3 light_dir = normalize(vtx_in.light_pos - vtx_in.frag_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = light_diffuse * diff * vec3(texture(diffuse_map, vtx_in.tex_coords));

    // specular
    vec3 view_dir = normalize(-vtx_in.frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), shine);
    vec3 specular = light_specular * spec * vec3(texture(specular_map, vtx_in.tex_coords));

    color = vec4(ambient + diffuse + specular, 1.0);
}
