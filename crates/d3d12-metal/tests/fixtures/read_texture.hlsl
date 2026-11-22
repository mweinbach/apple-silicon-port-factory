// A green texel enables the fullscreen triangle. GPU tests use its coverage to
// verify that the vertex stage consumed the bound shader resource.
Texture2D<float4> inputTexture : register(t0);

float4 main(uint vertex : SV_VertexID) : SV_Position {
    float2 corner = float2((vertex << 1) & 2, vertex & 2);
    float2 position = corner * float2(2, -2) + float2(-1, 1);
    position.x += inputTexture.Load(int3(0, 0, 0)).g > 0.5 ? 0 : 8;
    return float4(position, 0, 1);
}
