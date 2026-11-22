// Import fndb names into the Ghidra program.
//
// Input: TSV produced by `tools/fndb.sh ghidra-names` (default db/ghidra_names.tsv), columns:
//   va(hex)  namespace_path("a::b::C" or "")  simple_name  kind  comment
// kind: function (create function if missing, rename) | label (plain label)
//
// Existing USER_DEFINED names (typed in the GUI) are never overwritten; everything else
// (DEFAULT/ANALYSIS/IMPORTED, incl. names from a previous run of this script) is replaced.
// Missing input file is not an error (prints a warning) so the full analysis can be launched
// before fndb has been built.
// @category AW2
import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;

import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.script.GhidraScript;
import ghidra.app.util.NamespaceUtils;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Namespace;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;

public class AW2ImportNames extends GhidraScript {

    private static String clean(String s) {
        // Ghidra symbol names may not contain whitespace.
        return s.replaceAll("\\s+", "_");
    }

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        String path = args.length > 0 ? args[0] : "db/ghidra_names.tsv";
        File f = new File(path);
        if (!f.exists()) {
            println("AW2ImportNames: WARNING " + path + " not found; nothing imported. "
                + "Run tools/fndb.sh ghidra-names && tools/ghidra_analyze.sh --names");
            return;
        }
        SymbolTable st = currentProgram.getSymbolTable();
        int renamed = 0, created = 0, skipped = 0, failed = 0, labels = 0, n = 0, unchanged = 0;
        try (BufferedReader r = new BufferedReader(new FileReader(f))) {
            String line;
            while ((line = r.readLine()) != null) {
                if (line.isEmpty() || line.startsWith("#")) continue;
                if (monitor.isCancelled()) break;
                String[] c = line.split("\t", -1);
                if (c.length < 4) continue;
                n++;
                try {
                    Address a = toAddr(Long.parseUnsignedLong(c[0].replace("0x", ""), 16));
                    String nsPath = c[1];
                    String name = clean(c[2]);
                    String kind = c[3];
                    String comment = c.length > 4 ? c[4].replace("\\n", "\n") : "";
                    Namespace ns = currentProgram.getGlobalNamespace();
                    if (!nsPath.isEmpty()) {
                        ns = NamespaceUtils.createNamespaceHierarchy(clean(nsPath), null,
                            currentProgram, SourceType.IMPORTED);
                    }
                    if (kind.equals("label")) {
                        st.createLabel(a, name, ns, SourceType.IMPORTED);
                        labels++;
                        continue;
                    }
                    Function fn = getFunctionAt(a);
                    if (fn == null) {
                        if (getInstructionAt(a) == null) {
                            new DisassembleCommand(new AddressSet(a), null, true)
                                .applyTo(currentProgram, monitor);
                        }
                        fn = createFunction(a, null);
                        if (fn == null) { failed++; continue; }
                        created++;
                    }
                    Symbol s = fn.getSymbol();
                    SourceType src = s.getSource();
                    boolean same = s.getName().equals(name) && fn.getParentNamespace().equals(ns);
                    if (same) {
                        unchanged++;
                    } else if (src == SourceType.USER_DEFINED) {
                        skipped++;   // never clobber names typed in the Ghidra GUI
                    } else {
                        fn.setParentNamespace(ns);
                        fn.setName(name, SourceType.IMPORTED);
                        renamed++;
                    }
                    if (!comment.isEmpty()) {
                        setPlateComment(a, comment);
                    }
                } catch (Exception e) {
                    failed++;
                    if (failed < 50) println("AW2ImportNames: line " + n + ": " + e);
                }
                if (n % 20000 == 0) println("AW2ImportNames: " + n + " rows...");
            }
        }
        println(String.format("AW2ImportNames: rows=%d renamed=%d unchanged=%d created=%d labels=%d skipped(user-defined)=%d failed=%d",
            n, renamed, unchanged, created, labels, skipped, failed));
    }
}
