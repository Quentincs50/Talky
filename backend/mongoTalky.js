db = db.getSiblingDB('talky');

db.createCollection('messages', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['owner', 'content', 'message_type', 'location', 'location_id', 'createdAt', 'updatedAt'],
      properties: {
        _id: {
          bsonType: 'objectId'
        },
        owner: {
          bsonType: 'string'
        },
        content: {
          bsonType: 'string',
          minLength: 1,
          maxLength: 4000
        },
        message_type: {
          bsonType: 'string',
          enum: ['text', 'image', 'system']
        },
        location: {
          bsonType: 'string',
          enum: ['channel', 'private', 'server']
        },
        location_id: {
          bsonType: 'string'
        },
        edited: {
          bsonType: ['bool', 'null']
        },
        createdAt: {
          bsonType: 'date'
        },
        updatedAt: {
          bsonType: 'date'
        }
      }
    }
  }
});

db.messages.createIndex({ location: 1, location_id: 1, createdAt: 1 }, { name: 'idx_location_messages' });
db.messages.createIndex({ owner: 1, createdAt: 1 }, { name: 'idx_user_messages' });
db.messages.createIndex({ location: 1, owner: 1, location_id: 1, createdAt: 1 }, { name: 'idx_private_messages' });

db.createCollection('events', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['type', 'id_user', 'id_serv', 'created_at'],
      additionalProperties: false,
      properties: {
        type: {
          bsonType: 'string',
          enum: ['JOIN', 'LEAVE']
        },
        id_user: {
          bsonType: 'string'
        },
        id_server: {
          bsonType: 'string'
        },
        created_at: {
          bsonType: 'date'
        }
      }
    }
  }
});

db.events.createIndex({ id_server: 1, created_at: 1 }, { name: 'idx_events_server' });
db.events.createIndex({ id_user: 1, created_at: 1 }, { name: 'idx_events_user' });

db.createCollection('user_status', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['id_user', 'status', 'last_seen'],
      additionalProperties: false,
      properties: {
        id_user: {
          bsonType: 'string'
        },
        status: {
          bsonType: 'string',
          enum: ['ONLINE', 'OFFLINE']
        },
        last_seen: {
          bsonType: 'date'
        }
      }
    }
  }
});

db.user_status.createIndex({ id_user: 1 }, { unique: true });
db.user_status.createIndex({ status: 1 });
db.user_status.createIndex({ last_seen: 1 }, { expireAfterSeconds: 300, name: 'idx_user_status_ttl' });

print('messages, events, user_status créé');