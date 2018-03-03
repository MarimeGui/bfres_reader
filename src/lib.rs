extern crate ez_io;

mod error;
pub mod fres;

/* General Ideas:
File Type: BFRES
Data Type: FRES
Compression Method: Yaz0
Data inside: Sub-files

String Table Length + Offset (Header)
 @ Offset + 0x1C
 String Length
 String Data

File Offsets + Count, one for each sub-file type (12 types) (Header) -> Sub-file Index Group Offset Table
 @ Offset + Position where you got it from -> Sub-file Index Group Header
  Length of group
  Number of entries / sub-files (should match count in header)
   @ each entry -> Sub-file index group Entry
    Search Value
    Left Index
    Right Index
    Name Pointer
    Data pointer

Sub-file index group entry -> Sub-file absolute offset
*/
