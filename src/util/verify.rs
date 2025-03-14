use std::borrow::Cow;
use validator::ValidationError;

pub fn password(password: &str) -> Result<(), ValidationError> {
    let checks = [
        (password.len() < 8, "must be at least 8 characters long"),
        (password.len() > 200, "must be at most 200 characters long"),
        (
            password.chars().all(|c| c.is_ascii_alphanumeric()),
            "must contain at least one special character",
        ),
        (
            !password.chars().any(|c| c.is_lowercase()),
            "must contain at least one lowercase letter",
        ),
        (
            !password.chars().any(|c| c.is_uppercase()),
            "must contain at least one uppercase letter",
        ),
        (
            !password.chars().any(|c| c.is_numeric()),
            "must contain at least one number",
        ),
    ];

    for (not_valid, message) in checks {
        if not_valid {
            return Err(ValidationError::new("password").with_message(Cow::Borrowed(message)));
        }
    }

    Ok(())
}

pub fn username(username: &str) -> Result<(), ValidationError> {
    let checks = [
        (username.len() < 3, "must be at least 3 characters"),
        (username.len() > 20, "must be smaller than 20 characters"),
        (
            !username.chars().all(|c| c.is_alphanumeric()),
            "must contain only alphanumeric characters",
        ),
    ];

    for (not_valid, message) in checks {
        if not_valid {
            return Err(ValidationError::new("username").with_message(Cow::Borrowed(message)));
        }
    }

    Ok(())
}

pub fn otp(otp: &str) -> Result<(), ValidationError> {
    let checks = [
        (otp.len() != 6, "must be 6 characters long"),
        (
            !otp.chars().all(|c| c.is_numeric()),
            "must contain only numbers",
        ),
    ];

    for (not_valid, message) in checks {
        if not_valid {
            return Err(ValidationError::new("otp").with_message(Cow::Borrowed(message)));
        }
    }

    Ok(())
}
