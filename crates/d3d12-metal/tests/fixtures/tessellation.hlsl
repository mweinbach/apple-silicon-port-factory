// Redistributable SM 6.6 tessellation fixtures for d3d12-metal GPU tests (tests/gpu_tessellation.rs)
// and win32-rt D3D12 COM PSO tests.
// Compiled with `-validator-version 1.6` (PSV0 v2, non-"main" entry points) via
// crates/dxil-metal/tools/build_fixtures.sh.

#define TESS_RS \
    "RootFlags(ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT), " \
    "RootConstants(num32BitConstants=8, b0), " \
    "SRV(t0)"

cbuffer Params : register(b0) {
    float4 gTessFactors; // xyzw = outer[0..2]/outer[3], or tri outer+inside
    float4 gColor;       // rgba tint (w = scale for DS SRV blend)
};

// Structured buffer read by the Domain shader to verify kIRArgumentBufferHullDomainBindPoint (slot 3).
StructuredBuffer<float4> gDomainData : register(t0);

struct VsIn {
    float4 pos : POSITION;
    float4 col : COLOR0;
};

struct CpOut {
    float4 pos : POSITION;
    float4 col : COLOR0;
};

struct DsOut {
    float4 pos : SV_Position;
    float4 col : COLOR0;
};

// ---- Triangle domain (3 control points) ------------------------------------------------------

struct TriPatchConst {
    float edges[3] : SV_TessFactor;
    float inside   : SV_InsideTessFactor;
};

[RootSignature(TESS_RS)]
CpOut VS_Tri(VsIn v) {
    CpOut o;
    o.pos = v.pos;
    o.col = v.col;
    return o;
}

TriPatchConst PC_Tri(InputPatch<CpOut, 3> patch) {
    TriPatchConst o;
    o.edges[0] = gTessFactors.x;
    o.edges[1] = gTessFactors.y;
    o.edges[2] = gTessFactors.z;
    o.inside   = gTessFactors.w;
    return o;
}

[RootSignature(TESS_RS)]
[domain("tri")]
[partitioning("integer")]
[outputtopology("triangle_cw")]
[outputcontrolpoints(3)]
[patchconstantfunc("PC_Tri")]
CpOut HS_Tri(InputPatch<CpOut, 3> patch, uint i : SV_OutputControlPointID) {
    CpOut o = patch[i];
    return o;
}

[RootSignature(TESS_RS)]
[domain("tri")]
DsOut DS_Tri(TriPatchConst pc, float3 bary : SV_DomainLocation, const OutputPatch<CpOut, 3> patch) {
    DsOut o;
    float4 p = patch[0].pos * bary.x + patch[1].pos * bary.y + patch[2].pos * bary.z;
    float4 c = patch[0].col * bary.x + patch[1].col * bary.y + patch[2].col * bary.z;
    // Blend with SRV data from t0 (slot 0) and root constant color to verify HS/DS resource bindings.
    float4 srv = gDomainData[0];
    o.pos = float4(p.xy, p.z, 1.0);
    o.col = float4(c.rgb * gColor.rgb + srv.rgb * gColor.a, 1.0);
    return o;
}

// ---- Quad domain (4 control points, fractional_even) -----------------------------------------

struct QuadPatchConst {
    float edges[4]  : SV_TessFactor;
    float inside[2] : SV_InsideTessFactor;
};

[RootSignature(TESS_RS)]
CpOut VS_Quad(VsIn v) {
    CpOut o;
    o.pos = v.pos;
    o.col = v.col;
    return o;
}

QuadPatchConst PC_Quad(InputPatch<CpOut, 4> patch) {
    QuadPatchConst o;
    o.edges[0]  = gTessFactors.x;
    o.edges[1]  = gTessFactors.y;
    o.edges[2]  = gTessFactors.z;
    o.edges[3]  = gTessFactors.w;
    o.inside[0] = gTessFactors.x;
    o.inside[1] = gTessFactors.y;
    return o;
}

[RootSignature(TESS_RS)]
[domain("quad")]
[partitioning("fractional_even")]
[outputtopology("triangle_cw")]
[outputcontrolpoints(4)]
[patchconstantfunc("PC_Quad")]
CpOut HS_Quad(InputPatch<CpOut, 4> patch, uint i : SV_OutputControlPointID) {
    return patch[i];
}

[RootSignature(TESS_RS)]
[domain("quad")]
DsOut DS_Quad(QuadPatchConst pc, float2 uv : SV_DomainLocation, const OutputPatch<CpOut, 4> patch) {
    DsOut o;
    float4 top = lerp(patch[0].pos, patch[1].pos, uv.x);
    float4 bot = lerp(patch[2].pos, patch[3].pos, uv.x);
    float4 p   = lerp(top, bot, uv.y);
    float4 ctop = lerp(patch[0].col, patch[1].col, uv.x);
    float4 cbot = lerp(patch[2].col, patch[3].col, uv.x);
    float4 c    = lerp(ctop, cbot, uv.y);
    o.pos = float4(p.xy, p.z, 1.0);
    o.col = float4(c.rgb * gColor.rgb, 1.0);
    return o;
}

// ---- Optional Geometry Shader chained after Domain Shader ------------------------------------

[RootSignature(TESS_RS)]
[maxvertexcount(3)]
void GS_Swizzle(triangle DsOut input[3], inout TriangleStream<DsOut> stream) {
    for (uint i = 0; i < 3; ++i) {
        DsOut v = input[i];
        // Swap R and B channels so readback proves GS executed after DS.
        v.col = float4(v.col.b, v.col.g, v.col.r, 1.0);
        stream.Append(v);
    }
    stream.RestartStrip();
}

// ---- Pixel Shader ----------------------------------------------------------------------------

[RootSignature(TESS_RS)]
float4 PS_Color(DsOut i) : SV_Target {
    return i.col;
}
