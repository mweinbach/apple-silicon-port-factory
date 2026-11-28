// Original test fixture for pure in-register FP64 (`double`) lowering to FP32 (`float`) on Apple Silicon.
// Compiled with: dxc -T cs_6_6 -E CS_Fp64Lowering -Fo fp64_in_register.cs.dxil fp64_in_register.hlsl
RWByteAddressBuffer Output : register(u0);

[numthreads(4, 1, 1)]
void CS_Fp64Lowering(uint3 gid : SV_DispatchThreadID)
{
    double factor = ((gid.x & 1u) == 0u) ? 0.05 : 1.0;
    double scale = ((gid.x & 2u) == 0u) ? 38.0 : 32.0;
    double bias = (double)((float)gid.x * 0.25f);
    float result = (float)(scale * factor + bias);
    Output.Store(gid.x * 4u, asuint(result));
}
