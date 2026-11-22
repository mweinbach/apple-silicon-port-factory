// aw2-shaders GPU test fixture (our own shader): samples one texture through two samplers so a
// test can tell which sampler state the converted shader actually used. The test binds s0/s1
// either as static samplers (in a different declaration order than their registers) or through
// a sampler descriptor table and compares against a CPU reference.
// Rebuild: crates/aw2-shaders/tools/build_fixtures.sh
Texture2D<float4> tex : register(t0);
SamplerState samp0 : register(s0);
SamplerState samp1 : register(s1);
RWStructuredBuffer<float4> results : register(u0);

[numthreads(16, 1, 1)]
void main(uint id : SV_DispatchThreadID)
{
    // u spans [-1, 2] so clamp / repeat / mirror address modes give different texels.
    float u = -1.0 + (id + 0.5) / 16.0 * 3.0;
    results[id * 2 + 0] = tex.SampleLevel(samp0, float2(u, 0.5), 0);
    results[id * 2 + 1] = tex.SampleLevel(samp1, float2(u, 0.5), 0);
}
