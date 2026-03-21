
## Project Requirements
Rust https://rust-lang.org/tools/install/

## Compiling
Run ``cargo build --release`` in the root directory  
The compiled binary will be located in target/release

## Testing
Run ``cargo test`` in the root directory

### Code Coverage
Rust doesn't officially support code coverage reports, but cargo-llvm-cov seems to work pretty well.  
You can install it with ``cargo install cargo-llvm-cov``

After it's installed you can get a code coverage report by running ``cargo llvm-cov --summary-only`` in the root directory
