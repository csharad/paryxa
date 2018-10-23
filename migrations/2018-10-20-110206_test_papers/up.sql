CREATE TYPE TEST_TYPE AS ENUM ('Scheduled', 'FreeForm');

CREATE TABLE test_papers (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    type TEST_TYPE NOT NULL
);