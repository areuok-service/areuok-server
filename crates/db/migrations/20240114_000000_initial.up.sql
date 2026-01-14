-- Create devices table
CREATE TYPE device_mode AS ENUM ('signin', 'supervisor');

CREATE TABLE IF NOT EXISTS devices (
    device_id UUID PRIMARY KEY,
    device_name VARCHAR(255) NOT NULL UNIQUE,
    imei VARCHAR(255) UNIQUE,
    mode device_mode NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_name_updated_at TIMESTAMPTZ
);

-- Create supervision_requests table
CREATE TYPE supervision_status AS ENUM ('pending', 'accepted', 'rejected');

CREATE TABLE IF NOT EXISTS supervision_requests (
    request_id UUID PRIMARY KEY,
    supervisor_id UUID NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    target_id UUID NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    status supervision_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create supervision_relations table
CREATE TABLE IF NOT EXISTS supervision_relations (
    relation_id UUID PRIMARY KEY,
    supervisor_id UUID NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    target_id UUID NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create signin_records table
CREATE TABLE IF NOT EXISTS signin_records (
    id SERIAL PRIMARY KEY,
    device_id UUID NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    date TIMESTAMPTZ NOT NULL,
    streak INTEGER NOT NULL DEFAULT 0,
    UNIQUE(device_id, date)
);

-- Indexes for performance
CREATE INDEX idx_supervision_requests_target ON supervision_requests(target_id, status);
CREATE INDEX idx_supervision_requests_supervisor ON supervision_requests(supervisor_id);
CREATE INDEX idx_supervision_relations_supervisor ON supervision_relations(supervisor_id);
CREATE INDEX idx_supervision_relations_target ON supervision_relations(target_id);
CREATE INDEX idx_signin_records_device ON signin_records(device_id);
