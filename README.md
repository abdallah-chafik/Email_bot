# Email_bot
Un client SMTP simple et efficace écrit en Rust permettant d'envoyer des emails via Gmail. Ce projet utilise la bibliothèque `lettre` pour gérer les communications SMTP de manière sécurisée.

## Fonctionnalités

- Envoi d'emails via SMTP Gmail
- Authentification sécurisée avec support des mots de passe d'application Google
- Gestion des erreurs


## Prérequis

- Rust installé sur votre machine
- Un compte Gmail avec l'authentification à deux facteurs activée
- Un mot de passe d'application Google généré

## Technologies utilisées

- Rust
- Crate `lettre` pour la gestion SMTP
- Gmail SMTP

## Installation

```bash
git clone [votre-repo]
cd email-sender
cargo build

