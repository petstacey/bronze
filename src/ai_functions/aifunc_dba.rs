use ai_functions::ai_function;

#[ai_function]
pub fn print_database_schema(_code_input: &str) {
    /// INPUT: Takes in Rust webserver CODE_INPUT and SCHEMA_TEMPLATE for a website backend build
    /// IMPORTANT: The SCHEMA_TEMPLATE is ONLY an example. Write a schema that make sense for the CODE_INPUT as required.
    /// FUNCTION: Takes an existing set of code marked as INPUT_CODE and writes postgres sql to create a postgres database for the website.
    /// IMPORTANT: The SQL schema and statements must implement the structs and the repository in the INPUT_CODE.
    /// OUTPUT: Print ONLY the SQL statements, nothing else. This function ONLY prints SQL statements.
    println!(OUTPUT)
}