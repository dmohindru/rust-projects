/*
Cli to take options
// Character for which bitmap need to be generated
-c, --char <Character>
// String for which bitmap need to be generated, mode MUST be specified
-s, --string <String> // String for which bitmap need to be generated
-m, --mode <scroll/next>
// print info about the bin file
-i, --info
// Mandatory options
// Specify square grid size
-g, --grid-size <number> eg 5, 8
// File to write output data if -c or (-s and -m) options are specified
// File to read data and print info if -i options are specified
-f, --file <filename>

Permitted combination of options
-c, -g, -f
-s, -m, -g, -f
-i, -g, -f
*/
fn main() {
    println!("Hello, world!");
}
