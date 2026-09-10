-- Every note in a list, id-ordered. Typed by SQLite against your schema.
SELECT id, list_id, body, updated_at_ms
FROM notes
WHERE list_id = :listId
ORDER BY id
