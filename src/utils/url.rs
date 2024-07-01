use std::collections::HashMap;


pub fn build_url(
    base_url: String,
    endpoint: Option<String>,
    params: Option<HashMap<String, String>>,
) -> String {
    let mut url: Vec<String> = vec![base_url.clone()];

    if endpoint.is_some() {
        let mut endpoint = endpoint.unwrap();

        if base_url.as_str().ends_with("/") && endpoint.starts_with("/") {
            endpoint = endpoint[1..endpoint.len()].to_string();

            println!("there should be one slash between `base_url` and `endpoint`");
            println!("removed slash from `endpoint`");
        }

        url.push(endpoint);
    }

    if params.is_some() {
        let parameters: HashMap<String, String> = params.unwrap();
        let parameters_length: usize = parameters.len();

        url.push("?".to_string());

        for (key, value) in parameters.iter() {
            url.push(format!("{}={}", key, value));

            if parameters_length >= 2 {
                url.push("&".to_string())
            }
        }
    }

    return url.join("");
}
