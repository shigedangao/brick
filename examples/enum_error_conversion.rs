use brick::brick;

enum DbError {
    NotFound(String),
    Timeout(u32),
    InvalidData(String),
    #[allow(dead_code)]
    ConnectionLost(String),
}

// User-friendly error messages
#[derive(Debug)]
#[brick(converter = "From", source = "DbError")]
#[allow(dead_code)]
enum UserError {
    #[brick_field(rename = "NotFound", transform_fn = "format_not_found")]
    NotFound(String),

    #[brick_field(rename = "Timeout", transform_fn = "format_timeout")]
    Timeout(String),

    #[brick_field(rename = "InvalidData", transform_fn = "format_invalid")]
    ValidationError(String),

    #[brick_field(rename = "ConnectionLost", transform_fn = "format_connection")]
    NetworkError(String),
}

fn format_not_found(table: String) -> String {
    format!("❌ Could not find record in {}", table)
}

fn format_timeout(seconds: u32) -> String {
    format!("⏱️  Request timed out after {} seconds", seconds)
}

fn format_invalid(reason: String) -> String {
    format!("⚠️  Invalid data: {}", reason)
}

fn format_connection(details: String) -> String {
    format!("🔌 Connection issue: {}", details)
}

fn main() {
    // Database returns raw error
    let db_err = DbError::NotFound("users".to_string());
    let user_err: UserError = db_err.into();
    println!("{:?}", user_err);

    // Timeout example
    let timeout = DbError::Timeout(30);
    let user_timeout = UserError::from(timeout);
    println!("{:?}", user_timeout);

    // Validation error
    let invalid = DbError::InvalidData("email format incorrect".to_string());
    if let UserError::ValidationError(msg) = UserError::from(invalid) {
        println!("{}", msg);
    }
}
