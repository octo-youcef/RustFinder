# RustKit
A set of CLI tools built in Rust to simplify operation and navigation

## Finder
A tool to replace the complex bash `find` logic which searches for files (optionally) containing some string or regular expression pattern.

### The challenge
Finding files containing some string is a common use case in the shell, however the command is cumbersome:
```bash
# Bash command
find <dir> \
    -type f \
    -name <file pattern> \
    -exec grep -iH <search pattern> {} \;

# Target command
finder [<dir>] \
    -p <file pattern> \
    -s <search pattern>
```
