use spatialos_codegen::ASTBuilder;

fn main() -> () {
    let schema = ASTBuilder::default()
        .with_directory("./examples/schema")
        .build();
    println!("{:#?}", schema);
    let result = schema.generate("./examples/test/src/generated");
    println!("{:#?}", result);
}
