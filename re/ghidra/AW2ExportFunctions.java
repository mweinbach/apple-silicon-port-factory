// Export all functions + call graph from the Ghidra program as JSON Lines.
//
// Usage (headless post-script): AW2ExportFunctions.java <out.jsonl>
// One object per line:
//   {"va":"0x...","end":"0x...","size":N,"name":"ns::Name","sig":"prototype",
//    "thunk":"0x..."|null,"noreturn":bool,"callees":["0x...",...]}
// Ingest with: tools/fndb.sh ingest-ghidra db/ghidra_functions.jsonl
// @category AW2
import java.io.BufferedWriter;
import java.io.FileWriter;
import java.util.Set;
import java.util.TreeSet;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;

public class AW2ExportFunctions extends GhidraScript {

    static String esc(String s) {
        if (s == null) return "null";
        StringBuilder b = new StringBuilder("\"");
        for (char c : s.toCharArray()) {
            switch (c) {
                case '"': b.append("\\\""); break;
                case '\\': b.append("\\\\"); break;
                case '\n': b.append("\\n"); break;
                case '\r': b.append("\\r"); break;
                case '\t': b.append("\\t"); break;
                default:
                    if (c < 0x20) b.append(String.format("\\u%04x", (int) c));
                    else b.append(c);
            }
        }
        return b.append('"').toString();
    }

    static String hex(Address a) { return "\"0x" + Long.toHexString(a.getOffset()) + "\""; }

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        String out = args.length > 0 ? args[0] : "db/ghidra_functions.jsonl";
        int n = 0;
        try (BufferedWriter w = new BufferedWriter(new FileWriter(out))) {
            FunctionIterator it = currentProgram.getFunctionManager().getFunctions(true);
            while (it.hasNext() && !monitor.isCancelled()) {
                Function f = it.next();
                if (f.isExternal()) continue;
                Set<Long> callees = new TreeSet<>();
                for (Function c : f.getCalledFunctions(monitor)) {
                    if (!c.isExternal()) callees.add(c.getEntryPoint().getOffset());
                }
                StringBuilder sb = new StringBuilder();
                sb.append("{\"va\":").append(hex(f.getEntryPoint()));
                sb.append(",\"end\":").append(hex(f.getBody().getMaxAddress().add(1)));
                sb.append(",\"size\":").append(f.getBody().getNumAddresses());
                sb.append(",\"name\":").append(esc(f.getName(true)));
                sb.append(",\"sig\":").append(esc(f.getPrototypeString(false, false)));
                Function th = f.isThunk() ? f.getThunkedFunction(false) : null;
                sb.append(",\"thunk\":").append(th != null && !th.isExternal() ? hex(th.getEntryPoint()) : "null");
                sb.append(",\"noreturn\":").append(f.hasNoReturn());
                sb.append(",\"callees\":[");
                boolean first = true;
                for (Long c : callees) {
                    if (!first) sb.append(',');
                    sb.append("\"0x").append(Long.toHexString(c)).append('"');
                    first = false;
                }
                sb.append("]}\n");
                w.write(sb.toString());
                n++;
                if (n % 20000 == 0) println("AW2ExportFunctions: " + n + " functions...");
            }
        }
        println("AW2ExportFunctions: wrote " + n + " functions to " + out);
    }
}
