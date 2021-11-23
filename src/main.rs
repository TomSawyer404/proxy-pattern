use std::collections::HashMap;

/// Subject
trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u32, String);
}

/// Proxy
struct Nginx {
    application: Box<ApplicationServer>,
    max_allowed_req: u32,
    rate_limiter: HashMap<String, u32>,
}

impl Nginx {
    fn new() -> Self {
        Nginx {
            application: Box::new(ApplicationServer {}),
            max_allowed_req: 2,
            rate_limiter: HashMap::new(),
        }
    }

    fn check_rate_limiting(&mut self, url: &str) -> bool {
        self.rate_limiter.entry(url.to_string()).or_insert(1);

        if self.rate_limiter[&url.to_string()] > self.max_allowed_req {
            return false;
        }

        self.rate_limiter
            .insert(url.to_string(), self.rate_limiter[&url.to_string()] + 1);
        true
    }
}

impl Server for Nginx {
    fn handle_request(&mut self, url: &str, method: &str) -> (u32, String) {
        let allowed = self.check_rate_limiting(&url);
        if !allowed {
            return (403, "Not Allowed".to_string());
        }

        self.application.handle_request(url, method)
    }
}

/// Real subject
struct ApplicationServer {}

impl Server for ApplicationServer {
    fn handle_request(&mut self, url: &str, method: &str) -> (u32, String) {
        if url == "/app/status" && method == "GET" {
            return (200, "Ok".to_string());
        }

        if url == "/create/user" && method == "POST" {
            return (201, "User Created".to_string());
        }

        (404, "Not Ok".to_string())
    }
}

fn main() {
    let mut nginx_server = Nginx::new();
    let app_status_url = "/app/status";
    let create_user_url = "/create/user";

    let (http_code, body) = nginx_server.handle_request(app_status_url, "GET");
    println!(
        "\nUrl: {}\nHttpCode: {}\nBody: {}",
        app_status_url, http_code, body
    );

    let (http_code, body) = nginx_server.handle_request(app_status_url, "GET");
    println!(
        "\nUrl: {}\nHttpCode: {}\nBody: {}",
        app_status_url, http_code, body
    );

    let (http_code, body) = nginx_server.handle_request(app_status_url, "GET");
    println!(
        "\nUrl: {}\nHttpCode: {}\nBody: {}",
        app_status_url, http_code, body
    );

    let (http_code, body) = nginx_server.handle_request(create_user_url, "POST");
    println!(
        "\nUrl: {}\nHttpCode: {}\nBody: {}",
        app_status_url, http_code, body
    );

    let (http_code, body) = nginx_server.handle_request(create_user_url, "GET");
    println!(
        "\nUrl: {}\nHttpCode: {}\nBody: {}",
        app_status_url, http_code, body
    );
}
