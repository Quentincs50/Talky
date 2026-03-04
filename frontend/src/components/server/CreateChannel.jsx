import { useState } from "react";
import api from "../../config/axiosConfig";

const CreateChannel = ({ serverId, refreshChannels }) => {
  const [name, setName] = useState("");

  const handleCreate = async () => {
    if (!name.trim()) return;

    try {
      await api.post(`/server/${serverId}/channels`, {
        name
      });

      setName("");
      refreshChannels();
    } catch (err) {
      console.error("Error creating channel:", err);
    }
  };

  return (
    <div className="bg-white p-6 rounded shadow mb-8">
      <h3 className="font-semibold mb-4">
        Create Channel
      </h3>

      <div className="flex gap-3">
        <input
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="border p-3 rounded flex-1"
          placeholder="Channel name"
        />
        <button
          onClick={handleCreate}
          className="bg-yellow-500 px-5 py-2 rounded text-white hover:bg-yellow-600"
        >
          Create
        </button>
      </div>
    </div>
  );
};

export default CreateChannel;
