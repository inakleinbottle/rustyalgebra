//! A tensor key is a compact representation of a single tensor word, consisting of a finite
//! sequence of letters taken from the alphabet. In some respects, a tensor key is similar to
//! a string, except that the alphabet over which the tensor word is defined can be arbitrary.
//! To simplify matters, we restrict our attention to alphabets consisting of the numbers 1 to N
//! (inclusive), where N is a positive integer which we shall call the _width_ of the alphabet.
//!
//! A naive approach for defining a tensor word would be store a sequence of vectors in a
//! `Vec<LetterType>`, but in practice this is wasteful and not ideal for cache locality in cases
//! where this matters. Since a typical alphabet will have relatively few letters, typically not
//! more than 1024, we can form a compact representation of a tensor word by packing the binary
//! representation of each letter into the bits of a u64. Even in the case when the alphabet has
//! width 10 (1024 letters), we can still pack approximately 6 letters into a 64 bit integer type.
//! The only minor bump is that we must subtract 1 from the letter so the stored word consists of
//! letters from the modified alphabet 0 to N-1.