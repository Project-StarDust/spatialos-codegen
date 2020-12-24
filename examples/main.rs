use spatialos_codegen::ASTBuilder;

fn main() {
    let schema = ASTBuilder::default()
        .with_directory("./examples/schema_old")
        .with_directory("./examples/schema")
        .build();
    let _result = schema.generate("./examples/test/src/generated", "generated");
}
