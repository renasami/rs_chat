-- Add migration script here
CREATE TABLE rooms (
    room_id UUID PRIMARY KEY,
    room_name VARCHAR(30) NOT NULL,
    belongs UUID[],
    created_by UUID REFERENCES users(user_id),
    created_at TIMESTAMP NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT false
);
