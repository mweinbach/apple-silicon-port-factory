// Redistributable hs_6_6 fixtures for entry-point resolution tests (tests/entry_point.rs).
// Compiled with `-validator-version 1.6` so PSV0 is v2 (no entry point name) and dxil-metal
// must extract the entry point from `!dx.entryPoints` in DXIL bitcode metadata.

struct ControlPoint {
    float4 pos : POSITION;
    float2 uv  : TEXCOORD0;
};

struct PatchConst {
    float edges[3] : SV_TessFactor;
    float inside   : SV_InsideTessFactor;
};

cbuffer TessParams : register(b0) {
    float4 gFactors;
};

PatchConst PC_Custom(InputPatch<ControlPoint, 3> patch, uint patchId : SV_PrimitiveID) {
    PatchConst o;
    o.edges[0] = gFactors.x;
    o.edges[1] = gFactors.y;
    o.edges[2] = gFactors.z;
    o.inside   = gFactors.w;
    return o;
}

// Non-passthrough control-point phase: DXIL defines BOTH PC_Custom and HS_Custom.
[domain("tri")]
[partitioning("fractional_odd")]
[outputtopology("triangle_cw")]
[outputcontrolpoints(3)]
[patchconstantfunc("PC_Custom")]
ControlPoint HS_Custom(InputPatch<ControlPoint, 3> patch, uint i : SV_OutputControlPointID) {
    ControlPoint o = patch[i];
    o.pos.xyz *= gFactors.x;
    return o;
}

PatchConst PC_PassThrough(InputPatch<ControlPoint, 3> patch) {
    PatchConst o;
    o.edges[0] = 2.0;
    o.edges[1] = 2.0;
    o.edges[2] = 2.0;
    o.inside   = 2.0;
    return o;
}

// Pass-through control-point phase: DXIL sets the entry function pointer in !dx.entryPoints
// to null and only defines PC_PassThrough, while !dx.entryPoints still records "HS_PassThrough".
[domain("tri")]
[partitioning("integer")]
[outputtopology("triangle_cw")]
[outputcontrolpoints(3)]
[patchconstantfunc("PC_PassThrough")]
ControlPoint HS_PassThrough(InputPatch<ControlPoint, 3> patch, uint i : SV_OutputControlPointID) {
    return patch[i];
}
