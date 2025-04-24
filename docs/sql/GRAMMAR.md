```sql
-- Transaction Control
BEGIN;
COMMIT;
ROLLBACK;

-- Insert Document
INSERT INTO users VALUES {
  "name": "ChatGPT",
  "type": "AI",
  "score": 100
};

-- Query Document(s)
SELECT * FROM users;
SELECT * FROM users WHERE name = "ChatGPT";

-- Update
UPDATE users SET score = 120 WHERE name = "ChatGPT";

-- Delete
DELETE FROM users WHERE name = "ChatGPT";
```