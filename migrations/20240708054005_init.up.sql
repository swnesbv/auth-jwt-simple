-- Add up migration script here

-- Your SQL goes here
CREATE TABLE users (
	id         SERIAL       PRIMARY KEY,
	email      TEXT         NOT NULL UNIQUE,
	username   TEXT         NOT NULL UNIQUE,
	password   TEXT         NOT NULL,
	img        VARCHAR(255),
	status	   TEXT[] 		NOT NULL,
	created_at TIMESTAMPTZ  NOT NULL,
	updated_at TIMESTAMPTZ
);

INSERT INTO users (email, username, password, status, created_at)
VALUES ('admin@gmail.com', 'admin', '$pbkdf2-sha256$i=600000,l=32$83C1BHa1i8ERrnqiqSB6GQ$SeIofrKMqknulqrX0PQTgJYHIjxj87D4mNMrOtffMX4', array ['admin'], NOW());
INSERT INTO users (email, username, password, status, created_at)
VALUES ('two@example.com', 'two', '$pbkdf2-sha256$i=600000,l=32$83C1BHa1i8ERrnqiqSB6GQ$SeIofrKMqknulqrX0PQTgJYHIjxj87D4mNMrOtffMX4', array ['simply'], NOW());
INSERT INTO users (email, username, password, status, created_at)
VALUES ('three@example.com', 'three', '$pbkdf2-sha256$i=600000,l=32$83C1BHa1i8ERrnqiqSB6GQ$SeIofrKMqknulqrX0PQTgJYHIjxj87D4mNMrOtffMX4', array ['simply'], NOW());

CREATE TABLE article (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	title       VARCHAR(255) NOT NULL UNIQUE,
	description TEXT,
	img         VARCHAR(255),
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE comments (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	comment_on  JSON,
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE products (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	title       VARCHAR(255) NOT NULL,
	description TEXT,
	categories  TEXT[],
	cts  		TEXT[],
	amount  	JSON,
	price  		JSON,
	img         VARCHAR(255),
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE purchases (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	product_id  INTEGER      NOT NULL,
	categories  TEXT[],
	amount  	JSON,
	price  		JSON,
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id),
	FOREIGN KEY (product_id) REFERENCES products(id)
);
CREATE TABLE provision_d (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	title       VARCHAR(255) NOT NULL UNIQUE,
	description TEXT,
	st_date     Date,
	en_date     Date,
	s_dates     Date[],
	e_dates     Date[],
	dates       Date[],
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE provision_h (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	title       VARCHAR(255) NOT NULL UNIQUE,
	description TEXT,
	st_hour     TIMESTAMP,
	en_hour     TIMESTAMP,
	s_hours     TIMESTAMP[],
	e_hours     TIMESTAMP[],
	hours       TIMESTAMP[],
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE booking (
	id             SERIAL       PRIMARY KEY,
	user_id        INTEGER      NOT NULL,
	provision_d_id INTEGER,
	provision_h_id INTEGER,
	title          VARCHAR(255) NOT NULL UNIQUE,
	description    TEXT,
	st_date        Date,
	en_date        Date,
	st_hour        TIMESTAMP,
	en_hour        TIMESTAMP,
	completed      BOOLEAN      NOT NULL DEFAULT false,
	created_at     TIMESTAMPTZ  NOT NULL,
	updated_at     TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (provision_d_id) REFERENCES provision_d(id) ON DELETE CASCADE,
	FOREIGN KEY (provision_h_id) REFERENCES provision_h(id) ON DELETE CASCADE
);
CREATE TABLE schedule (
	id          SERIAL       PRIMARY KEY,
	user_id     INTEGER      NOT NULL,
	title       VARCHAR(255) NOT NULL UNIQUE,
	description TEXT,
	st_hour     TIMESTAMP,
	en_hour     TIMESTAMP,
	hours       TIMESTAMP[],
	occupied    TIMESTAMP[],
	places      INTEGER[],
	non_places  INTEGER[],
	completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE recording (
	id          SERIAL      PRIMARY KEY,
	user_id     INTEGER     NOT NULL,
	to_schedule INTEGER     NOT NULL,
	record_d    Date,
	record_h    TIMESTAMP,
	places      INTEGER[],
	tickets     JSON,
	completed   BOOLEAN     NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ NOT NULL,
	updated_at  TIMESTAMPTZ,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (to_schedule) REFERENCES schedule(id) ON DELETE CASCADE
);
CREATE TABLE groups (
    id          SERIAL 		 PRIMARY KEY,
    user_id     INTEGER      NOT NULL,
    title       VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    img         TEXT,
    completed   BOOLEAN      NOT NULL DEFAULT false,
	created_at  TIMESTAMPTZ  NOT NULL,
    updated_at  TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE subscriptions (
    id           SERIAL 	  PRIMARY KEY,
    user_id      INTEGER      NOT NULL,
    title        VARCHAR(255) NOT NULL UNIQUE,
    description  TEXT,
    to_user      INTEGER,
    to_group     INTEGER,
    dialogue     TEXT 		  UNIQUE,
    additionally JSON      	  NOT NULL,
    completed    BOOLEAN      NOT NULL DEFAULT false,
	created_at   TIMESTAMPTZ  NOT NULL,
    updated_at   TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (to_user) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (to_group) REFERENCES groups(id) ON DELETE CASCADE
);
CREATE TABLE chat_public (
	id          SERIAL      PRIMARY KEY,
	user_id     INTEGER     NOT NULL,
	joined      TEXT,
	came_out    TEXT,
	message     TEXT,
	created_at  TIMESTAMPTZ NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE TABLE chat_room (
	id          SERIAL      PRIMARY KEY,
	user_id     INTEGER     NOT NULL,
	joined      TEXT,
	came_out    TEXT,
	message     TEXT,
	room		TEXT     	NOT NULL,
	created_at  TIMESTAMPTZ NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE TABLE chat_groups (
	id          SERIAL      PRIMARY KEY,
	user_id     INTEGER     NOT NULL,
	joined      TEXT,
	came_out    TEXT,
	dialogue    INTEGER[],
	message     TEXT,
	created_at  TIMESTAMPTZ NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE TABLE sessions (
	session_token BYTEA             PRIMARY KEY,
	id integer REFERENCES users(id) ON DELETE CASCADE
);