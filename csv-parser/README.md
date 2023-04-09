# CSV Parser

## [RFC4180 Compliant](https://datatracker.ietf.org/doc/html/rfc4180#page-2) CSV Parser written with my own finite-state-machine macro

This will become a fully compliant CSV parser. It is written with my own finite-state-machine macro. I think it is already pretty fast.
The code needs some refactoring and I want to add some more tests. But it is already usable. Only quote escaping is currently missing.

I want to add a some utility functions to the CSVData struct and implement some useful traits like IntoIterator etc.

Checkout the benchmarks for the simple example
