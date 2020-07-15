use layer_0_ascii85::ascii85::parse_file;

fn main() {
    println!("{:?}", parse_file("../layer_1_bitwise/layer_1.txt"));
}
