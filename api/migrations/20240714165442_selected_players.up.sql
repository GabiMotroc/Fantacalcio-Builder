-- Add up migration script here
CREATE TABLE if NOT EXISTS selected_players(
    user_id integer NOT NULL REFERENCES users (id),
    player_id integer NOT NULL REFERENCES players (id),
    PRIMARY KEY (user_id, player_id)
);