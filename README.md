# Unbound Tome
### A free and open-source TTRPG platform for creating and viewing game content.

Initial milestone is aimed at supporting creation of DnD 5e campaigns and character sheets.


## Stack
- Axum
- Axum-login
- SeaORM w/ Postgres
- Tailwind
- HTMX
- Maud
- Oso

## Planned Features
- Oauth2 support via Axum-login
- Support for 5e.tools json import and updates
- import/export of character sheets
- import/export of campaign data
- import/export of different systems
- export charsheets as PDF
- in-campaign chat 
- Admin dashboard
- event log
- tracing and monitoring
- PWA support for android and ios

## Extra Planned Features
- Support Docker and k8s
- Consider supporting Valkey(Redis fork) and OpenTelemetry


## Installation
### Built using rust stable branch.

1. Install sea-orm-cli.
    ```bash
    cargo install sea-orm-cli
    ```

2. Create the .env file and configure your tokens (OAuth2 if needed) and the DB url.

3. Initialize the postgres container.
    ```bash
    docker compose -f dockercompose.yml up -d
    ```
4. Run migrations.
    ```bash
    sea-orm-cli migrate fresh
    ```

### Development
5. Start The development server.
    ```bash
    cargo watch -x run
    ```

### Production
5. Build the production binary.
    ```bash
    cargo build --release
    ```

## FAQ
1. __Why is the password verification so slow? (eg. 400ms on my PC)__

    Password-auth crate's password verification is considerably slower if built in debug mode. Building using --release flag will reduce the verification considerably. (20ms on my PC)