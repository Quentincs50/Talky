import api from "../../config/axiosConfig";

const MembersList = ({ members, serverId, refreshMembers, isOwner }) => {
  const changeRole = async (userId, newRole) => {
    try {
      console.log("PATCH", `/server/${serverId}/members/${userId}/role`, { id_user: userId, role: newRole });
      const res = await api.patch(`/server/${serverId}/members/${userId}/role`, {
        id_user: userId,
        role: newRole
      });
      console.log("Role updated:", res.data);
      refreshMembers();
    } catch (err) {
      console.error("Error updating role:", err.response?.data || err.message);
    }
  };


  const transferOwnership = async (newOwnerId) => {
    if (!window.confirm("Transfer ownership? This cannot be undone.")) return;

    try {
      await api.post(
        `/server/${serverId}/transfer/${newOwnerId}`
      );

      refreshMembers();

    } catch (err) {
      console.error("Transfer failed:", err);
    }
  };


  return (
    <div className="mt-10">
      <h2 className="text-xl font-semibold mb-4">
        Members
      </h2>

      {members.map(member => (
        <div
          key={member.id_user}
          className="flex justify-between items-center p-4 bg-white rounded mb-3 shadow"
        >
          <div>
            <p className="font-medium">{member.username}</p>
            <p className="text-sm text-gray-500">
              {member.role}
            </p>
          </div>
          {isOwner && member.role !== "OWNER" ? (
            <div className="flex gap-2">
              <select
                value={member.role}
                onChange={(e) =>
                  changeRole(member.id_user, e.target.value)
                }
                className="border p-2 rounded"
              >
                <option value="ADMIN">ADMIN</option>
                <option value="MEMBER">MEMBER</option>
              </select>

              <button
                onClick={() => transferOwnership(member.id_user)}
                className="bg-yellow-500 text-white px-3 rounded"
              >
                Transfer Owner
              </button>
            </div>
          ) : (
            <p className="font-semibold">{member.role}</p>
          )}

        </div>
      ))}
    </div>
  );
};

export default MembersList;
