# Windows x64 timestamp benchmark. Each export accepts an iteration count in ECX
# and returns elapsed guest ticks. See docs/DIAGNOSTICS.md for build and run commands.
.text
.macro packed_timestamp
    rdtscp
    shlq $32, %rdx
    orq %rdx, %rax
.endm

# kind 0 measures back-to-back instructions, 1 consumes the full timestamp,
# and 2 uses the timestamp as the address of a load observed by the caller.
.macro benchmark name, op, kind
    .p2align 4
    .globl \name
    .seh_proc \name
\name:
    .seh_endprologue
    movl %ecx, %r8d
    xorq %r10, %r10
    .if \kind == 2
        leaq table(%rip), %r11
    .endif
    packed_timestamp
    movq %rax, %r9
    testl %r8d, %r8d
    jz .Ldone_\name
.Lloop_\name:
    .if \kind == 0
        .rept 8
            \op
        .endr
    .else
        .rept 4
            \op
            .if \kind == 1
                shlq $32, %rdx
                orq %rdx, %rax
                xorq %rax, %r10
            .else
                andl $7, %eax
                movq (%r11,%rax,8), %rdx
                addq %rdx, %r10
                .ifc \op,rdtsc
                    rdtscp
                .endif
            .endif
        .endr
    .endif
    decl %r8d
    jnz .Lloop_\name
.Ldone_\name:
    packed_timestamp
    subq %r9, %rax
    movq %r10, checksum(%rip)
    retq
    .seh_endproc
    .section .drectve,"yn"
    .ascii " /export:\name"
    .text
.endm

benchmark bench_empty, nop, 0
benchmark bench_rdtsc, rdtsc, 0
benchmark bench_rdtscp, rdtscp, 0
benchmark use_tsc, rdtsc, 1
benchmark use_tscp, rdtscp, 1
benchmark load_tsc, rdtsc, 2
benchmark load_tscp, rdtscp, 2

.globl read_checksum
read_checksum:
    movq checksum(%rip), %rax
    retq
.section .drectve,"yn"
.ascii " /export:read_checksum"
.data
.p2align 6
table: .quad 1, 2, 3, 4, 5, 6, 7, 8
checksum: .quad 0
