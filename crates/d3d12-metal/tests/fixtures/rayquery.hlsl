// aw2-d3d12 GPU test fixture (tests/gpu_rayquery.rs): inline ray queries (DXR 1.1, cs_6_5).
//
// One ray per thread, origin (x, y, -1), direction +z, with x/y from the `Rays` buffer. The
// same TLAS is reachable as t0 (descriptor table) and t1 (root SRV); root constant 0 picks.
// Root constant 1 is the ray flags, 2 the instance mask.
// Out[2i]   = (committed status, InstanceID, InstanceIndex, PrimitiveIndex)
// Out[2i+1] = (asuint(t), asuint(bary.x), asuint(bary.y), contribution | frontFace << 31)
#define RS "DescriptorTable(SRV(t0)), SRV(t1), UAV(u0), RootConstants(num32BitConstants=4, b0), SRV(t2)"

RaytracingAccelerationStructure SceneTable : register(t0);
RaytracingAccelerationStructure SceneRoot : register(t1);
RWStructuredBuffer<uint4> Out : register(u0);
StructuredBuffer<float2> Rays : register(t2);
cbuffer Params : register(b0) {
    uint useRoot;
    uint rayFlags;
    uint instanceMask;
    uint pad;
};

void Trace(RaytracingAccelerationStructure scene, uint i) {
    RayDesc ray;
    ray.Origin = float3(Rays[i], -1.0);
    ray.Direction = float3(0.0, 0.0, 1.0);
    ray.TMin = 0.0;
    ray.TMax = 100.0;

    RayQuery<RAY_FLAG_NONE> q;
    q.TraceRayInline(scene, rayFlags, instanceMask, ray);
    while (q.Proceed()) {
        if (q.CandidateType() == CANDIDATE_NON_OPAQUE_TRIANGLE) {
            q.CommitNonOpaqueTriangleHit();
        }
    }
    uint status = q.CommittedStatus();
    if (status == COMMITTED_TRIANGLE_HIT) {
        float2 b = q.CommittedTriangleBarycentrics();
        uint extra = q.CommittedInstanceContributionToHitGroupIndex() | (q.CommittedTriangleFrontFace() ? 0x80000000u : 0u);
        Out[2 * i] = uint4(status, q.CommittedInstanceID(), q.CommittedInstanceIndex(), q.CommittedPrimitiveIndex());
        Out[2 * i + 1] = uint4(asuint(q.CommittedRayT()), asuint(b.x), asuint(b.y), extra);
    } else {
        Out[2 * i] = uint4(status, 0xFFFFFFFFu, 0xFFFFFFFFu, 0xFFFFFFFFu);
        Out[2 * i + 1] = uint4(0, 0, 0, 0);
    }
}

[RootSignature(RS)]
[numthreads(8, 1, 1)]
void main(uint3 tid : SV_DispatchThreadID) {
    if (useRoot != 0) {
        Trace(SceneRoot, tid.x);
    } else {
        Trace(SceneTable, tid.x);
    }
}
