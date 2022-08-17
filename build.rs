fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = vec![
        "proto/todo.proto",
        "proto/types/messages.proto"
    ]
    .iter()
    .map(|name| std::env::current_dir().unwrap().join(name))
    .collect::<Vec<_>>();

    let include_dirs = vec![
        "proto",
    ]
        .iter()
        .map(|name| std::env::current_dir().unwrap().join(name))
        .collect::<Vec<_>>();


    tonic_build::configure()
        // .build_client(false)
        .compile(&proto_files, &include_dirs)
        .unwrap();

    for path in vec![proto_files, include_dirs].concat() {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    Ok(())
}
