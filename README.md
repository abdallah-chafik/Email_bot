Email_Bot 📧
A simple and efficient SMTP client written in Rust for sending emails via Gmail. This project uses the lettre library for secure SMTP communications.

Features ✨
Gmail SMTP email sending
Secure authentication with Google application password support
File attachments support (PDF, images, etc.)
Bulk email sending via JSON configuration
Robust error handling
Detailed sending logs
Prerequisites 📋
Rust installed on your machine
Gmail account with two-factor authentication enabled
Generated Google application password
Technologies Used 🛠️
Rust
lettre crate for SMTP management
serde and serde_json crates for configuration
Gmail SMTP
Installation 💻
bash
Copy Code
git clone [your-repo]
cd email-sender
cargo build
Configuration ⚙️
Recipients Configuration
Create an email_config.json file in the project root:

json
Copy Code
{
    "sender": "your.email@gmail.com",
    "recipients": [
        "recipient1@example.com",
        "recipient2@example.com",
        "recipient3@example.com"
    ],
    "subject": "Email Subject",
    "body": "Message body",
    "smtp_password": "your_app_password"
}
Attachments 📎
Place your attachment files in an accessible folder
Modify the file path in the code according to your needs
Usage 🚀
Configure your email_config.json file
Prepare your attachments
Run the program:
bash
Copy Code
cargo run
Project Structure 📁
email-bot/
├── src/
│   └── main.rs
├── email_config.json
├── Cargo.toml
└── README.md
Contributing 🤝
Contributions are welcome! Feel free to:

Fork the project
Create a feature branch
Commit your changes
Push to the branch
Open a Pull Request
