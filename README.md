# Open CDS Parser
**Open Sourced and Reusable AST & Parser for SAP CAP CDS**

[![CI](https://github.com/zkud/open-cds-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/zkud/open-cds-parser/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/zkud/open-cds-parser/branch/main/graph/badge.svg?token=BITDY89WY7)](https://codecov.io/gh/zkud/open-cds-parser) [![Hits-of-Code](https://hitsofcode.com/github/zkud/open-cds-parser?branch=main)](https://hitsofcode.com/github/zkud/open-cds-parser/view?branch=main) 

## Overview

**!!! Currently the crate is in it's rough unstable beta, so be causious to use it in production or somehow rely on the interface. !!!**

The ```open-cds-parser``` crate was designed as the foundation for any tooling for SAP CAP CDS, which needs to work with the language
and do it efficiently. The crate itself is robustly inspired by a pretty famous project ESTree for JavaScript, which lays
under the hood of eslint and other similar tooling.

Try to implement your own handy tooling for CAP CDS :)

## Usage

To get started add this to your ```Cargo.toml```:
```
open_cds_parser="0.0.1"
```

## Features

- Fully editable CDS Absract Syntax Tree 
- Support of event-like usage via the visitor pattern 

## License

[MIT](LICENSE)
