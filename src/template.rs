// use once_cell::sync::Lazy;
// use string_template::Template;


// pub static SERVICE: Lazy<Template> = Lazy::new(|| Template::new(r#"use crate::{
pub static SERVICE: &str = r#"use crate::\{
    client::ClientArc,
    api::\{Api, self},
};

pub struct { title }(ClientArc);

impl Api for { title } \{
    fn client(&self) -> ClientArc \{
        self.0.clone()
    }

    fn from_client(client: ClientArc) -> Self \{
        Self(client)
    }
}

impl { title } \{
{{ for endpoint in endpoints }}
{{ call endpoint with ctx.endpoint }}
{{ endfor }}
}

pub mod input \{
{{ for input_struct in input_structs }}
{{ call input_struct with ctx.input_structs }}
{{ endfor }}
}

pub mod output \{
{{output_structs}}
}

pub mod types \{
{{types}}
}
"#;

// pub static GET_ENDPOINT: Lazy<Template> = Lazy::new(|| Template::new(
pub static GET_ENDPOINT: &str = r#"    pub async fn {{method_name}}(&self) -> api::Response<output::{{method_type}}> {
        let request = api::Request {
            input: None,
            endpoint: "https://slack.com/api{{method_path}}",
        };
        self.client().get(request).await
    }
"#;

// pub static POST_ENDPOINT: Lazy<Template> = Lazy::new(|| Template::new(
pub static POST_ENDPOINT: &str = r#"    pub async fn {{method_name}}<'a, T>(&self, input: T) -> api::Response<output::{{method_type}}>
    where
        T: AsRef<input::{{method_type}}<'a>>,
    {
        let request = api::Request {
            input: Some(input.as_ref()),
            endpoint: "https://slack.com/api{{method_path}}",
        };
        self.client().post(request).await
    }"#;

// pub static INPUT_STRUCT: Lazy<Template> = Lazy::new(|| Template::new(r#"
pub static INPUT_STRUCT: &str = r#"
    #[derive(Default, serde::Serialize)]
    pub struct {{method_type}}<'a> {
    {{input_fields}}}
    pub type {{method_type}}Input<'a> = {{method_type}}<'a>;

    impl<'a> AsRef<{{method_type}}<'a>> for {{method_type}}<'a> {
        fn as_ref(&self) -> &{{method_type}}<'a> {
            self
        }
    }
"#;

// pub static OUTPUT_STRUCT: Lazy<Template> = Lazy::new(|| Template::new(r#"
pub static OUTPUT_STRUCT: &str = r#"
    #[derive(Default, serde::Deserialize)]
    pub struct {{method_type}}<'a> {
    {{fields}}}
    pub type {{method_type}}Input<'a> = {{method_type}}<'a>;
"#;

pub static STRUCT_FIELD: &str = r#"    pub {{field_name}}: {{field_type}},
    "#;
