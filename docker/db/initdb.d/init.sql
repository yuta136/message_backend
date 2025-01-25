CREATE TABLE IF NOT EXISTS database_name.messages (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO database_name.messages
(message, created_at) 
VALUES 
(
    'こんにちは',
    '2025-01-21 02:02:49'
);

INSERT INTO database_name.messages
(message, created_at) 
VALUES 
(
    '元気ですか？',
    '2025-01-22 02:02:49'
);