use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

fn main() {
    let email = Message::builder()
        .from("abdellah.chafik.pro@gmail.com".parse().unwrap()) // email d'envoie
        .to("abdellah.chaafik@gmail.com".parse().unwrap())// email destination
        .subject("Collaboration") // objet de l'email
        .body(String::from("Test")) // contenue de l'email
        .unwrap();

    let creds = Credentials::new(
        String::from("abdellah.chafik.pro@gmail.com"), //email d'applications
        String::from("egux wgtx fpqz vsch") // mots passe de l'applications gmail
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com") // serveur d'envoie d'email
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email envoyé!"),
        Err(e) => println!("Erreur survenue.. :{:?}", e),// gestion d'erreur
    }
}