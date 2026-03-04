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