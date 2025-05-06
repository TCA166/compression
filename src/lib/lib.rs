/// Module providing LZ family compression and decompression functions.
///
/// ## LZ family of algorithms
///
/// All algorithms in this family are based on the idea of using a sliding
/// window, and utilizing previous data as a dictionary of some kind. The
/// individual algorithms differ in how the dictionary actually looks, and
/// how we point into the dictionary, but the idea is the same.
///
/// ## Example
///
/// Here's a simple example how to compress binary data using the lz77
/// algorithm:
/// ```
/// use generic_compression::lz::lz77::lz77_encode;
///
/// let input = b"ABABABABA";
/// let encoded = lz77_encode(input, 4, 4);
/// assert!(encoded.len() < input.len());
/// ```
///
/// And here's how to compress a struct using the lz78 algorithm:
/// ```
/// use generic_compression::lz::lz78::{lz78_encode};
///
/// #[derive(Clone, PartialEq)]
/// struct MyStruct {
///     a: u8,
///     b: String,
/// }
///
/// impl MyStruct {
///     pub fn new(a: u8, b: String) -> Self {
///         MyStruct { a, b }
///     }
/// }
///
/// impl Default for MyStruct {
///     fn default() -> Self {
///         MyStruct::new(0, "".to_string())
///     }
/// }
///
/// let input = vec![
///     MyStruct::default(),
///     MyStruct::default(),
///     MyStruct::new(1, "hello".to_string()),
///     MyStruct::new(2, "world".to_string()),
/// ];
///
/// let encoded = lz78_encode(&input, 4, 4);
/// assert!(encoded.len() < input.len());
/// ```
/// Now if you really wanted to be fancy, placing those struct instances in a
/// [Rc](std::rc::Rc) would be really nice and performant. Also playing around
/// with [transforms](crate::transform) of your input data might be a good idea.
pub mod lz;

/// Module providing common transform functions. These don't compress data, but
/// provide a way to transform data altering it's properties; usually to make it
/// easier to compress.
///
/// ## Information theory
///
/// Usually when compressing data we want to reduce the amount of information
/// we are dealing with. The more random an event is, the more information it
/// carries. A very good example of this is how saying that the sun will rise
/// is unsurprising and carries little information. Entropy is the measure
/// of information a certain source carries. The more random a source is, the
/// higher the entropy.
///
/// ## Transformations and compression
///
/// This simple intro to information theory aside; by transforming data we can
/// change the randomness of the data. For example, sorted data is less random,
/// and therefore carries less information and is easier to compress. Assuming
/// that the reduction in randomness is greater than the overhead of the
/// transformation, we can achieve better compression ratios.
///
/// ## Example
///
/// Here's a simple example of how to use the Move-To-Front (MTF) transform on
/// text:
///
/// ```
/// use generic_compression::transform::mtf::{encode_move_to_front};
///
/// let input = vec!['l', 'o', 'r', 'e', 'm', 'i', 'p', 's', 'u', 'm'];
/// let mut ordering = vec!['e', 'i', 'l', 'm', 'o', 'p', 'r', 's', 'u'];
///
/// let encoded = encode_move_to_front(&input, &mut ordering);
/// assert_eq!(encoded, vec![2, 4, 6, 3, 5, 5, 6, 7, 8, 4]);
/// ```
/// We can see how beforehand we had not a single repeated character, and
/// now we have a single number repeated twice. That means we have reduced
/// randomness.
pub mod transform;

/// Module providing common encoding algorithms. Encoding algorithms are used to
/// convert data into a different format for storage or transmission.
/// Compression can be seen as a special case of encoding, where the goal is to
/// reduce the size of the data.
pub mod encoding;
