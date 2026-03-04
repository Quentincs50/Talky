DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_type t
        JOIN pg_enum e ON t.oid = e.enumtypid
        WHERE t.typname = 'role_type' AND e.enumlabel = 'OWNER'
    ) THEN
        ALTER TYPE role_type ADD VALUE 'OWNER';
    END IF;
END$$;
