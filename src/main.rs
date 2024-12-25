use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{MultiPart, SinglePart, Attachment};
use lettre::transport::smtp::authentication::Credentials;
use std::fs::read;

fn main() {
    // Lecture du fichier pdf 
    let pdf_content = match read(r"C:\Users\ssdhp\OneDrive\Images\boarding-pass.pdf") {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading PDF file: {:?}", e);
            return;
        }
    };

    // Creation du contenue du mail
    let email = Message::builder()
        .from("abdellah.chafik.pro@gmail.com".parse().unwrap())
        .to("abdellah.chaafik@gmail.com".parse().unwrap())
        .subject("Collaboration")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::plain(String::from("Test"))
                )
                .singlepart(
                    Attachment::new(String::from("document.pdf"))
                        .body(pdf_content, "application/pdf".parse().unwrap())
                )
        )
        .unwrap();
    // mdp et mail de l'application gmail
    let creds = Credentials::new(
        String::from("abdellah.chafik.pro@gmail.com"),
        String::from("egux wgtx fpqz vsch")
    );
    // Configuration du serveur de mailing
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email avec pièce jointe envoyé!"),
        Err(e) => println!("Erreur survenue.. :{:?}", e),
    }
}