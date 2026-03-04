import { useParams, useNavigate } from "react-router-dom";
import { useEffect } from "react";
import api from "../../config/axiosConfig";

const InvitePage = () => {

  const { code } = useParams();
  const navigate = useNavigate();

  useEffect(() => {
    if (!code) return;
    joinWithCode();
  }, [code]);

  const joinWithCode = async () => {
    try {
      const res = await api.post(`/invitations/${code}/join`,{});

      navigate(`/servers/${res.data.server_id}`);

    } catch (err) {
      console.error("Error joining with invite:", err);
      alert("Invalid invite code");
      navigate("/servers");
    }
  };

  return <p>Joining...</p>;
};

export default InvitePage;
