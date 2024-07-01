use std::collections::HashMap;

pub fn build_url(
        base_url: String,
        endpoint: Option<String>,
        params: Option<HashMap<String, String>>,
) -> String {
    let mut url: Vec<String> = vec![base_url];

    if endpoint.is_some() {url.push(endpoint.unwrap())}

    if params.is_some() {
        let parameters: HashMap<String, String> = params.unwrap();
        let parameters_length: usize = parameters.len();

        url.push("?".to_string());

        for (key, value) in parameters.iter() {
            url.push(format!("{}={}", key, value));

            if parameters_length >= 2 {url.push("&".to_string())}
        }
    }

    return url.join("");
}
