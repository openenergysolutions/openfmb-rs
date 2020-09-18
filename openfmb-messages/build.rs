fn main() {
    const PROTOS: [&str; 15] = [
            "proto/breakermodule/breakermodule.proto",
            "proto/commonmodule/commonmodule.proto",
            "proto/coordinationservicemodule/coordinationservicemodule.proto",
            "proto/essmodule/essmodule.proto",
            "proto/generationmodule/generationmodule.proto",
            "proto/interconnectionmodule/interconnectionmodule.proto",
            "proto/loadmodule/loadmodule.proto",
            "proto/metermodule/metermodule.proto",
            "proto/optimizermodule/optimizermodule.proto",
            "proto/reclosermodule/reclosermodule.proto",
            "proto/regulatormodule/regulatormodule.proto",
            "proto/resourcemodule/resourcemodule.proto",
            "proto/solarmodule/solarmodule.proto",
            "proto/switchmodule/switchmodule.proto",
            "proto/uml.proto",
        ];
    const INCLUDES: [&str; 1] = ["proto"];
    const OUT: &str = "src";
    openfmb_codegen::Config::new()
        .btree_map(&["."])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir(OUT)
        .compile_protos(&PROTOS, &INCLUDES).unwrap();
}
