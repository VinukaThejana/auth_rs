use maud::{DOCTYPE, Markup, html};

pub fn send_otp(otp: &str, purpose: &str) -> Markup {
    let purpose = format!(
        "Please use the following One-Time Password (OTP) to {}. This OTP is valid for a limited time.",
        purpose
    );

    html! {
    (DOCTYPE)
    html {
    head {
    meta charset="UTF-8";
    meta name="viewport" content="width=device-width, initial-scale=1.0";
    title { "OTP Verification" }
    }
    body style="margin: 0; padding: 0; background-color: #f2f2f2;" {
    table role="presentation" cellpadding="0" cellspacing="0" border="0" width="100%" {
    tr {
    td style="padding: 20px 0;" {
    table align="center" cellpadding="0" cellspacing="0" border="0" width="600"
    style="border-collapse: collapse; background-color: #ffffff; border-radius: 8px; overflow: hidden; box-shadow: 0 4px 10px rgba(0,0,0,0.15);"
    {
    tr {
    td align="center" style="background-color: #2D89EF; padding: 30px 0;" {
    h1 style="color: #ffffff; font-family: Arial, sans-serif; font-size: 28px; margin: 0;" { "OTP Verification" }
    }
    }
    tr {
    td style="padding: 40px 30px; font-family: Arial, sans-serif;" {
    p style="color: #333333; font-size: 16px; margin: 0 0 20px;" { "Hello," }
    p style="color: #333333; font-size: 16px; margin: 0 0 20px;" {
    (purpose.as_str())
    }
    table align="center" cellpadding="0" cellspacing="0" border="0" style="margin: 20px auto;" {
    tr {
    td style="background-color: #f1f1f1; padding: 15px 25px; border-radius: 4px; text-align: center;" {
    span style="display: block; font-size: 32px; color: #2D89EF; font-weight: bold; letter-spacing: 5px;" { (otp) }
    }
    }
    }
    p style="color: #666666; font-size: 14px; margin: 20px 0 0;" {
    "If you did not request this OTP, please ignore this email."
    }
    }
    }
    tr {
    td style="background-color: #f7f7f7; padding: 20px 30px; text-align: center;" {
    p style="color: #999999; font-size: 12px; margin: 0;" {
    "© 2025 auth_rs. All rights reserved."
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
}
