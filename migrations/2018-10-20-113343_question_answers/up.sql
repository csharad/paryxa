CREATE TABLE question_answers (
    id SERIAL PRIMARY KEY,
    test_room_id INT NOT NULL,
    test_question_id INT NOT NULL,
    answered_option INT NOT NULL,

    FOREIGN KEY (test_room_id)
        REFERENCES test_rooms (id)
        ON DELETE CASCADE,

    FOREIGN KEY (test_question_id)
        REFERENCES test_questions (id)
        ON DELETE CASCADE,

    FOREIGN KEY (answered_option)
        REFERENCES question_options (id)
        ON DELETE CASCADE
);