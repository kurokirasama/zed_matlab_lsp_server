# zed_matlab_lsp_server

This project is a Zed language server extension for MATLAB, leveraging the official MATLAB Language Server provided by MathWorks.

## Requirements

*   [Zed Editor](https://zed.dev/)
*   [MATLAB 2021b+](https://mathworks.com/products/matlab.html) or newer
*   [Node.js](https://nodejs.org/)

## Installation
### Install extension
Install the extension from the Zed Extension tab.

### Install server
First, you need to install the official MATLAB Language Server.

```bash
git clone https://github.com/mathworks/MATLAB-language-server
cd MATLAB-language-server/
npm install && npm run compile && npm run package
```
