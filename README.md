# Coreutils Translation Benchmark

This benchmark suite will utilize the foreign function interface between C and Rust to test the Rust translations of the most critical C functions. Translating an entire codebase can be unnecessary and expensive, so this benchmark focuses on small groups of critical functions.

The process of the benchmark is as follows:

- We take a set of manually picked C functions from GNU Coreutils

- Generate Rust bindings for their dependencies using bindgen

- Pick out the bindings the translated Rust code is most likely to need to supply as context to the LLM

- Create a prompt consisting of instructions, the relevant Rust bindings, and the C code to translate

- Send to LLM and attempt to compile Rust code into shared library

- Attempt to link shared library with a modified version of GNU coreutils in which the function written in Rust is marked as `extern` and compile

- Test behavior


For each file/function, we generate a wrapper.h, which is a file with all the necessary #include statements. We use this file to generate Rust bindings.
The Rust bindings will be too large of a file to feed into the LLM as context. It can be thousands of lines.

For the benchmark, the C function, wrapper.h, and bindings.rs will all be prepared ahead of time. Meanwhile, this translation script will take a customizable prompt as input, feed it to the LLM, compile, and test the result.

## Naming conventions
Each test will be named after the C file being (partially) translated. So to test echo.c, run `python3 translate.py prompt.txt echo`
Each test will have a Rust project associated with it, generated by Cargo. So for echo.c, there will be a directory called echo_rust, and in it's src/ directory will be bindings.rs and lib.rs, where the translation goes.

## TODO
- Test whether supplying relevant Rust bindings decreases the number of compilation errors


