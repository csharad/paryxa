CREATE TABLE test_rooms (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    user_id INT NOT NULL,
    test_paper_id INT NOT NULL,
    test_schedule_id INT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    finish_time TIMESTAMP,
    has_withdrawn BOOL
);