
# String tables
Contains a list of null-terminated strings (section and symbol names)

### Standard ELF Section Definitions

| Section Name | Section Type | Flags | Use |
| :--- | :--- | :--- | :--- |
| **Core Data** | | | |
| `.bss` | `SHT_NOBITS` | A, W | Uninitialized data (Block Size Segment) |
| `.data` | `SHT_PROGBITS` | A, W | Initialized data segment |
| `.rodata` | `SHT_PROGBITS` | A | Read-only data (constants and literals) |
| `.text` | `SHT_PROGBITS` | A, X | Executable machine code |
| `.interp` | `SHT_PROGBITS` | [A] | Program interpreter path name (e.g., dynamic linker) |
| **Symbol/String Tables** | | | |
| `.shstrtab` | `SHT_STRTAB` | none | Section name string table |
| `.strtab` | `SHT_STRTAB` | none | General string table for symbols (symbol names) |
| `.symtab` | `SHT_SYMTAB` | [A] | Linker symbol table |
| `.dynsym` | `SHT_DYNSYM` | A | Symbol table specific to dynamic linking |
| `.hash` | `SHT_HASH` | A | Symbol hash table (for fast lookup of symbols) |
| **Linking & Dynamic Loading** | | | |
| `.dynamic` | `SHT_DYNAMIC` | A[, W] | Table containing runtime information needed by the dynamic loader. |
| `.dynstr` | `SHT_STRTAB` | A | String table specifically for the `.dynamic` section keys/values. |
| `.got` | `SHT_PROGBITS` | mach. dep. | Global Offset Table (stores addresses of external variables). |
| `.plt` | `SHT_PROGBITS` | mach. dep. | Procedure Linkage Table (handles function calls to shared libraries). |
| **Debugging & Metadata** | | | |
| `.comment` | `SHT_PROGBITS` | none | Version control information (often ignored by the linker). |
| `.note` | `SHT_NOTE` | none | A section reserved for platform or build-specific notes/metadata. |
| **Relocations** | | | |
| *(Unnamed)* | *(N/A)* | [A] | Relocations required for section name (links code to external references). |
