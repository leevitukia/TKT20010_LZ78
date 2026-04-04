
Coverage Report (cargo llvm-cov --html)

bit_traits.rs:
Function Coverage: 100.00% (2/2)
Line Coverage: 100.00% (6/6)
Region Coverage: 90.00% (9/10)

bitreader.rs:
Function Coverage: 100.00% (4/4)
Line Coverage: 100.00% (52/52)
Region Coverage: 96.94% (95/98)

bitwriter.rs:
Function Coverage: 100.00% (5/5)
Line Coverage: 100.00% (46/46)
Region Coverage: 94.25% (82/87)

huffman.rs:
Function Coverage: 100.00% (12/12)
Line Coverage: 100.00% (105/105)
Region Coverage: 95.92% (235/245)

lz78.rs:
Function Coverage: 100.00% (5/5)
Line Coverage: 97.73% (86/88)
Region Coverage: 90.40% (160/177)

main.rs:
Function Coverage: 0.00% (0/1)
Line Coverage: 0.00% (0/14)
Region Coverage: 0.00% (0/37)



The inputs used for testing the bitwriter/reader classes are currently only the min/max of the integer size.

The inputs for LZ78/Huffman tests are mostly random, which isn't ideal in terms of run to run variance

The tests can be reproduced by running ``cargo test``

## LZ78 Tests

### encode_empty_input()
Tests whether or not the encoder handles empty inputs correctly

### decoded_file_matches_input()
Tests the encoder and decoder at the same time by encoding a 5 MiB input and checking if the decoded version matches the input

### encoded_file_is_smaller_than_input()
Encodes 10 KiB worth of zeroes and checks if the output is smaller than that

## Huffman Coding Tests

### encode_empty_input()
Tests whether or not the encoder handles empty inputs correctly

### decoded_file_matches_input()
Tests the encoder and decoder at the same time by encoding a 5 MiB input and checking if the decoded version matches the input

### encoded_file_is_smaller_than_input()
Encodes 10 KiB worth of zeroes and checks if the output is smaller than that

## BitWriter Tests

### write_10_bits()()
Writes a 10 bit integer and checks whether or not it matches the expected byte vector

### write_4_bit_integers()()
Writes 4 separate 4 bit integers and checks whether or not it matches the expected byte vector

### write_20_bits_with_10_bit_chunks_and_continuation_bit()
Writes a 20 bit integer using the variable length format with 10 bit chunks and checks whether or not it matches the expected byte vector

## BitReader Tests

### read_10_bits()()
Reads a 10 bit integer and checks whether or not it matches what was written

### read_4_bit_integers()()
Reads 4 separate 4 bit integers and checks if they match the 4 written integers

### read_20_bits_with_10_bit_chunks_and_continuation_bit()
Reads a 20 bit integer written using the variable length format and checks if it matches the written integer
