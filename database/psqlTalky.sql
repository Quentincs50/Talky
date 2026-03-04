CREATE EXTENSION IF NOT EXISTS pgcrypto;



CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;



--user

CREATE TABLE users (
  id_user UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR(50)NOT NULL UNIQUE,
  email VARCHAR(100) NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  profile_pic BYTEA,
  status VARCHAR(20) DEFAULT 'offline',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

CREATE TRIGGER trg_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE users IS 'Utilisateurs de talky';
COMMENT ON COLUMN users.password_hash IS 'mot de passe';


--server

CREATE TABLE servers (
  id_serv UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name_serv VARCHAR(100) NOT NULL,
  owner_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT fk_server_owner
        FOREIGN KEY (owner_id)
        REFERENCES users(id_user)
        ON DELETE CASCADE
);

CREATE INDEX idx_servers_owner ON servers(owner_id);

CREATE TRIGGER trg_servers_updated_at
BEFORE UPDATE ON servers
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE servers IS 'Serveurs de discussion';

--servMem
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

--channel

CREATE TABLE channels (
  id_chan UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name_chan VARCHAR(100) NOT NULL,
  id_serv UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT fk_channel_server
        FOREIGN KEY (id_serv)
        REFERENCES servers(id_serv)
        ON DELETE CASCADE,

    CONSTRAINT unique_channel_name_per_server
        UNIQUE (name_chan, id_serv)
);

CREATE INDEX idx_channels_server ON channels(id_serv);

CREATE TRIGGER trg_channels_updated_at
BEFORE UPDATE ON channels
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE channels IS 'discussion par serveur';

--invitations

CREATE TABLE invitations (
  id_inv UUID PRIMARY KEY DEFAULT gen_random_uuid,
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

-- CREATE TABLE files (
--   id UUID PRIMARY KEY,
--   url TEXT NOT NULL,
--   user_id UUID REFERENCES users(id),
--   channel_id UUID REFERENCES channels(id),
--   type VARCHAR(20),
--   created_at TIMESTAMPTZ DEFAULT NOW()
-- );


DO $$
BEGIN
    RAISE NOTICE 'base talky créée';
    RAISE NOTICE 'psql: user, server, channel';
END $$;
