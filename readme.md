## Language-Oriented Frameworks and Tools: Integrity Tests

A small helper tool which generates integrity tests for LOFT::RGM specification model written in Rust.

**IMPORTANT**
This is **not** a validation tool, thus it does not need to be infitely precise. 
It just helps us to avoid lots of copy-paste work on boilerplate integrity tests code.

### Building
```cargo build --release```

### Usage

```cargo run <path_to_configuration>```

Example YAML-based configurations can be found under `configurations/` folder. To use loft-it you will probably need to edit example configuration according to your working setup.

### License
Copyright (c) 2018-2022 ArSysOp