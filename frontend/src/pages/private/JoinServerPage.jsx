import { useState } from "react";
import { useNavigate } from "react-router-dom";
import api from "../../config/axiosConfig";

const JoinServerPage = () => {
  const [input, setInput] = useState("");
  const navigate = useNavigate();

  const extractCode = (value) => {
    
    if (value.includes("/invite/")) {
      return value.split("/invite/")[1];
    }
    
    return value;
  };

  const handleJoin = async () => {
    const code = extractCode(input.trim());

    if (!code) {
      alert("Please enter an invite link or code");
      return;
    }

    try {
      const res = await api.post(`/invitations/${code}/join`, {});
      const serverId = res.data.server_id;

      navigate(`/servers/${serverId}`);
    } catch (err) {
      console.error(err);
      alert("Invalid or expired invite");
    }
  };

  return (
    <div className="flex items-center justify-center h-screen bg-gray-100">
      <div className="bg-white p-8 rounded shadow-md w-96">
        <h2 className="text-xl font-bold mb-4">Join a Server</h2>

        <input
          type="text"
          placeholder="Paste invite link or code"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="w-full border p-2 rounded mb-4"
        />

        <button
          onClick={handleJoin}
          className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 rounded"
        >
          Join
        </button>
      </div>
    </div>
  );
};

export default JoinServerPage;
