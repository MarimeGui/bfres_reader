# Rust BFRES
*Reads BFRES in Rust*

Thanks to:
* [Custom Mario Kart 8 Wiki](http://mk8.tockdom.com/wiki/BFRES_(File_Format)) for file specification
* [BFRESTool](https://github.com/aboood40091/BFRES-Tool) on how to use Index Groups
* [OBJ Wikipedia Article](https://en.wikipedia.org/wiki/Wavefront_.obj_file) for exporting OBJ
* [GTX-Extractor](https://github.com/aboood40091/GTX-Extractor) for the de-swizzling code, I did not come up with that

Run the Basic Info: 
```
$ cargo run --release --bin basic_info your_file.sbfres
```

Run the OBJ Exporter:
```
$ cargo run --release --bin obj_exporter your_file.sbfres output_folder
```

Possible Improvements:
* Simplify architecture (make folders and move stuff)
* Make a more convenient way of using Vertices and related data
* Read Skeleton related data
* DAE Export (Incl. Vertices, Materials, Skeleton)
* OBJ + Material export
* Write documentation