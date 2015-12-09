
This is my substring search workspace.

Documentation
-------------

Fast substring search for strings and byte strings, using the `two-way algorithm`_.

This is the same code as is included in Rust's libstd to “power” ``str::find(&str)``,
but here it is exposed with some improvements:

- Available for byte string searches using ``&[u8]``
- Using ``memchr`` for the single byte case, which is ultra fast.
- Having an optional SSE4.2 accelerated version which is much faster.

Use cargo feature ``pcmp`` to enable SSE4.2 / pcmpestri accelerated version.

- ``twoway::find_bytes(text: &[u8], pattern: &[u8]) -> Option<usize>``
- ``twoway::rfind_bytes(text: &[u8], pattern: &[u8]) -> Option<usize>``
- ``twoway::find_str(text: &str, pattern: &str) -> Option<usize>``
- ``twoway::rfind_str(text: &str, pattern: &str) -> Option<usize>``

Recent Changes
--------------

- 0.1.1

  - Add ``rfind_bytes``, ``rfind_str``

- 0.1.0

  - Initial release
  - Add ``find_bytes``, ``find_str``

License
-------

MIT / APACHE-2.0


Interesting Links
-----------------

.. _`two-way algorithm`: http://www-igm.univ-mlv.fr/~lecroq/string/node26.html

- Two Way: http://www-igm.univ-mlv.fr/~lecroq/string/node26.html
- Matters Computational: http://www.jjj.de/fxt/#fxtbook


Notes
-----

Consider denying 0/n factorizations, see
http://lists.gnu.org/archive/html/bug-gnulib/2010-06/msg00184.html
