CREATE TABLE test_subscriptions (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    user_id INT NOT NULL,
    test_paper_id INT NOT NULL,
    test_schedule_id INT NOT NULL,

    FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE,

    FOREIGN KEY (test_paper_id)
        REFERENCES test_papers (id)
        ON DELETE CASCADE,

    FOREIGN KEY (test_schedule_id)
        REFERENCES test_schedules (id)
        ON DELETE CASCADE
);