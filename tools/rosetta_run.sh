#!/bin/sh
# Run an x86_64 Mach-O PIE binary under Rosetta 2 via dlopen() inside the
# system x86_64 Python interpreter with ROSETTA_DISABLE_AOT=1.
#
# When Rosetta's kernel AOT cache lock is wedged (e.g. after an interrupted
# debugserver session), direct execve() of newly compiled x86_64 binaries blocks
# in uninterruptible sleep (state U) waiting on oahd. Launching an already-cached
# system x86_64 binary (/usr/bin/python3) with ROSETTA_DISABLE_AOT=1 and loading
# our PIE binary via dlopen() runs purely in Rosetta's in-process JIT and
# bypasses oahd completely.
set -e
export ROSETTA_DISABLE_AOT=1
export ROSETTA_ADVERTISE_AVX=1
exec arch -x86_64 /usr/bin/python3 -c '
import ctypes, os, sys

argv_bytes = [os.fsencode(a) for a in sys.argv[1:]]
if not argv_bytes:
    sys.stderr.write("usage: rosetta_run.sh <x86_64-pie-binary> [args...]\n")
    os._exit(2)

libc = ctypes.CDLL("/usr/lib/libSystem.B.dylib")
libc._NSGetArgc.restype = ctypes.POINTER(ctypes.c_int)
libc._NSGetArgv.restype = ctypes.POINTER(ctypes.POINTER(ctypes.c_char_p))

argc = len(argv_bytes)
c_argv = (ctypes.c_char_p * (argc + 1))(*argv_bytes, None)
libc._NSGetArgc()[0] = argc
libc._NSGetArgv()[0] = ctypes.cast(c_argv, ctypes.POINTER(ctypes.c_char_p))

exe_path = os.path.abspath(sys.argv[1])
exe = ctypes.CDLL(exe_path)
main_fn = exe.main
main_fn.argtypes = [ctypes.c_int, ctypes.POINTER(ctypes.c_char_p)]
main_fn.restype = ctypes.c_int
rc = main_fn(argc, c_argv)
os._exit(rc)
' "$@"
