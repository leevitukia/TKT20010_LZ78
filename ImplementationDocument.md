
## General Structure

The program is a CLI tool that takes in the input & output paths, the algorithm to use and whether or not to encode or decode.

Bitpacking logic is implemented in bitwriter.rs & bitreader.rs with a helper trait in bit_traits.rs.

LZ78 encoding/decoding is handled in LZ78.rs and Huffman encoding/decoding is handled in huffman.rs


## Shortcomings
### Performance
Both LZ78 and Huffman coding take a while to encode and decode compared to other implementations

### Testing
The tests I've written cover a relatively limited set of scenarios

## LLM Usage
I used the Claude Sonnet 4.6 model by Anthropic to help understand different compression concepts and some Rust patterns and syntax. All code was written by me
