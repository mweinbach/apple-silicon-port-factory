// Minimal DXC driver for generating aw2-shaders test fixtures (our own HLSL, never game shaders).
// Drives any libdxcompiler.dylib through its COM API. Prefer an open-source DXC build; the
// D3DMetal (GPTK / CrossOver) dylib is a dev-only fallback. See tools/README.md. Usually run
// via tools/build_fixtures.sh, or by hand:
//   clang++ -std=c++17 -arch <dylib arch> -O1 -o /tmp/dxc_compile crates/aw2-shaders/tools/dxc_compile.cpp
//   DXCOMPILER=/path/to/libdxcompiler.dylib /tmp/dxc_compile in.hlsl out.dxil cs_6_0 main
// The output is unsigned (no dxil.dll validator); tests fix the container hash with
// `aw2_shaders::hash::dxbc_checksum`.
//
// Interfaces mirror DXC's WinAdapter.h for non-Windows (COM-compatible vtables, no virtual
// destructor in IUnknown); wchar_t is 4 bytes.
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <dlfcn.h>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

struct GUID {
    uint32_t a;
    uint16_t b, c;
    uint8_t d[8];
};
typedef int32_t HRESULT;

struct IUnknownX {
    virtual HRESULT QueryInterface(const GUID &, void **) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
    // (current DXC WinAdapter: no virtual destructor in IUnknown)
};
struct IDxcBlob : IUnknownX {
    virtual void *GetBufferPointer() = 0;
    virtual size_t GetBufferSize() = 0;
};
struct IDxcBlobEncoding : IDxcBlob {
    virtual HRESULT GetEncoding(int *known, uint32_t *code_page) = 0;
};
struct IDxcOperationResult : IUnknownX {
    virtual HRESULT GetStatus(HRESULT *status) = 0;
    virtual HRESULT GetResult(IDxcBlob **result) = 0;
    virtual HRESULT GetErrorBuffer(IDxcBlobEncoding **errors) = 0;
};
struct DxcBuffer {
    const void *ptr;
    size_t size;
    uint32_t encoding;
};
struct IDxcCompiler3 : IUnknownX {
    virtual HRESULT Compile(const DxcBuffer *src, const wchar_t **args, uint32_t argc, void *include_handler, const GUID &riid,
                            void **result) = 0;
};

static const GUID CLSID_DxcCompiler = {0x73e22d93, 0xe6ce, 0x47f3, {0xb5, 0xbf, 0xf0, 0x66, 0x4f, 0x39, 0xc1, 0xb0}};
static const GUID IID_IDxcCompiler3 = {0x228b4687, 0x5a6a, 0x4730, {0x90, 0x0c, 0x97, 0x02, 0xb2, 0x20, 0x3f, 0x54}};
static const GUID IID_IDxcOperationResult = {0xcedb484a, 0xd4e9, 0x445a, {0xb9, 0x91, 0xca, 0x21, 0xca, 0x15, 0x7d, 0xc2}};

typedef HRESULT (*DxcCreateInstanceFn)(const GUID &, const GUID &, void **);

static std::wstring widen(const char *s) {
    std::wstring w;
    for (; *s; ++s) w.push_back((wchar_t)(unsigned char)*s);
    return w;
}

int main(int argc, char **argv) {
    if (argc < 5) {
        fprintf(stderr, "usage: %s in.hlsl out.dxil profile entry [extra dxc args...]\n", argv[0]);
        return 2;
    }
    const char *lib = getenv("DXCOMPILER");
    void *h = dlopen(lib ? lib : "libdxcompiler.dylib", RTLD_NOW);
    if (!h) {
        fprintf(stderr, "dlopen: %s\n", dlerror());
        return 1;
    }
    auto create = (DxcCreateInstanceFn)dlsym(h, "DxcCreateInstance");
    IDxcCompiler3 *compiler = nullptr;
    if (!create || create(CLSID_DxcCompiler, IID_IDxcCompiler3, (void **)&compiler) < 0 || !compiler) {
        fprintf(stderr, "DxcCreateInstance(IDxcCompiler3) failed\n");
        return 1;
    }
    std::ifstream in(argv[1], std::ios::binary);
    std::stringstream ss;
    ss << in.rdbuf();
    std::string src = ss.str();

    std::vector<std::wstring> wargs = {widen(argv[1]), L"-T", widen(argv[3]), L"-E", widen(argv[4])};
    for (int i = 5; i < argc; ++i) wargs.push_back(widen(argv[i]));
    std::vector<const wchar_t *> pargs;
    for (auto &a : wargs) pargs.push_back(a.c_str());

    DxcBuffer buf = {src.data(), src.size(), 65001 /* UTF-8 */};
    IDxcOperationResult *res = nullptr;
    HRESULT hr = compiler->Compile(&buf, pargs.data(), (uint32_t)pargs.size(), nullptr, IID_IDxcOperationResult, (void **)&res);
    if (hr < 0 || !res) {
        fprintf(stderr, "Compile failed: 0x%08x\n", (unsigned)hr);
        return 1;
    }
    HRESULT status = 0;
    res->GetStatus(&status);
    IDxcBlobEncoding *errors = nullptr;
    if (res->GetErrorBuffer(&errors) >= 0 && errors && errors->GetBufferSize()) {
        fwrite(errors->GetBufferPointer(), 1, errors->GetBufferSize(), stderr);
    }
    if (status < 0) {
        fprintf(stderr, "compilation failed: 0x%08x\n", (unsigned)status);
        return 1;
    }
    IDxcBlob *obj = nullptr;
    if (res->GetResult(&obj) < 0 || !obj) {
        fprintf(stderr, "no object\n");
        return 1;
    }
    std::ofstream out(argv[2], std::ios::binary);
    out.write((const char *)obj->GetBufferPointer(), (std::streamsize)obj->GetBufferSize());
    printf("%s: %zu bytes\n", argv[2], obj->GetBufferSize());
    return 0;
}
