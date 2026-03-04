import React, { useState, useRef } from 'react';
import { useAuth } from '../../hooks/useAuth';
import {cn} from "../../lib/utils.js";
import { SidebarDemo } from '../../components/layout/SidebarDemo';


export default function ProfilePage() {
    const { user, updateUser } = useAuth();

    const [username, setUsername] = useState(user?.username || 'Donna');
    const [email, setEmail] = useState(user?.email || 'donna@exemple.com');
    const [status, setStatus] = useState(user?.status || "Disponible...");
    const [isEditing, setIsEditing] = useState(false);

    const [avatar, setAvatar] = useState(null);
    const fileInputRef = useRef(null);

    const [showPasswordForm, setShowPasswordForm] = useState(false);
    const [passwords, setPasswords] = useState({ old: '', new: '', confirm: '' });

    const handleSaveProfile = async (e) => {
        e.preventDefault();
        const result = await updateUser({ username, email, status });
        if (result.success) {
            setIsEditing(false);
        }
    };

    const handleAvatarChange = (e) => {
        const file = e.target.files[0];
        if (file) {
            setAvatar(URL.createObjectURL(file));
        }
    };

    return (
        <div className={cn(
            "mx-auto flex flex-1 flex-col overflow-hidden rounded-md border border-neutral-200 bg-gray-100 md:flex-row dark:border-neutral-700 dark:bg-neutral-800",
            "h-screen w-screen"
        )}>

            {/* SIDEBAR */}
            <SidebarDemo />
            <div className="h-screen w-screen bg-[#0f172a] text-slate-200 flex items-center justify-center p-4">
                <div className="max-w-5xl w-full bg-[#1e293b] rounded-3xl shadow-2xl border border-slate-700 overflow-hidden flex flex-col md:flex-row min-h-[600px]">

                    <div className="w-full md:w-1/3 bg-[#334155] p-8 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-slate-600">
                        <div className="relative group cursor-pointer" onClick={() => fileInputRef.current.click()}>
                            {avatar ? (
                                <img src={avatar} alt="Profile" className="w-40 h-40 rounded-full object-cover ring-4 ring-indigo-500 shadow-2xl" />
                            ) : (
                                <div className="w-40 h-40 bg-indigo-600 rounded-full flex items-center justify-center text-6xl font-black shadow-2xl ring-4 ring-slate-400">
                                    {username.charAt(0).toUpperCase()}
                                </div>
                            )}
                            <div className="absolute inset-0 bg-black/40 rounded-full opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity text-white text-sm font-bold">
                                Changer la photo
                            </div>
                            <input type="file" hidden ref={fileInputRef} onChange={handleAvatarChange} accept="image/*" />
                        </div>

                        <div className="mt-6 text-center">
                            <h1 className="text-2xl font-black text-white truncate w-48">{username}</h1>
                            <p className="text-indigo-300 font-medium italic mt-2">"{status}"</p>
                        </div>
                    </div>

                    <div className="w-full md:w-2/3 p-8 md:p-12 overflow-y-auto">
                        <div className="flex justify-between  mb-8">
                            <h2 className="text-sm uppercase tracking-[0.2em] font-black text-slate-400">Paramètres du compte</h2>
                            <button
                                onClick={() => setIsEditing(!isEditing)}
                                className="text-indigo-400 font-bold hover:text-indigo-300 transition-colors">
                                {isEditing ? "Annuler" : "Modifier"}
                            </button>
                        </div>

                        <form onSubmit={handleSaveProfile} className="space-y-6">
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div className="space-y-2">
                                    <label className="text-sm font-bold text-slate-500">Nom d'utilisateur</label>
                                    <input
                                        type="text" value={username} disabled={!isEditing}
                                        onChange={(e) => setUsername(e.target.value)}
                                        className="w-full bg-[#0f172a] border border-slate-700 rounded-xl p-3 text-white focus:border-indigo-500 outline-none transition-all disabled:opacity-50" />
                                </div>
                                <div className="space-y-2">
                                    <label className="text-sm font-bold text-slate-500">Email</label>
                                    <input
                                        type="email" value={email} disabled={!isEditing}
                                        onChange={(e) => setEmail(e.target.value)}
                                        className="w-full bg-[#0f172a] border border-slate-700 rounded-xl p-3 text-white focus:border-indigo-500 outline-none transition-all disabled:opacity-50" />
                                </div>
                            </div>

                            <div className="space-y-2">
                                <label className="text-sm font-bold text-slate-500">Statut personnalisé</label>
                                <input
                                    type="text" value={status} disabled={!isEditing}
                                    onChange={(e) => setStatus(e.target.value)}
                                    className="w-full bg-[#0f172a] border border-slate-700 rounded-xl p-3 text-white focus:border-indigo-500 outline-none transition-all disabled:opacity-50"
                                />
                            </div>

                            {isEditing && (
                                <button type="submit" className="w-full bg-indigo-600 hover:bg-indigo-500 py-3 rounded-xl font-black text-white shadow-lg transition-all active:scale-95 uppercase text-xs tracking-widest">
                                    Enregistrer les modifications
                                </button>
                            )}
                        </form>

                        <div className="mt-12 pt-8 border-t border-slate-700">
                            <div className="flex justify-between items-center mb-6">
                                <h2 className="text-sm uppercase tracking-[0.2em] font-black text-slate-400">Sécurité</h2>
                                {!showPasswordForm && (
                                    <button
                                        onClick={() => setShowPasswordForm(true)}
                                        className="text-xs font-black text-indigo-400 hover:underline">
                                        Changer le mot de passe
                                    </button>
                                )}
                            </div>

                            {showPasswordForm && (
                                <div className="space-y-4 animate-in fade-in slide-in-from-top-4 duration-300">
                                    <input
                                        type="password" placeholder="Ancien mot de passe"
                                        className="w-full bg-[#0f172a] border border-slate-700 rounded-xl p-3 text-sm outline-none focus:border-indigo-500"
                                        onChange={(e) => setPasswords({ ...passwords, old: e.target.value })} />
                                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        <input
                                            type="password" placeholder="Nouveau mot de passe"
                                            className="w-full bg-[#0f172a] border border-slate-700 rounded-xl p-3 text-sm outline-none focus:border-indigo-500"
                                            onChange={(e) => setPasswords({ ...passwords, new: e.target.value })} />

                                        <input
                                            type="password" placeholder="Confirmer"
                                            className="w-full bg-[#0f172a] border border-slate-700 rounded-xl p-3 text-sm outline-none focus:border-indigo-500"
                                            onChange={(e) => setPasswords({ ...passwords, confirm: e.target.value })} />
                                    </div>

                                    <div className="flex gap-3">
                                        <button className="flex-1 bg-slate-700 hover:bg-slate-600 py-2 rounded-lg text-xs font-bold transition-all">
                                            Mettre à jour
                                        </button>
                                        <button
                                            onClick={() => setShowPasswordForm(false)}
                                            className="flex-1 bg-transparent border border-slate-600 hover:bg-slate-800 py-2 rounded-lg text-xs font-bold transition-all">
                                            Annuler
                                        </button>
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}