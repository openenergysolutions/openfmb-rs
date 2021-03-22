use openfmb_descriptors::OPENFMB_DESCRIPTORS;



fn main() {
    println!("traversing FileDescriptorSet for OpenFMB...");
    for file_descriptor in OPENFMB_DESCRIPTORS.file.iter() {
        println!("{:?}", file_descriptor);
    }
}
