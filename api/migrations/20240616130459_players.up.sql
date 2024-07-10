-- Add up migration script here
create type position as enum ('Goalkeeper', 'Defender', 'Midfielder', 'Forward');

create table if not exists players(
    id serial primary key,
    fantacalcio_id integer unique not null,
    position "position" not null,
    name text not null,
    team text not null,
    is_active bool default true not null
);