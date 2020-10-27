use proto_types::FileDescriptorProto;

pub type Module = Vec<String>;

pub fn module(file: &FileDescriptorProto) -> Module {
    file.package()
        .split('.')
        .filter(|s| !s.is_empty())
        .map(to_snake)
        .collect()
}
