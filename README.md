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
- Import/export of character sheets
- Import/export of campaign data
- Import/export of different systems
- Export charsheets as PDF
- In-campaign chat 
- Admin dashboard
- Event log
- Tracing and monitoring
- PWA support for android and ios

## Extra Planned Features
- Implement CI/CD via Github Actions
- Support Docker and Docker Compose
- Support Helm Charts for Kubernetes deployments
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