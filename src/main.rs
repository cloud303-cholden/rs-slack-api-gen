use std::collections::HashMap;

use convert_case::{Case, Casing};
use string_template::Template;

mod template;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
struct Service {
    title: String,
    endpoints: String,
    input_structs: String,
    output_structs: String,
    types: String,
}

impl Service {
    fn render(&self) -> String {
        let mut args = HashMap::<&str, &str>::new();
        args.insert("title", &self.title);
        args.insert("endpoints", &self.endpoints);
        args.insert("input_structs", &self.input_structs);
        args.insert("output_structs", &self.output_structs);
        args.insert("types", &self.types);

        template::SERVICE.render(&args)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = "https://raw.githubusercontent.com/slackapi/slack-api-specs/master/web-api/slack_web_openapi_v2.json";
    let client = reqwest::Client::new();
    let resp: openapiv2::OpenApi = client.get(spec).send().await?.json().await?;


    let mut services = HashMap::<String, Service>::new();

    for (method_path, details) in resp.paths {
        let (service_name, endpoint) = method_path.split_once('.').unwrap();
        let service_name = service_name.to_string().replace('/', "");
        let service_title = service_name.to_case(Case::Title);
        let method_name = endpoint.replace('.', "_").to_case(Case::Snake);
        let method_type = method_path.replace('/', "").replace('.', "_").to_case(Case::UpperCamel);
        if method_type != "ChatPostMessage" {
            continue
        }

        // let mut service = services
        //     .get_mut
        //     .get_mut(&service_name)
        //     .unwrap_or_else(|| &mut Service::new(service_title));
        let mut endpoint = String::new();
        let mut input_struct = String::new();
        let mut output_struct = String::new();
        let mut types = String::new();

        let mut endpoint_args = HashMap::new();
        endpoint_args.insert("method_name", method_name.as_ref());
        endpoint_args.insert("method_type", method_type.as_ref());
        endpoint_args.insert("method_path", method_path.as_ref());

        if details.get.is_some() {
            endpoint = template::GET_ENDPOINT.render(&endpoint_args);
        }
        if details.post.is_some() {
            endpoint = template::POST_ENDPOINT.render(&endpoint_args);
        }
        // println!("{endpoint}");

        let parameters = details.post.unwrap().parameters.unwrap();
        let mut input_struct_args = HashMap::<&str, &str>::new();
        input_struct_args.insert("method_type", method_type.as_ref());

        let mut input_fields = String::new();
        for parameter in parameters {
            match parameter {
                openapiv2::ParameterOrReference::Parameter(p) => {
                    if let openapiv2::ParameterType::Header = p.in_ { 
                        continue
                    }
                    if let "token" = p.name.as_ref() { 
                        continue
                    }
                    let mut input_field_args = HashMap::<&str, &str>::new();
                    input_field_args.insert("field_name", &p.name);
                    let mut field_type = match p.type_.unwrap() {
                        openapiv2::PrimitiveType::String => "&'a str".to_string(),
                        openapiv2::PrimitiveType::Boolean => "bool".to_string(),
                        _ => unimplemented!(),
                    };
                    let field_required = p.required.unwrap_or(false);
                    if !field_required {
                        field_type = format!("Option<{}>", field_type);
                    }
                    input_field_args.insert("field_type", &field_type);
                    input_fields.push_str(&api_input_field_template.render(&input_field_args))
                },
                openapiv2::ParameterOrReference::Reference(p) => println!("{p:?}"),
            }
        }
        input_struct_args.insert("input_fields", &input_fields);
        input_struct = api_input_struct_template.render(&input_struct_args);
        // println!("{input_struct}");

        services.entry(service_name.clone()).and_modify(|service| {
            if !endpoint.is_empty() {
                service.endpoints.push_str("\n\n");
                service.endpoints.push_str(&endpoint);
            }
            service.input_structs.push_str(&input_struct);
            // service.output_structs.push_str(&output_struct);
            // service.types.push_str(&types);
        }).or_insert(Service {
            title: service_title,
            endpoints: endpoint,
            input_structs: input_struct,
            output_structs: output_struct,
            types,
        });
        let service = services.get(&service_name).unwrap().render();
        println!("{service}")
    }

    Ok(())
}
