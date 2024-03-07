CREATE TABLE users (
id integer primary key autoincrement,
name text
);

CREATE TABLE devices (
id integer primary key autoincrement,
name text,
notification text,
type text
);

CREATE TABLE user_device (
id integer primary key autoincrement,
user_id integer,
device_id integer,
CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id),
CONSTRAINT fk_devices FOREIGN KEY (device_id) REFERENCES devices(id)
);
