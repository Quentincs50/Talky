
CREATE TYPE role_type AS ENUM ('ADMIN', 'MEMBER');
CREATE TABLE server_members (
  id_user UUID NOT NULL,
  id_serv UUID NOT NULL,
  role role_type NOT NULL DEFAULT 'MEMBER',
  joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  
  CONSTRAINT pk_server_members
        PRIMARY KEY (id_user, id_serv),

    CONSTRAINT fk_member_user
        FOREIGN KEY (id_user)
        REFERENCES users(id_user)
        ON DELETE CASCADE,

    CONSTRAINT fk_member_server
        FOREIGN KEY (id_serv)
        REFERENCES servers(id_serv)
        ON DELETE CASCADE

);
CREATE INDEX idx_members_server ON server_members(id_serv);
CREATE INDEX idx_members_user ON server_members(id_user);
CREATE INDEX idx_members_role ON server_members(role);

COMMENT ON TABLE server_members IS 'Membres des serveurs et leurs rôles';