CREATE TYPE GENDER_TYPE AS ENUM ('Male', 'Female', 'Other');
CREATE TYPE USER_TYPE AS ENUM ('Admin', 'Normal');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    first_name TEXT,
    last_name TEXT,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    gender GENDER_TYPE,
    contact TEXT,
    type USER_TYPE NOT NULL
);

CREATE UNIQUE INDEX unique_email on users (LOWER(email));
