# generic_compression

[![Rust](https://github.com/TCA166/compression/actions/workflows/rust.yml/badge.svg)](https://github.com/TCA166/compression/actions/workflows/rust.yml)

A simple Rust library and repository providing generic implementation of
compression algorithms. Instead of providing end-to-end compression for byte
arrays, this library provides a set of generic algorithms, that encode and
decode data into logical representations of compressed data. So for example, for
the `lz77` algorithm, if you provide a byte array, instead of outputting a
compressed byte array, this library outputs a vector of triples.

This means, that this library will not be ideal for the most common use cases,
however, if you wish to roll your own compression algorithm, out of prebuilt
algorithms this library is perfect for you. It is also a good way to learn
about compression algorithms, as the code is simple and easy to read.

## Library Features

- `lz77`: An implementation of the
  [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78#LZ77) compression
  algorithm.
- `lz78`: An implementation of the
  [LZ78](https://en.wikipedia.org/wiki/LZ77_and_LZ78#LZ78) compression
  algorithm.
- `lzw`: An implementation of the
  [LZW](https://en.wikipedia.org/wiki/LZ77_and_LZ78#LZW) compression algorithm.
- `MTF`: An implementation of the
  [Move-to-Front](https://en.wikipedia.org/wiki/Move-to-front_transform)
  transform.
- `BWT`: An implementation of the
  [Burrows-Wheeler Transform](https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform)
  transform.
- Huffman Encoding: An implementation of the
  [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding) algorithm.
- Arithmetic Encoding: An implementation of the
  [Arithmetic coding](https://en.wikipedia.org/wiki/Arithmetic_coding)
  algorithm.
- Serde support: The intermediate compressed data structures are serializable
  and deserializable using the `serde` library using the `serde` feature.

## Command Line Utility

This package also provides a command line utility that utilizes the library
to provide a simple interface for compressing and decompressing files using the
`lz77`, `lz78` and `lzw` algorithms. The command line utility is not the main
feature of this package, but it is a nice addition for testing and playing
around with how parameters affect the compression ratio and speed of the
algorithms.
