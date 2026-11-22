// aw2-d3d12 GPU test fixture (tests/gpu_dispatch_rays.rs): DXR 1.0 state objects + DispatchRays (lib_6_3).
//
// Global root signature:
//   param 0: SRV(t0) - RaytracingAccelerationStructure Scene
//   param 1: UAV(u0) - RWStructuredBuffer<uint4> Out
//   param 2: SRV(t1) - StructuredBuffer<float2> Rays
//   param 3: RootConstants(num32BitConstants=4, b0) - (rayFlags, instanceMask, missIndex, hitGroupMultiplier)
//
// Local root signature (used by ClosestHitLocal):
//   param 0: RootConstants(num32BitConstants=2, b0, space1) - (localTag, localBias)

GlobalRootSignature GlobalRS = {
    "SRV(t0), UAV(u0), SRV(t1), RootConstants(num32BitConstants=4, b0)"
};

RaytracingAccelerationStructure Scene : register(t0);
RWStructuredBuffer<uint4> Out : register(u0);
StructuredBuffer<float2> Rays : register(t1);

cbuffer GlobalParams : register(b0) {
    uint rayFlags;
    uint instanceMask;
    uint missIndex;
    uint hitMultiplier;
};

cbuffer LocalParams : register(b0, space1) {
    uint localTag;
    uint localBias;
};

struct [raypayload] Payload {
    uint status : read(caller) : write(caller, closesthit, miss, anyhit);
    uint instanceID : read(caller) : write(caller, closesthit, miss);
    uint instanceIndex : read(caller) : write(caller, closesthit, miss);
    uint primIndex : read(caller) : write(caller, closesthit, miss);
    float t : read(caller) : write(caller, closesthit, miss);
    float baryX : read(caller) : write(caller, closesthit, miss);
    float baryY : read(caller) : write(caller, closesthit, miss);
    uint extra : read(caller, anyhit) : write(caller, closesthit, miss, anyhit);
};

[shader("raygeneration")]
void RayGenMain() {
    uint i = DispatchRaysIndex().x;
    RayDesc ray;
    ray.Origin = float3(Rays[i], -1.0);
    ray.Direction = float3(0.0, 0.0, 1.0);
    ray.TMin = 0.0;
    ray.TMax = 100.0;

    Payload p;
    p.status = 0xFFFFFFFFu;
    p.instanceID = 0;
    p.instanceIndex = 0;
    p.primIndex = 0;
    p.t = -1.0;
    p.baryX = 0.0;
    p.baryY = 0.0;
    p.extra = 0;

    TraceRay(Scene, rayFlags, instanceMask, 0, hitMultiplier, missIndex, ray, p);

    Out[2 * i] = uint4(p.status, p.instanceID, p.instanceIndex, p.primIndex);
    Out[2 * i + 1] = uint4(asuint(p.t), asuint(p.baryX), asuint(p.baryY), p.extra);
}

[shader("miss")]
void MissMain(inout Payload p) {
    p.status = 0;
    p.instanceID = 0xDEAD0001u;
    p.instanceIndex = DispatchRaysIndex().x;
    p.primIndex = DispatchRaysDimensions().x;
    p.t = RayTCurrent();
    p.baryX = 0.0;
    p.baryY = 0.0;
    p.extra = 0x1111u;
}

[shader("miss")]
void MissAlt(inout Payload p) {
    p.status = 0;
    p.instanceID = 0xDEAD0002u;
    p.instanceIndex = DispatchRaysIndex().x;
    p.primIndex = DispatchRaysDimensions().x;
    p.t = RayTCurrent();
    p.baryX = 0.0;
    p.baryY = 0.0;
    p.extra = 0x2222u;
}

[shader("closesthit")]
void ClosestHitMain(inout Payload p, in BuiltInTriangleIntersectionAttributes attr) {
    p.status = 1;
    p.instanceID = InstanceID();
    p.instanceIndex = InstanceIndex();
    p.primIndex = PrimitiveIndex();
    p.t = RayTCurrent();
    p.baryX = attr.barycentrics.x;
    p.baryY = attr.barycentrics.y;
    p.extra = p.extra | HitKind();
}

[shader("closesthit")]
void ClosestHitLocal(inout Payload p, in BuiltInTriangleIntersectionAttributes attr) {
    p.status = 1;
    p.instanceID = InstanceID() + localBias;
    p.instanceIndex = InstanceIndex();
    p.primIndex = PrimitiveIndex();
    p.t = RayTCurrent();
    p.baryX = attr.barycentrics.x;
    p.baryY = attr.barycentrics.y;
    p.extra = localTag;
}

[shader("anyhit")]
void AnyHitRejectPrim1(inout Payload p, in BuiltInTriangleIntersectionAttributes attr) {
    if (PrimitiveIndex() == 1) {
        IgnoreHit();
    }
    p.extra = 0xA000u;
}
