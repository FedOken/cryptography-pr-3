#### Description
The library implements the following functionality:
1. <b>S block</b> based encryption. The bit representation is split into 4 bits in a block. The first two bits lead.
1.1. Substitution table for <i>encryption</i>
    |    | 00   | 01   | 10   | 11   |
    | ---| -----| -----| -----| -----|
    | 00 | 1010 | 0100 | 0011 | 1100 |
    | 01 | 0110 | 1001 | 0001 | 1111 |
    | 10 | 1110 | 0010 | 1101 | 0111 |
    | 11 | 0000 | 1000 | 0101 | 1011 |

    1.2. Substitution table for decryption
    |    | 00   | 01   | 10   | 11   |
    | ---| -----| -----| -----| -----|
    | 00 | 1100 | 0110 | 1001 | 0010 |
    | 01 | 0001 | 1110 | 0100 | 1011 |
    | 10 | 1110 | 0101 | 0000 | 1111 |
    | 11 | 0011 | 1101 | 1000 | 0111 |

2. <b>P block</b> based encryption. <i>Key</i> is used, XOR operation is applied to blocks of length 8 bits. Further it is possible to implement left or right shift and other operations. Conceptual realization is implemented.

#### Run:
`cargo test`

#### Additional:
In file lib.rs there are tests with examples of application and comments how the code works.

#### Examples:
String: <i>"H"</i>, key: <i>999</i>.
String in bits: `H => 01001000`
Bits encoded with S and P block: `01001000 => 10010001`

String: <i>"Hello"</i>, key: <i>998</i>.
String in bits: `Hello => 01001000 01100101 01101100 01101100 01101111`
Bits encoded with S and P block: `01001000 01100101 01101100 01101100 01101111 => 10001000 11111111 11110110 11110110 11111101`

String: <i>"Hello"</i>, key: <i>999</i>.
String in bits: `Hello => 01001000 01100101 01101100 01101100 01101111`
Bits encoded with S and P block: `01001000 01100101 01101100 01101100 01101111 => 10001001 11111110 11110111 11110111 11111100`

