CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    total_points BIGINT DEFAULT 0,
    current_streak INTEGER DEFAULT 0,
    avatar_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TYPE event_status AS ENUM ('open', 'closed', 'resolved');

CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    options JSONB NOT NULL,
    closing_time TIMESTAMPTZ NOT NULL,
    status event_status DEFAULT 'open',
    actual_outcome VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    CHECK (closing_time > created_at)
);

CREATE TABLE predictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    event_id UUID REFERENCES events(id) ON DELETE CASCADE,
    predicted_outcome VARCHAR(100) NOT NULL,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    points_awarded INTEGER,
    is_correct BOOLEAN,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, event_id) -- One prediction per user per event
);

CREATE INDEX idx_predictions_user_id ON predictions(user_id);
CREATE INDEX idx_predictions_event_id ON predictions(event_id);
CREATE INDEX idx_events_status_closing ON events(status, closing_time);