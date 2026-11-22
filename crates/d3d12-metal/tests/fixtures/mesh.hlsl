// aw2-d3d12 mesh-shader GPU test fixture (our own shader, not game content).
// Four quads, one per quadrant of the render target:
//   q0 top-left red, q1 top-right green, q2 bottom-left blue, q3 bottom-right grey (root constant).
// mainAS culls the quads whose bit is set in `cullMask` and forwards the survivors in its payload;
// mainMS draws payload.quads[SV_GroupID]. mainMSDirect (no amplification stage) draws quad
// SV_GroupID. Built by crates/aw2-shaders/tools/build_fixtures.sh.

#define RS "RootConstants(num32BitConstants=4, b0)"

cbuffer Params : register(b0)
{
    uint cullMask;
    uint grey;      // 0..255, q3's colour
    uint pad0;
    uint pad1;
};

struct Payload
{
    uint quads[4];
};

groupshared Payload s_payload;
groupshared uint s_count;

[RootSignature(RS)]
[numthreads(4, 1, 1)]
void mainAS(uint tid : SV_GroupThreadID)
{
    if (tid == 0)
    {
        uint n = 0;
        for (uint i = 0; i < 4; i++)
        {
            if (((cullMask >> i) & 1) == 0)
            {
                s_payload.quads[n] = i;
                n++;
            }
        }
        s_count = n;
    }
    GroupMemoryBarrierWithGroupSync();
    DispatchMesh(s_count, 1, 1, s_payload);
}

struct VOut
{
    float4 pos : SV_Position;
    float4 color : COLOR0;
};

float4 quad_color(uint q)
{
    switch (q)
    {
    case 0: return float4(1, 0, 0, 1);
    case 1: return float4(0, 1, 0, 1);
    case 2: return float4(0, 0, 1, 1);
    default: return float4(grey / 255.0, grey / 255.0, grey / 255.0, 1);
    }
}

void emit_quad(uint q, uint tid, out vertices VOut verts[4], out indices uint3 tris[2])
{
    SetMeshOutputCounts(4, 2);
    float2 origin = float2((q & 1) ? 0.0 : -1.0, (q & 2) ? -1.0 : 0.0);
    float2 corner = float2(tid & 1, tid >> 1);
    verts[tid].pos = float4(origin + corner, 0.5, 1);
    verts[tid].color = quad_color(q);
    if (tid < 2)
    {
        tris[tid] = tid == 0 ? uint3(0, 1, 2) : uint3(1, 3, 2);
    }
}

[RootSignature(RS)]
[outputtopology("triangle")]
[numthreads(4, 1, 1)]
void mainMS(uint tid : SV_GroupThreadID, uint gid : SV_GroupID, in payload Payload p,
            out vertices VOut verts[4], out indices uint3 tris[2])
{
    emit_quad(p.quads[gid], tid, verts, tris);
}

[RootSignature(RS)]
[outputtopology("triangle")]
[numthreads(4, 1, 1)]
void mainMSDirect(uint tid : SV_GroupThreadID, uint gid : SV_GroupID,
                  out vertices VOut verts[4], out indices uint3 tris[2])
{
    emit_quad(gid, tid, verts, tris);
}

[RootSignature(RS)]
float4 mainPS(VOut i) : SV_Target
{
    return i.color;
}
