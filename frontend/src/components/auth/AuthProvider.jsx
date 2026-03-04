import { useEffect, useState } from "react";
import api from "../../config/axiosConfig";
import { AuthContext } from "../../context/AuthContext";

export default function AuthProvider({ children }) {
    const [user, setUser] = useState(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const checkAuth = async () => {
            const token = localStorage.getItem("token");
            if (token) {
                try {
                    const res = await api.get("/auth/me");
                    setUser(res.data);
                } catch {
                    localStorage.removeItem("token");
                }
            }
            setLoading(false);
        };
        checkAuth();
    }, []);


    const login = async (email, password) => {
    try {
        const res = await api.post("/auth/login", { email, password });
        localStorage.setItem("token", res.data.token);
        
        const userData = res.data.user || res.data;
        setUser({
            id: userData.id_user || userData.id,
            username: userData.username,
            email: email,
            status: userData.status || 'online'
        });
    } catch (error) {
        console.error("Erreur de connexion détaillée :", error);
        throw error;
    }
};

 
    const updateUser = async (updatedData) => {
        try {
            await api.put("/auth/me", updatedData); 
            setUser(prev => ({ ...prev, ...updatedData }));
            return { success: true };
        } catch (error) {
            console.error("Erreur update:", error);
            setUser(prev => ({ ...prev, ...updatedData }));
            return { success: false }; 
        }
    };

    const register = async (email, password, username) => {
        await api.post("/auth/signup", { email, password, username });
    };

    const logout = () => {
        localStorage.removeItem("token");
        setUser(null);
    };

    const isAuthenticated = () => {
        return user !== null;
    };

    return (
        <AuthContext.Provider value={{ user, loading, login, register, logout, updateUser, isAuthenticated }}>
            {children}
        </AuthContext.Provider>
    );
}
