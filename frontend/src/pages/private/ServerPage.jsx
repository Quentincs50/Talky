"use client";
import {motion} from "motion/react";
import {useAuth} from "../../hooks/useAuth.js";
import {cn} from "../../lib/utils.js";
import {SidebarDemo} from "../../components/layout/SidebarDemo.jsx";
import {useNavigate, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import api from "../../config/axiosConfig.js";
import {Autocomplete, Card, CardBody, Spinner} from "@heroui/react";
import TextField from '@mui/material/TextField';
import {useDebounce} from 'react-use';
import serverService from "../../services/serverService.js";
import {PlaceholdersAndVanishInput} from "../../components/ui/placeholders-and-vanish-input.jsx";
import CreateServerButton from "../../components/ui/CreateServerButton.jsx";

const ServerPage = () => {

    const {user} = useAuth();
    const [servers, setServers] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [errorMessage, setErrorMessage] = useState("");
    const [query, setQuery] = useState('');
    const [serverName, setServerName] = useState("");
    const {serverId} = useParams();
    const [debouncedSearchTerm, setDebouncedSearchTerm] = useState('');
    const navigate = useNavigate();

    const placeholders = [
        "Find your server",
        "Start now !",
        "Create channel and share with your friends",
    ];

    // HOOK LIST SERVER
    useEffect(() => {
        fetchServers(debouncedSearchTerm);
    }, [debouncedSearchTerm]);

    // LIMIT API TIME RESPONSE
    useDebounce(() => setDebouncedSearchTerm(query), 300, [query])


    // FIND SERVER
    const fetchServers = async (query = '') => {
        setIsLoading(true);
        setErrorMessage('');

        try {
            const data = await serverService.listServers();

            if (!data || data.length === 0) {
                setErrorMessage('Aucun serveur trouvé');
                setServers([]);
                return;
            }

            const filtered = query
                ? data.filter(s => s.name_serv.toLowerCase().includes(query.toLowerCase()))
                : data;

            setServers(filtered);

        } catch (error) {
            console.error(`Error fetching servers: ${error}`);
            setErrorMessage(`Error fetching servers: ${error}`);
        } finally {
            setIsLoading(false);
        }
    }


    const createServer = async (e) => {
        e.preventDefault();
        if (!serverName.trim()) return;
        try {
            const newServer = await api.post("/server/createServer", {name: serverName});
            setServers(prev => [...prev, newServer]);
            setServerName("");
            fetchServers();
            console.log("Server created");
        } catch (error) {
            console.error('Error , fail to create server:', error);
        }
    }

    return (
        <div className={cn(
            "mx-auto flex flex-1 flex-col overflow-hidden rounded-md border border-neutral-200 bg-gray-100 md:flex-row dark:border-neutral-700 dark:bg-neutral-800",
            "h-screen w-screen"
        )}>

            {/* SIDEBAR */}
            <SidebarDemo/>

            {/* PAGE */}
            <div
                className="relative my-10 flex flex-col w-screen items-center justify-center">
                <div
                    className="absolute inset-y-0 left-0 h-full w-px bg-neutral-200/80 dark:bg-neutral-800/80">
                    <div
                        className="absolute top-0 h-40 w-px bg-gradient-to-b from-transparent via-blue-500 to-transparent"/>
                </div>
                <div
                    className="absolute inset-y-0 right-0 h-full w-px bg-neutral-200/80 dark:bg-neutral-800/80">
                    <div
                        className="absolute h-40 w-px bg-gradient-to-b from-transparent via-blue-500 to-transparent"/>
                </div>
                <div
                    className="absolute inset-x-0 bottom-0 h-px w-full bg-neutral-200/80 dark:bg-neutral-800/80">
                    <div
                        className="absolute mx-auto h-px w-40 bg-gradient-to-r from-transparent via-blue-500 to-transparent"/>
                </div>
                <div className="px-4 py-10 md:py-20">
                    <h1
                        className="relative z-10 mx-auto max-w-4xl text-center text-2xl font-bold text-slate-700 md:text-4xl lg:text-7xl dark:text-slate-300">
                        {"Discover your next favorite server"
                            .split(" ")
                            .map((word, index) => (
                                <motion.span
                                    key={index}
                                    initial={{opacity: 0, filter: "blur(4px)", y: 10}}
                                    animate={{opacity: 1, filter: "blur(0px)", y: 0}}
                                    transition={{
                                        duration: 0.3,
                                        delay: index * 0.1,
                                        ease: "easeInOut",
                                    }}
                                    className="mr-2 inline-block">
                                    {word}
                                </motion.span>
                            ))}
                    </h1>
                    <motion.div
                        initial={{
                            opacity: 0,
                        }}
                        animate={{
                            opacity: 1,
                        }}
                        transition={{
                            duration: 0.3,
                            delay: 1,
                        }}
                        className="relative z-10 mt-8 flex flex-wrap items-center justify-center gap-4">

                        <PlaceholdersAndVanishInput
                            placeholders={placeholders}
                            onChange={(e) => setQuery(e.target.value)}
                            onSubmit={() => fetchServers(query)}
                        />

                        <CreateServerButton createServer={createServer}
                                            serverName={serverName}
                                            setServerName={setServerName}
                        />

                    </motion.div>
                    <section
                        className="relative mt-20 rounded-3xl border border-neutral-200 bg-neutral-100 p-2 shadow-md dark:border-neutral-800 dark:bg-neutral-900 cursor-pointer"
                    >
                        <h2>All Servers</h2>

                        {isLoading ? (
                            <Spinner/>
                        ) : errorMessage ? (
                            <p className="text-red-500">{errorMessage}</p>
                        ) : (
                            <ul className="grid grid-cols-2 gap-6 sm:grid-cols-4 md:grid-cols-4">
                                {servers.map((s) => (
                                    <li key={s.id_serv}>
                                        <motion.div
                                            initial={{
                                                opacity: 0,
                                                y: 10,
                                            }}
                                            animate={{
                                                opacity: 1,
                                                y: 0,
                                            }}
                                            transition={{
                                                duration: 0.3,
                                                delay: 1.2,
                                            }}
                                            className="relative mt-20 rounded-3xl border border-neutral-200 bg-neutral-100 p-4 shadow-md dark:border-neutral-800 dark:bg-neutral-900 cursor-pointer"
                                            onClick={() => navigate(`/servers/${s.id_serv}`)}>
                                            <p>{s.name_serv}</p>
                                        </motion.div>
                                    </li>
                                ))}
                            </ul>
                        )}
                    </section>

                </div>
            </div>
        </div>
    );

}

export default ServerPage;