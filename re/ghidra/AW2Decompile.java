// Decompile functions to C files.
//
// One-shot:  AW2Decompile.java <outdir> <va> [<va> ...]
//            writes <outdir>/<va>.c (va = lowercase hex without 0x prefix, e.g. 1404aa1e0.c)
// Server:    AW2Decompile.java <outdir> --serve <queuedir> [idle_minutes]
//            keeps the program open; polls <queuedir> for *.req files (one VA per line),
//            decompiles each VA into <outdir>, then renames <name>.req -> <name>.done.
//            Writes <queuedir>/heartbeat every poll; exits when <queuedir>/stop exists or
//            after idle_minutes (default 240) without requests.
// @category AW2
import java.io.File;
import java.io.FileWriter;
import java.nio.file.Files;
import java.util.List;

import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileOptions;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.listing.Function;

public class AW2Decompile extends GhidraScript {
    private DecompInterface ifc;

    private void openDecompiler() {
        ifc = new DecompInterface();
        DecompileOptions opts = new DecompileOptions();
        ifc.setOptions(opts);
        ifc.toggleCCode(true);
        ifc.toggleSyntaxTree(false);
        ifc.setSimplificationStyle("decompile");
        if (!ifc.openProgram(currentProgram)) {
            throw new RuntimeException("decompiler failed to open program: " + ifc.getLastMessage());
        }
    }

    private void decompileOne(String vaText, File outdir) throws Exception {
        String clean = vaText.trim().toLowerCase().replace("0x", "");
        if (clean.isEmpty()) return;
        long off = Long.parseUnsignedLong(clean, 16);
        File outf = new File(outdir, Long.toHexString(off) + ".c");
        File tmp = new File(outdir, Long.toHexString(off) + ".c.tmp");
        StringBuilder sb = new StringBuilder();
        Address a = toAddr(off);
        Function f = getFunctionAt(a);
        if (f == null) f = getFunctionContaining(a);
        if (f == null) {
            // Ghidra missed it (e.g. fndb gap function with no reference): create it in memory
            // (not saved in -readOnly mode).
            try {
                if (getInstructionAt(a) == null) {
                    new DisassembleCommand(new AddressSet(a), null, true).applyTo(currentProgram, monitor);
                }
                f = createFunction(a, null);
                if (f != null) sb.append("// NOTE: function created on the fly (not in the Ghidra DB)\n");
            } catch (Exception e) {
                sb.append("// create function failed: ").append(e).append('\n');
            }
        }
        if (f == null) {
            sb.append("// AW2Decompile: no function at/containing 0x").append(clean).append('\n');
        } else {
            long t0 = System.currentTimeMillis();
            DecompileResults res = ifc.decompileFunction(f, 120, monitor);
            sb.append("// ").append(f.getName(true)).append("  @ 0x")
              .append(Long.toHexString(f.getEntryPoint().getOffset()))
              .append("  size=").append(f.getBody().getNumAddresses())
              .append("  (Ghidra ").append(System.currentTimeMillis() - t0).append(" ms)\n");
            if (f.getEntryPoint().getOffset() != off) {
                sb.append("// NOTE: 0x").append(clean).append(" is inside this function\n");
            }
            sb.append("// prototype: ").append(f.getPrototypeString(false, false)).append('\n');
            if (res != null && res.decompileCompleted() && res.getDecompiledFunction() != null) {
                sb.append(res.getDecompiledFunction().getC());
            } else {
                sb.append("// decompilation failed: ")
                  .append(res != null ? res.getErrorMessage() : "null result").append('\n');
            }
        }
        try (FileWriter w = new FileWriter(tmp)) { w.write(sb.toString()); }
        if (!tmp.renameTo(outf)) {
            Files.move(tmp.toPath(), outf.toPath(), java.nio.file.StandardCopyOption.REPLACE_EXISTING);
        }
    }

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            println("usage: AW2Decompile.java <outdir> <va>... | <outdir> --serve <queuedir> [idle_min]");
            return;
        }
        File outdir = new File(args[0]);
        outdir.mkdirs();
        openDecompiler();
        try {
            if (!args[1].equals("--serve")) {
                for (int i = 1; i < args.length; i++) decompileOne(args[i], outdir);
                return;
            }
            File q = new File(args[2]);
            q.mkdirs();
            long idleMs = (args.length > 3 ? Long.parseLong(args[3]) : 240) * 60_000L;
            long last = System.currentTimeMillis();
            File hb = new File(q, "heartbeat");
            File stop = new File(q, "stop");
            println("AW2Decompile: serving queue " + q);
            while (!monitor.isCancelled()) {
                try (FileWriter w = new FileWriter(hb)) { w.write(Long.toString(System.currentTimeMillis())); }
                if (stop.exists()) { stop.delete(); break; }
                File[] reqs = q.listFiles((d, n) -> n.endsWith(".req"));
                if (reqs != null && reqs.length > 0) {
                    for (File r : reqs) {
                        List<String> lines;
                        try { lines = Files.readAllLines(r.toPath()); } catch (Exception e) { continue; }
                        for (String l : lines) {
                            try { decompileOne(l, outdir); }
                            catch (Exception e) { println("AW2Decompile: " + l + ": " + e); }
                        }
                        File done = new File(q, r.getName().replace(".req", ".done"));
                        r.renameTo(done);
                    }
                    last = System.currentTimeMillis();
                } else if (System.currentTimeMillis() - last > idleMs) {
                    println("AW2Decompile: idle timeout");
                    break;
                } else {
                    Thread.sleep(150);
                }
            }
        } finally {
            ifc.dispose();
            if (args.length > 2 && args[1].equals("--serve")) {
                new File(args[2], "heartbeat").delete();
            }
        }
    }
}
