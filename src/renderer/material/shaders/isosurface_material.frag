
uniform vec3 camera_position;
uniform vec4 surface_color;
uniform float metallic;
uniform float roughness;
uniform sampler3D tex;
uniform vec3 size;
uniform float threshold;

in vec3 pos;

layout (location = 0) out vec4 outColor;

void main() {
    int steps = 200;
    float step_size = length(size) / float(steps);
    vec3 step = step_size * normalize(pos - camera_position);
    vec3 p = pos;
    for(int i = 0; i < 200; i++) {
        if(i == steps-1 || p.x < -0.501*size.x || p.y < -0.501*size.y || p.z < -0.501*size.z || p.x > 0.501*size.x || p.y > 0.501*size.y || p.z > 0.501*size.z) {
            outColor = vec4(0.0, 0.0, 0.0, 0.0);
            break;
        }
        float color = texture(tex, (p / size) + 0.5).r;
        if(color >= threshold) {
            vec3 normal = vec3(0.0, 1.0, 0.0);
            outColor.rgb = calculate_lighting(surface_color.rgb, p, normal, metallic, roughness, 1.0);
            outColor.rgb = reinhard_tone_mapping(outColor.rgb);
            outColor.rgb = srgb_from_rgb(outColor.rgb);
            outColor.a = surface_color.a;
            break;
        }
        p += step;
    }
}