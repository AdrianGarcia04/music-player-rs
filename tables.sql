CREATE TABLE artist (
        id SERIAL PRIMARY KEY,
        name VARCHAR(60)
);

CREATE TABLE album (
        id SERIAL PRIMARY KEY,
        name VARCHAR(60)
);

CREATE TABLE genre (
        id SERIAL PRIMARY KEY,
        name VARCHAR(20)
);

CREATE TABLE disc (
        id SERIAL PRIMARY KEY,
        name VARCHAR(20)
);

CREATE TABLE song (
        id SERIAL PRIMARY KEY,
        artist_id INTEGER REFERENCES artist(id),
        album_id INTEGER REFERENCES album(id),
        genre_id INTEGER REFERENCES genre(id),
        disc_id INTEGER REFERENCES disc(id),
        title VARCHAR(60),
        lyrics TEXT,
        year INTEGER,
        duration INTEGER,
        date_recorded TIMESTAMP,
        date_released TIMESTAMP,
                UNIQUE(title)
);
