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