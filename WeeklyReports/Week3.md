
I spent most of this week on implementing bitpacking and Huffman coding. 

The program has progressed well! You can encode/decode any file with either algorithm, but the syntax for it needs a little bit of work. LZ78 is also very slow to encode at the moment, probably due to it using byte vectors as a key to a hashmap.

I learned a bit more about Huffman coding and how to use the "clap" library, which is used for CLI argument parsing. I also re-learned how bitwise operations & masks work

I'm probably going to spend most of next week on documentation, tests and performance since all of them are in kind of a sorry state

Once again I didn't keep track of the time I've spent on this but it's probably somewhere in the 5-10 hour range
