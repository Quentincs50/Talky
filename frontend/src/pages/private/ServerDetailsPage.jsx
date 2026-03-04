"use client";

import { SidebarDemo } from "../../components/layout/SidebarDemo";
import MembersList from "../../components/server/MembersList";
import CreateChannel from "../../components/server/CreateChannel";
import { useAuth } from "../../hooks/useAuth";



import { useNavigate, useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import api from "../../config/axiosConfig";

const ServerDetailsPage = () => {
  const { user } = useAuth();
  const { serverId } = useParams();
  const navigate = useNavigate();

  const [isMember, setIsMember] = useState(false);
  const [isOwner, setIsOwner] = useState(false);
  const [isAdmin, setIsAdmin] = useState(false);

  const [inviteCode, setInviteCode] = useState(null);
  const [channels, setChannels] = useState([]);


  const [members, setMembers] = useState([]);
  const [server, setServer] = useState(null);
  const [loading, setLoading] = useState(true);


  const fetchServer = async () => {
    try {
      const res = await api.get("/server/listServers");
      const found = res.data.find(s => s.id_serv === serverId);
      setServer(found || null);
    } catch (err) {
      console.error("error fetch", err);
    }
  };
  const fetchMembers = async () => {
    if (!serverId || !user) return;
    try {
      const res = await api.get(`/server/${serverId}/members`);
      setMembers(res.data);


      const memberFound = res.data.some(
        m => m.id_user === user.id
      );
      setIsMember(memberFound);


      const ownerFound = res.data.some(
        m => m.id_user === user.id && m.role === "OWNER",
      );
      setIsOwner(ownerFound);

      const adminFound = res.data.some(
        m => m.id_user === user.id && m.role === "ADMIN"
      );

      setIsAdmin(adminFound);


    } catch (err) {
      console.error("erreur fetch members", err);
    } finally {
      setLoading(false);
    }

  };


  const fetchChannels = async () => {
    if (!serverId) return;

    try {
      const res = await api.get(`/server/${serverId}/channels`);
      setChannels(res.data);
    } catch (err) {
      console.error("Error fetching channels", err);
    }
  };

  const joinServer = async () => {
    if (!serverId) return;
    try {
      await api.post(`/server/${serverId}/join`, {});
      fetchMembers();
    } catch (err) {
      console.error("error join server", err);
      alert("Failed to join server");
    }
  };

  const deleteChannel = async (channelId) => {
    if (!window.confirm("Delete this channel?")) return;

    try {
      await api.delete(`/channel/${channelId}`);
      fetchChannels();
    } catch (err) {
      alert("Only admin or owner can delete channel");
    }
  };


  const leaveServer = async () => {
    if (!serverId) return;
    if (isOwner) return;
    if (!window.confirm("Leave this server?")) return;
    try {
      await api.delete(`/server/${serverId}/leave`);
      navigate("/servers")
    } catch (err) {
      console.error("Error leaving server:", err);
      navigate("/servers")
    }
  };


  const createInvitation = async () => {
    if (!serverId) return;
    try {
      const res = await api.post(
        `/invitations/${serverId}/invitations`,
        {}
      );
      setInviteCode(res.data.code);
    } catch (err) {
      console.error("Error creating invitation:", err);
      alert("Failed to create invitation");
    }
  };
  const deleteServer = async () => {
    if (!window.confirm("Are you sure you want to delete this server?")) return;

    try {
      await api.delete(`/server/deleteServer/${serverId}`);
      navigate("/servers");
    } catch (err) {
      alert("Only owner can delete this server");
    }
  };


  useEffect(() => {
    if (!serverId || !user) return;
    fetchServer();
    fetchMembers();
    fetchChannels();
  }, [serverId, user]);



  return (
    <div className="flex h-screen">
      <SidebarDemo />

      <div className="flex-1 p-8 bg-gray-50 overflow-y-auto">

        <h1 className="text-3xl font-bold mb-6">
          {server?.name_serv || "Loading..."}
        </h1>


        <CreateChannel serverId={serverId} refreshChannels={fetchChannels} />

        <div className="mt-6">
          <h2 className="text-xl font-semibold mb-4">Channels</h2>

          {channels.length === 0 ? (
            <p className="text-gray-500">No channels yet</p>
          ) : (
            <ul className="space-y-2">
              {channels.map(channel => (
                <li
                  key={channel.id_chan}
                  className="flex justify-between items-center bg-white p-3 rounded shadow"
                >
                  <span># {channel.name}</span>

                  {(isOwner) && (
                    <button
                      onClick={() => deleteChannel(channel.id_chan)}
                      className="text-red-500 hover:text-red-700"
                    >
                      Delete
                    </button>
                  )}
                </li>
              ))}
            </ul>
          )}
        </div>


        {loading ? (
          <p>Loading members...</p>
        ) : (
          <MembersList
            members={members}
            serverId={serverId}
            refreshMembers={fetchMembers}
            isOwner={isOwner}
          />
        )}
        <div className="mt-10 border-t pt-6 space-y-4">


          {!isMember ? (
            <button
              onClick={joinServer}
              className="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded"
            >
              Join Server
            </button>
          ) : !isOwner ? (
            <button
              onClick={leaveServer}
              className="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded"
            >
              Leave Server
            </button>
          ) : (
            <p className="text-gray-500 font-semibold">
              You are the owner of this server
            </p>
          )}

          {isOwner && (
            <div className="space-y-4">
              <div>
                <button
                  onClick={createInvitation}
                  className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded"
                >
                  Create Invitation
                </button>

                {inviteCode && (
                  <div className="mt-3 p-3 bg-gray-200 rounded">
                    <p className="text-sm font-semibold">Invite Code:</p>
                    <p>{inviteCode}</p>

                    <p className="mt-2 text-sm font-semibold">
                      Share Link:
                    </p>
                    <p>
                      http://localhost:5173/invite/{inviteCode}
                    </p>
                  </div>
                )}
              </div>

              <button
                onClick={deleteServer}
                className="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded"
              >
                Delete Server
              </button>
            </div>
          )}
        </div>

      </div>
    </div>
  );
};

export default ServerDetailsPage;



