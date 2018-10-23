CREATE TABLE test_questions (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    question TEXT NOT NULL,
    test_paper_id INT NOT NULL,
    correct_option_id INT NOT NULL,

    FOREIGN KEY (test_paper_id)
        REFERENCES test_papers (id)
        ON DELETE CASCADE
);