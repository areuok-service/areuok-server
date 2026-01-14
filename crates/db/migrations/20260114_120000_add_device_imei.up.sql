-- Add IMEI and nickname update tracking fields to devices table
ALTER TABLE devices ADD COLUMN IF NOT EXISTS imei VARCHAR(255) UNIQUE;

ALTER TABLE devices ADD COLUMN IF NOT EXISTS last_name_updated_at TIMESTAMPTZ;

-- Add index for IMEI queries
CREATE INDEX IF NOT EXISTS idx_devices_imei ON devices(imei);

-- Add unique constraint for device_name
ALTER TABLE devices ADD CONSTRAINT devices_device_name_key UNIQUE (device_name);
