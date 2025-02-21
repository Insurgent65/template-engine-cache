use serde_json::json;
use neutralts::Template;

fn main() {
    let mut schema = json!({
        "config": {
            "cache_prefix": "neutral-cache",
            "cache_dir": "", // empty = current system tmp dir
            "cache_on_post": false,
            "cache_on_get": true,
            "cache_on_cookies": true,
            "cache_disable": false,
            "filter_all": false
        },
        "data": {
            "inject": "<>{:exit; 403 :}</>",
            "CONTEXT": {
                "GET": {
                    "inject": "<>{:exit; 403 :}</>"
                }
            }
        }
    });

    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            if let Some(val) = args.next() {
                let name = arg[2..].to_string();
                let value = val;
                schema["data"]["CONTEXT"]["GET"][&name.to_owned()] = json!(value.clone());
            }
        }
    }

    if let Some(inject_value) = schema["data"]["CONTEXT"]["GET"].get("inject") {
        schema["data"]["inject"] = json!(inject_value);
    }

    // Create and render template
    let mut template = Template::from_file_value("tpl/index.ntpl", schema).unwrap();
    let content = template.render();

    println!("{}", content);
    println!("Status code: {} {}", template.get_status_code(), template.get_status_text());
    println!("Render time: {:?}", template.get_time_duration());

}
