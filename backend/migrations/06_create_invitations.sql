CREATE TABLE invitations (
  id_inv UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  id_serv UUID NOT NULL,
  code VARCHAR(20) NOT NULL UNIQUE,
  created_by UUID NOT NULL,
  expires_at TIMESTAMPTZ,
  max_uses INTEGER,
  uses_count INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),


  CONSTRAINT fk_invitation_server
        FOREIGN KEY (id_serv)
        REFERENCES servers(id_serv)
        ON DELETE CASCADE,

    CONSTRAINT fk_invitation_creator
        FOREIGN KEY (created_by)
        REFERENCES users(id_user)
        ON DELETE CASCADE,

    CONSTRAINT check_uses_count_positive
        CHECK (uses_count >= 0),

    CONSTRAINT check_max_uses_positive
        CHECK (max_uses IS NULL OR max_uses > 0)

);

CREATE INDEX idx_invitations_server ON invitations(id_serv);
CREATE INDEX idx_invitations_code ON invitations(code);
CREATE INDEX idx_invitations_expires_at ON invitations(expires_at);

COMMENT ON TABLE invitations IS 'Codes invitation aux serveurs';