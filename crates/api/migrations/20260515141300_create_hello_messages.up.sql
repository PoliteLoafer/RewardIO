CREATE TABLE IF NOT EXISTS hello_messages (
    id SERIAL PRIMARY KEY,
    message TEXT NOT NULL
);

INSERT INTO hello_messages (message)
SELECT 'Hello from Postgres!'
WHERE NOT EXISTS (SELECT 1 FROM hello_messages);