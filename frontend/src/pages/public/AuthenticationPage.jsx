import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';

export default function AuthenticationPage() {
    const [isLogin, setIsLogin] = useState(true);
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [username, setUsername] = useState('');
    const navigate = useNavigate();
    
    const { login, register } = useAuth(); 

    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            if (isLogin) {
                await login(email, password);
                navigate('/servers'); 
            } else {
                await register(email, password, username);
                setIsLogin(true); 
            }
        } catch (error) {
            console.error("Erreur d'auth :", error);
            alert("Oups, l'authentification a échoué !");
        }
    };


    return (
        <div className="min-h-screen w-full flex items-center justify-center bg-gray-950 px-4">
            <div className="max-w-md w-full bg-gray-900 border border-gray-800 rounded-2xl shadow-2xl p-8">
                <div className="text-center mb-8">
                    <h2 className="text-3xl font-bold text-white">
                        {isLogin ? 'Bienvenue' : 'Créer un compte'}
                    </h2>
                </div>

                <form className="space-y-6" onSubmit={handleSubmit}>
                    {!isLogin && (
                        <div>
                            <label className="block text-sm mr-64 font-medium text-gray-300">Nom d'utilisateur</label>
                            <input 
                                type="text" 
                                value={username}
                                onChange={(e) => setUsername(e.target.value)}
                                className="mt-1 block w-full bg-gray-800 border border-gray-700 rounded-lg py-2 px-3 text-white focus:ring-2 focus:ring-indigo-500"
                                required/>
                        </div>
                    )}

                    <div>
                        <label className="block text-sm mr-96 font-medium text-gray-300">Email</label>
                        <input 
                            type="email" 
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            className="mt-1 block w-full bg-gray-800 border border-gray-700 rounded-lg py-2 px-3 text-white focus:ring-2 focus:ring-indigo-500"
                            required/>
                    </div>

                    <div>
                        <label className="block text-sm mr-72 font-medium text-gray-300">Mot de passe</label>
                        <input 
                            type="password" 
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            className="mt-1 block w-full bg-gray-800 border border-gray-700 rounded-lg py-2 px-3 text-white focus:ring-2 focus:ring-indigo-500"
                            required/>
                    </div>

                    <button type="submit" className="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded-lg transition-all">
                        {isLogin ? 'Se connecter' : "S'inscrire"}
                    </button>
                </form>

                <div className="mt-6 text-center">
                    <button onClick={() => setIsLogin(!isLogin)} className="text-indigo-400 hover:text-indigo-300 text-sm">
                        {isLogin ? "Pas encore de compte ? S'inscrire" : "Déjà un compte ? Se connecter"}
                    </button>
                </div>
            </div>
        </div>
    );
}