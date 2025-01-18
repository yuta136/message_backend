CREATE TABLE IF NOT EXISTS database_name.messages (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    message TEXT NOT NULL,
    message2 TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO database_name.messages
(message, message2, created_at) 
VALUES 
(
    'こんにちは',
    'こんにちは2',
    '2025-01-21 02:02:49'
);

INSERT INTO database_name.messages
(message, message2, created_at) 
VALUES 
(
    'こんにちは3',
    'こんにちは4',
    '2025-01-22 02:02:49'
);