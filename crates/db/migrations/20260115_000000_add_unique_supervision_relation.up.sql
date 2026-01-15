-- Add unique constraint to prevent duplicate supervision relations
CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_supervision_relation
ON supervision_relations (supervisor_id, target_id);
