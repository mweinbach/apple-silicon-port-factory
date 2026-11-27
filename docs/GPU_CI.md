# Self-Hosted Apple Silicon GPU, Audio & E2E CI

GitHub-hosted macOS runners (`macos-26` in [`.github/workflows/ci.yml`](../.github/workflows/ci.yml))
have no physical Apple Silicon GPU, no CoreAudio output hardware, no Metal Shader Converter, and no
MoltenVK. To validate Metal 4 rendering, offline shader compilation, WASAPI→CoreAudio, live Windows
x86-64 graphics guests, `d3d12-replay`, and benchmarks continuously, the repository includes a
dedicated self-hosted workflow: [`.github/workflows/gpu-e2e.yml`](../.github/workflows/gpu-e2e.yml).

---

## 1. Security Model & Trigger Policy

Because this is a public repository and self-hosted runners execute code on maintainer-owned Apple
Silicon hardware, `.github/workflows/gpu-e2e.yml` **never** runs automatically on untrusted fork
pull requests:

| Trigger | Runs GPU CI? | Why |
|---|---|---|
| `push` to `main` | Yes | Already reviewed and merged to `main`. |
| `workflow_dispatch` | Yes | Explicit manual run by a maintainer (with optional `run_bench` input). |
| `pull_request_target` (`types: [labeled]`) with label `gpu-ci` | Only after `gpu-ci-pr` approval | GitHub executes `gpu-e2e.yml` strictly from the trusted base branch (`main`), so a fork PR cannot edit the workflow to bypass the gate. Applying `gpu-ci` starts the `ubuntu-latest` gate job (`approve-pr-gpu`) behind the protected GitHub Environment `gpu-ci-pr`. A runner owner configured as a Required Reviewer must explicitly approve the run before the self-hosted Mac job executes, and checkout is cryptographically pinned to the approved `pull_request.head.sha`. `opened`, `reopened`, and `synchronize` are excluded, so pushing new commits never re-triggers the workflow. |
| Any other `pull_request` / `pull_request_target` event | **No** | Untrusted code never reaches the self-hosted runner. |

Additional hardening in `gpu-e2e.yml`:
- Trusted base-branch workflow orchestration (`pull_request_target`): PR branches cannot modify `approve-pr-gpu`, `environment: gpu-ci-pr`, or the SHA checks.
- Protected GitHub Environment `gpu-ci-pr` with Required Reviewers (runner owner(s)) gates all PR runs on `ubuntu-latest` before any checkout occurs on the self-hosted Mac — label/triage permission alone cannot execute code on the runner host.
- PR checkouts are pinned to the exact approved `pull_request.head.sha` and verified with `git rev-parse HEAD` before running any build or test command.
- Minimal top-level `permissions: { contents: read }`.
- `actions/checkout` pinned to a full commit SHA with `persist-credentials: false`.
- Per-run isolated `PORT_AOT_DIR`, `PORT_CRASH_DIR`, and `TRANSLATOR_HOME` under `${{ runner.temp }}`.
- Prefer configuring the runner in `--ephemeral` mode (or inside a dedicated non-admin macOS account with zero commercial game files, personal credentials, or SSH keys in `$HOME`).

---

## 2. Runner Registration & Required Labels

Register an Apple Silicon Mac (macOS 26+ with Metal 4 GPU support) as a GitHub Actions runner for
the repository using these **exact** labels:

- `self-hosted` (applied automatically by `actions/runner`)
- `macOS` (applied automatically on macOS)
- `ARM64` (applied automatically on Apple Silicon)

The workflow targets:

```yaml
runs-on: [self-hosted, macOS, ARM64]
```

### Step-by-step registration

1. Create a dedicated non-admin macOS user (for example `ci-runner`) and log in to an Aqua GUI
   session if you want windowed `NSWindow` tests in `d3d11_e2e` and `vulkan_e2e` to open real
   windows (headless tests also run without a display).
2. In GitHub: **Settings → Actions → Runners → New self-hosted runner → macOS (ARM64)**.
3. Download and verify the official `actions-runner-osx-arm64-*.tar.gz` tarball from GitHub into
   `~/actions-runner`, then configure it:
   ```sh
   ./config.sh \
     --url https://github.com/<owner>/<repo> \
     --token <REGISTRATION_TOKEN> \
     --name apple-silicon-gpu-1 \
     --labels self-hosted,macOS,ARM64 \
     --work _work
   ```
