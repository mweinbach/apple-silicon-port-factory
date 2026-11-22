// Fullscreen-triangle vertex shader with a non-"main" entry point, compiled for validator 1.0 so
// PSV0 carries no entry name (runtime info v2). The converter must find "mainVS" from the
// bitcode. Rebuild with tools/build_fixtures.sh.
float4 mainVS(uint id : SV_VertexID) : SV_Position {
    float2 uv = float2((id << 1) & 2, id & 2);
    return float4(uv * float2(2, -2) + float2(-1, 1), 0, 1);
}
