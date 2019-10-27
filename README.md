# Features
- Debugging on Linux (x64 or Aarch64), macOS and Windows,
- Conditional breakpoints, function breakpoints, data breakpoints, logpoints,
- Launch debuggee in integrated or external terminal,
- Disassembly view with instruction-level stepping,
- Loaded modules view,
- Python scripting,
- HTML rendering for advanced visualizations,
- Rust language support with built-in visualizars for vectors, strings and other standard types,
- Global and workspace defaults for launch configurations,
- Remote debugging,
- Reverse debugging (experimental, requires compatible backend).

For full details please see the [Users Manual](MANUAL.md).

# Requirements
- 64-bit OS,
- Python 3.3 or later (optional for non-Windows platforms)
- Python 3.6 64 bit (required on Windows)

# Quick Start
Here's a minimal debug configuration to get you started:
```javascript
{
    "name": "Launch",
    "type": "lldb",
    "request": "launch",
    "program": "${workspaceFolder}/<my program>",
    "args": ["-arg1", "-arg2"],
}
```

# Links
- [Users Manual](MANUAL.md)
- [Debugging in VS Code](https://code.visualstudio.com/docs/editor/debugging)
- [Troubleshooting](https://github.com/vadimcn/vscode-lldb/wiki/Troubleshooting)
- [Wiki](https://github.com/vadimcn/vscode-lldb/wiki)
- [Chat](https://gitter.im/vscode-lldb/QnA)


# Screenshots

C++ debugging with data visualization ([Howto](https://github.com/vadimcn/vscode-lldb/wiki/Data-visualization)):<br>
![source](images/plotting.png)
<br>
<br>
Rust debugging:<br>
![source](images/source.png)