4. Install and start the runner service (use a LaunchAgent inside the logged-in GUI session so
   Metal + `NSWindow` + CoreAudio have access to the window server and audio HAL):
   ```sh
   ./svc.sh install
   ./svc.sh start
   ```
5. Create the `gpu-ci` repository label in **Issues / Pull Requests → Labels** so maintainers can
   request GPU CI on reviewed pull requests.
6. Create the **`gpu-ci-pr`** protected Environment in **Settings → Environments → New environment**:
   - Enable **Required reviewers** and add only the runner owner(s) trusted with the Mac host.
   - Optionally restrict deployment branches or combine with `--ephemeral` runner registration so
     each job runs on a clean runner registration.

---

## 3. Host Software Prerequisites

Install the following on the self-hosted Apple Silicon runner:

| Dependency | Purpose | Install / Location |
|---|---|---|
| **macOS 26+ / 27+ (Apple Silicon)** | Physical Metal 4 GPU + CoreAudio HAL | Hardware requirement |
| **Xcode + Metal Toolchain** | Compiles MSL → `.metallib` offline (`xcrun -sdk macosx metal`) | `xcode-select --install` + `xcodebuild -downloadComponent MetalToolchain` |
| **Rust (`aarch64-apple-darwin`)** | Builds and tests all workspace crates | `rustup target add aarch64-apple-darwin` |
| **Zig 0.16.0** | Cross-compiles Windows x86-64 test guests (`d3d11_e2e`, `d3d12_e2e`, `vulkan_e2e`, `bench`) | On `PATH` or set `ZIG=/path/to/zig` |
| **Metal Shader Converter 4.x** | DXIL SM 6.x → Metal AIR (`libmetalirconverter.dylib`) | `~/.local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib` (or `METAL_IRCONVERTER_DYLIB`) |
| **`libdxilconv.dylib`** | FXC SM 4/5 → DXIL for `d3d11_e2e`, `d3d12_e2e`, and `dxbc_sm5` | `~/.local/lib/libdxilconv.dylib` (or `AW2_DXILCONV`; see `crates/dxil-metal/tools/dxilconv/build.sh`) |
| **MoltenVK (`libMoltenVK.dylib`)** | Vulkan 1.3 driver for `win32-rt` `misc::vulkan_1` and `vulkan_e2e` | `brew install molten-vk` (or `PORT_VULKAN_DYLIB`) |

---

## 4. Suites Executed by `gpu-e2e.yml`

All GPU and graphics tests execute with `MTL_DEBUG_LAYER=1` and `--test-threads=1`:

1. **`cargo runner-build`** — builds the universal `port-runner` binary for E2E guests and `bench`.
2. **`d3d12-metal`** — library unit/GPU tests plus integration targets `gpu_clear_resolve`,
   `gpu_mesh`, and `gpu_rayquery`.
3. **`dxil-metal`** — `--features metal` unit, converter, and `MTL4Archive` PSO cache tests, plus
   explicit offline SPIR-V and WGSL `.metallib` compilation verification when `xcrun -sdk macosx metal`
   is installed.
4. **`win32-rt`** — `d3d11`, `misc::vulkan_1` (`PORT_TEST_MOLTENVK=1`), and `audio` (WASAPI → CoreAudio).
5. **`port-loader` E2E** — live Windows x86-64 PE guests for `d3d11_e2e`, `d3d12_e2e`, and
   `vulkan_e2e` (`PORT_TEST_MOLTENVK=1`).
6. **`d3d12-replay`** — Metal 4 execution backend, synthetic trace generators, benchmark timing, and
   golden pixel hash tests.
7. **`cargo opsdb -- bench`** — guest CPU/SIMD/lock/D3D12 benchmark suite against `db/bench_baselines.json`.

### Running the exact GPU CI suite locally

```sh
export MTL_DEBUG_LAYER=1 PORT_TEST_MOLTENVK=1
cargo runner-build
cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1
cargo test -p d3d12-metal --target aarch64-apple-darwin --test gpu_clear_resolve --test gpu_mesh --test gpu_rayquery -- --test-threads=1
cargo test -p dxil-metal --features metal --target aarch64-apple-darwin -- --test-threads=1
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- d3d11 misc::vulkan_1 audio --test-threads=1
cargo test -p port-loader --target aarch64-apple-darwin --test d3d11_e2e --test d3d12_e2e --test vulkan_e2e -- --test-threads=1
cargo test -p d3d12-replay --target aarch64-apple-darwin -- --test-threads=1
cargo opsdb -- bench
```
