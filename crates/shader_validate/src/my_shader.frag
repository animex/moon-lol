#version 150

struct UniformsPixel
{
    vec4 cPaletteSelectMain;
    vec4 cPaletteSrcMixerMain;
    vec4 cAlphaErosionParams;
    vec4 cAlphaErosionTextureMixer;
    float AlphaTestReferenceValue;
    vec4 vReflectionFColor;
    vec4 APPLY_TEAM_COLOR_CORRECTION;
};

uniform UniformsPixel _UniformsPixel;

uniform sampler2D TEXTURE;
uniform sampler2D sPalettesTexture;
uniform sampler2D TEXTUREMULT;
uniform sampler2D CMB_TEX_PIXEL_COLOR_REMAP_RAMP_SMP_Clamp_No_Mip;
uniform samplerCube REFLECTION_MAP;
uniform sampler2D sAlphaErosionTexture;
uniform sampler2D NAVMESH_MASK_TEXTURE;

in vec4 TEXCOORD0;
in vec3 TEXCOORD1;
in vec4 TEXCOORD3;
in vec4 TEXCOORD4;
in vec2 TEXCOORD6;
in vec3 COLOR0;
out vec4 SV_Target0;

void main()
{
    float _85 = 1.0 / TEXCOORD1.z;
    vec2 _86 = TEXCOORD1.xy * _85;
    vec4 _91 = texture(TEXTURE, _86);
    _91.w = texture(TEXTURE, TEXCOORD3.zw * _85).w;
    vec4 _112 = texture(sPalettesTexture, vec2(clamp(dot(_91, _UniformsPixel.cPaletteSrcMixerMain), 0.0, 1.0), _UniformsPixel.cPaletteSelectMain.x) + vec2(_UniformsPixel.cPaletteSelectMain.z, _UniformsPixel.cPaletteSelectMain.w));
    vec4 _117 = texture(TEXTUREMULT, TEXCOORD3.xy);
    vec4 _118 = vec4(_112.x, _112.y, _112.z, _91.w) * _117;
    vec4 _119 = TEXCOORD0 * _118;
    vec3 _120 = _119.xyz;
    vec4 _126 = texture(CMB_TEX_PIXEL_COLOR_REMAP_RAMP_SMP_Clamp_No_Mip, vec2(dot(_120, vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875)), 0.5));
    vec3 _133 = vec3(0.0);
    if (_126.w > 0.0)
    {
        _133 = _126.xyz;
    }
    else
    {
        _133 = _120;
    }
    float _134 = _119.w;
    vec4 _138 = vec4(_133, _134);
    vec4 _164 = vec4(0.0);
    if (_UniformsPixel.APPLY_TEAM_COLOR_CORRECTION.x != 0.0)
    {
        float _144 = _133.y - _133.x;
        vec4 _163 = vec4(0.0);
        if (_144 > 0.001000000047497451305389404296875)
        {
            _163 = vec4(0.0, _133.y * 0.300000011920928955078125, _133.z + (_133.y * 3.0), _134 * 1.75);
        }
        else
        {
            vec4 _162 = vec4(0.0);
            if (_144 < (-0.001000000047497451305389404296875))
            {
                vec4 _159 = _138;
                _159.y = _133.y + (_133.x * 0.300000011920928955078125);
                _159.w = _134 * 2.0;
                _162 = _159;
            }
            else
            {
                _162 = _138;
            }
            _163 = _162;
        }
        _164 = _163;
    }
    else
    {
        _164 = _138;
    }
    float _173 = _118.w;
    float _196 = clamp(dot(texture(sAlphaErosionTexture, _86), _UniformsPixel.cAlphaErosionTextureMixer), 0.0, 1.0);
    vec2 _206 = clamp((vec2(_UniformsPixel.cAlphaErosionParams.x) - vec2(_196 - _UniformsPixel.cAlphaErosionParams.y, _196)) * _UniformsPixel.cAlphaErosionParams.zw, vec2(0.0), vec2(1.0));
    float _211 = _164.w * (_206.x - _206.y);
    vec3 _213 = clamp(((_164.xyz + (((texture(REFLECTION_MAP, TEXCOORD4.xyz).xyz * TEXCOORD4.w) * _173) * mix(vec3(1.0), _UniformsPixel.vReflectionFColor.xyz, vec3(TEXCOORD4.w)))).xyz + (COLOR0 * _173)).xyz, vec3(0.0), vec3(1.0));
    vec4 _214 = vec4(_213.x, _213.y, _213.z, _164.w);
    vec4 _218 = texture(NAVMESH_MASK_TEXTURE, TEXCOORD6);
    float _219 = _218.x;
    float _220 = isnan(_211) ? _219 : (isnan(_219) ? _211 : min(_219, _211));
    _214.w = _220;
    if ((_220 - _UniformsPixel.AlphaTestReferenceValue) < 0.0)
    {
        discard;
    }
    SV_Target0 = _214;
}

