CREATE TABLE test_schedules (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    test_paper_id INT NOT NULL,
    time TIMESTAMP NOT NULL,
    duration INTEGER NOT NULL,

    FOREIGN KEY (test_paper_id)
        REFERENCES test_papers (id)
        ON DELETE CASCADE
);