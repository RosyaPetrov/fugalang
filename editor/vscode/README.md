# Fugu Language

VS Code language support for Fugu.

## Features

- Syntax highlighting for `.f`, `.fg`, and `.fugu` files.
- Fugu keywords such as `module`, `use`, `fn`, `let`, `mut`, `struct`, `pub`, and control-flow words.
- Line and block comments.
- String, raw string, character, number, keyword, operator, and type highlighting.
- Basic snippets for functions, structs, imports, and mutable bindings.
- Bracket matching, auto-closing pairs, indentation, and folding markers.

## Development

Open this folder in VS Code and press `F5` to launch an Extension Development Host.

To package the extension:

```bash
npm install -g @vscode/vsce
vsce package
```
