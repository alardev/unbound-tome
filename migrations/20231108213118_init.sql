CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create users table.
create table if not exists "appuser"
(
    appuser_id   uuid default gen_random_uuid(),
    username     varchar(30) unique not null,
    password     text,
    access_token text,
    PRIMARY KEY (appuser_id)
);

-- Insert "ferris" user.
insert into "appuser" (username, password)
values ('ferris', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');