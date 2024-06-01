-- Create `sysgroup` table.
create table if not exists "sysgroup" (
    sysgroup_id     uuid default gen_random_uuid(),
    name            varchar(30) not null unique,
    PRIMARY KEY (sysgroup_id)
);

-- Create `syspermission` table.
create table if not exists "syspermission" (
    syspermission_id    uuid default gen_random_uuid(),
    name                varchar(30) not null unique,
    PRIMARY KEY (syspermission_id)
);


-- # Join tables.

-- Create `appuser_sysgroup` table for many-to-many relationships between users and groups.
create table if not exists "appuser_sysgroup" (
    appuser_id     uuid references appuser(appuser_id),
    sysgroup_id    uuid references sysgroup(sysgroup_id),
    primary key (appuser_id, sysgroup_id)
);

-- Create `sysgroup_syspermission` table for many-to-many relationships between groups and permissions.
create table if not exists "sysgroup_syspermission" (
    sysgroup_id         uuid references sysgroup(sysgroup_id),
    syspermission_id    uuid references syspermission(syspermission_id),
    primary key (sysgroup_id, syspermission_id)
);


-- # Fixture hydration.

-- Insert "admin" user.
insert into appuser (username, password)
values (
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw'
);

-- Insert "users", "moderators" and "superusers" groups.
insert into sysgroup (name) values ('users');
insert into sysgroup (name) values ('moderators');
insert into sysgroup (name) values ('administrators');

-- Insert individual permissions.
-- Create.
insert into syspermission (name) values ('protected.create');
insert into syspermission (name) values ('restricted.create');
-- Read.
insert into syspermission (name) values ('protected.read');
insert into syspermission (name) values ('restricted.read');
-- Update.
insert into syspermission (name) values ('protected.update');
insert into syspermission (name) values ('restricted.update');
-- Delete.
insert into syspermission (name) values ('protected.delete');
insert into syspermission (name) values ('restricted.delete');

-- Insert group permissions.
insert into sysgroup_syspermission (sysgroup_id, syspermission_id)
values (
    (select sysgroup_id from sysgroup where name = 'users'),
    (select syspermission_id from syspermission where name = 'protected.read')
), (
    (select sysgroup_id from sysgroup where name = 'administrators'),
    (select syspermission_id from syspermission where name = 'restricted.read')
);

-- Insert users into groups.
insert into appuser_sysgroup (appuser_id, sysgroup_id)
values (
    (select appuser_id from appuser where username = 'ferris'),
    (select sysgroup_id from sysgroup where name = 'users')
), (
    (select appuser_id from appuser where username = 'admin'),
    (select sysgroup_id from sysgroup where name = 'users')
), (
    (select appuser_id from appuser where username = 'admin'),
    (select sysgroup_id from sysgroup where name = 'administrators')
);