# Experiments in diff algorithms

(currently only
[Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
is computed, no edit scripts)

Here *n* denotes the total length of the inputs
and *d* the Levenshtein distance between them.

Three algorithms are implemented:

* the classic *O(n^2)* dynamic programming algorithm
* Myers' *O(nd)* greedy search,
  this one is used in practice (e.g. in GNU diff)
* Myers' *O(n+d^2)* algorithm improves the previous one
  by skipping common substrings in constant time.
  This relies on the suffix and LCP arrays and an RMQ structure on the latter.
  The constant factors are terrible.

See: *An O(ND) Difference Algorithm and its Variations*, Eugene W. Myers

Possible future work:

* The naive LCP computation in the *O(nd)* algorithm could most likely be micro-optimized
  using techniques you would use for `strcmp` and the like:
  SIMD, `rep` instructions, unrolling, eliding bounds checks (sentinel characters at the end...).
* Building the suffix array and subsequent structures takes a long time.
  We don't need *all* the suffixes:
  We check the first few common characters naively anyway, as it's faster than an RMQ.
  Therefore, we could proceed naively until we encounter two suffixes that *are* in
  the partial suffix array. Difference cover sampling should work.
