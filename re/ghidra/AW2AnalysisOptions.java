// Pre-analysis option tweaks for AlanWake2.exe (headless). Keeps Ghidra defaults except
// a few analyzers that are useless or very slow on this 62 MB binary.
// @category AW2
import ghidra.app.script.GhidraScript;

public class AW2AnalysisOptions extends GhidraScript {
    private void opt(String name, String value) {
        try {
            setAnalysisOption(currentProgram, name, value);
            println("AW2: analysis option '" + name + "' = " + value);
        } catch (Exception e) {
            println("AW2: could not set option '" + name + "': " + e.getMessage());
        }
    }

    @Override
    public void run() throws Exception {
        opt("Embedded Media", "false");          // scans all bytes for media headers; noise
        opt("Decompiler Parameter ID", "false");  // extremely slow on 140k+ functions (default off)
        opt("Aggressive Instruction Finder", "false");
        // Keep: x86 constant reference analyzer, PE exception handling (.pdata), RTTI analyzer,
        // Demangler Microsoft, Decompiler Switch Analysis, Function ID, Stack, Non-returning.
    }
}
