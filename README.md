# idf-parser

Rust library for parsing Intermediate Data Format files used in PCB design.

The IDF specification describes three file types:
- Board file `.emn`: Contains the board outline and component placement.
- Panel file `.emn`: Contains the panel outline and board placement.
- Library file `.emp`: Contains the component library.

Example usage:

```rust
use idf_parser::parse_board_file;
use idf_parser::parse_library_file;

let board = parse_board_file("src/test_files/board.emn").unwrap();
let panel = parse_board_file("src/test_files/panel.emn").unwrap();
let library = parse_library_file("src/test_files/library.emp").unwrap();
```
Currently, this only supports the IDF 3.0 format, given its wide adoption with version 4.0 being a newer standard that is
not widely used yet.

[IDF v 3.0 file specification](http://www.simplifiedsolutionsinc.com/images/idf_v30_spec.pdf)

## Limitations
- Only the IDF 3.0 format is supported.
- Only tested with a small number of IDF files, 

## Requests
If you are interested in contributing the following would be very helpful:
- If you have an IDF file that does not parse correctly, please open an issue with the file attached.
- Assist in implementation of the IDF [4.0](https://www.simplifiedsolutionsinc.com/images/idf_v40_spec.pdf) and
[2.0](http://www.simplifiedsolutionsinc.com/images/idf_v20_spec.pdf) formats.

## Maintainers
Laurence Cullen: laurence@vanellus.tech  
[Vanellus Technologies](https://vanellus.tech/)