CREATE TABLE TASK_ASSIGNMENTS (
    id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES tasks (id),
    user_id INTEGER REFERENCES users (id),
    start_time TIMESTAMP NOT NULL,
    duration INTEGER NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    priority INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('task_assignments');
