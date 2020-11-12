use spatial_codegen::AST;

fn main() -> () {
    let schema = AST::from("./examples/schema");
    println!("{:#?}", schema);
    let result = schema.generate("./examples/test/src/generated");
    println!("{:#?}", result);
}
