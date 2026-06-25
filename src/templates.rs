use askama::Template;

// This will generate the code using the template in this path,
// Relative to the `templates` dir in the crate root
#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    // the field name should match the variable name in your template
    name: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Debug, Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {}
