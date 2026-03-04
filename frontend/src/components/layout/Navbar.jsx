import { useEffect, useState } from "react";
import { NavLink } from "react-router-dom";
import api from "../../config/axiosConfig";
import { useAuth } from "../../hooks/useAuth";

export default function Navbar() {
    const {user} = useAuth()
    const [servers, setServers] = useState([]);
    const [form, setForm] = useState(false);
    const [server, setServer] = useState("");


    const handleCreateServer = async (e) => {
        e.preventDefault();
        if (!server.trim()) return;
        const response = await api.post("/server/createServer", { name: server });
        setServers([...servers, response.data])
        setServer("");
        setForm(false);
    }

    useEffect(() => {
        if(user) {
            api.get("/server/listServers").then((response) => setServers(response.data))
        }
    }, [user])

    return(
        
        <nav className="fixed top-0 left-0 z-40 w-64 h-screen flex flex-col justify-between bg-gray-900 border-r border-gray-800 p-4">
            <div className="flex-1 overflow-y-auto space-y-2">
                {servers.map((s) => (
                    <NavLink 
                        key={s.id_serv} 
                        to={`/servers/${s.id_serv}/members`}
                        className={({ isActive }) => 
                            `block px-3 py-2 rounded-lg text-sm truncate ${
                                isActive ? "bg-indigo-600 text-white" : "text-gray-300 hover:bg-gray-800"
                            }`
                        }
                    >
                        {s.name_serv}
                    </NavLink>
                ))}
            </div>
            <div>
                {form ? (
                    <form onSubmit={handleCreateServer} className="space-y-2">
                        <input
                            type="text"
                            value={server}
                            onChange={(e) => setServer(e.target.value)}
                            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white focus:ring-2 focus:ring-indigo-500 focus:outline-none"
                        />
                        <button type="submit" className="flex-1 bg-indigo-600 hover:bg-indigo-700 text-white text-sm py-1.5 rounded-lg">Créer</button>
                        <button type="button" onClick={() => setForm(false)} className="flex-1 bg-gray-800 hover:bg-gray-700 text-gray-300 text-sm py-1.5 rounded-lg">Annuler</button>
                    </form>
                ) : (
                    <button 
                        onClick={() => setForm(true)}
                        className="w-full py-2 rounded-lg border border-dashed border-gray-600 text-gray-400 hover:border-indigo-500 hover:text-indigo-400"
                    >+</button>
                )}
            </div>

            <div>
                {user && (
                    <NavLink to={`/profile/${user.id}`} className="border-t border-gray-800 pt-4 flex items-center gap-3">
                        <img src={user.profile_pic} className="w-10 h-10 rounded-full bg-gray-700"/>
                        <p className="text-sm text-white truncate">{user.username}</p>
                        <p lassName="text-xs text-gray-400 truncate">{user.email}</p>
                    </NavLink>
                )}
            </div>
            
            
        </nav>
    )
}