# Rust BFRES
*Reads BFRES in Rust*

Thanks to:
* [Custom Mario Kart 8 Wiki](http://mk8.tockdom.com/wiki/BFRES_(File_Format))
* [BFRESTool](https://github.com/aboood40091/BFRES-Tool)

Run the Basic Info: 
```
$ cargo run --release --bin basic_info your_file.sbfres
```

Run the OBJ Exporter:
```
$ cargo run --release --bin obj_exporter your_file.sbfres output_folder
```