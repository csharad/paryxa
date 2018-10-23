CREATE TABLE question_options (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    option TEXT NOT NULL,
    test_question_id INT NOT NULL,
    is_correct BOOL,

    FOREIGN KEY (test_question_id)
        REFERENCES test_questions (id)
        ON DELETE CASCADE
);