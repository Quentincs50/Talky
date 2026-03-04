"use client";
import React, {useEffect, useState} from "react";
import {Sidebar, SidebarBody, SidebarLink} from "../ui/sidebar";
import {useNavigate} from "react-router-dom";

import {
    IconHash,
    IconMessageCircle,
    IconSpeakerphone,
    IconServer,
    IconArrowLeft,
    IconSettings,
    IconUserBolt, IconSquarePlus,
} from "@tabler/icons-react";
import {motion} from "motion/react";
import api from "../../config/axiosConfig.js";
import {useAuth} from "../../hooks/useAuth.js";
import {useParams} from "react-router-dom";
import {ICON_DIM} from "../../constants/style.js";
import serverService from "../../services/serverService.js";


export function SidebarDemo() {

    const {user, logout} = useAuth();
    const [servers, setServers] = useState([]);
    const [channels, setChannels] = useState([]);
    const {serverId} = useParams();
    const navigate = useNavigate();
    const [open, setOpen] = useState(false);

// HOOK LIST SERVER
    useEffect(() => {
        if (user) {
            api.get('/server/listServers')
                .then(res => setServers(res.data));
        }
    }, [user]);

// HOOK LIST CHANNEL
    useEffect(() => {
        if (user && serverId) {
            api.get(`/server/${serverId}/channels`)
                .then(res => setChannels(res.data));
        }
    }, [user, serverId]);


    const links = [
        {
            label: "Profile",
            href: `/profile/${user?.id}`,
            icon: (
                <IconUserBolt className={ICON_DIM}/>
            ),
        },
        {
            label: "Settings",
            href: `/profile/${user?.id}`,
            icon: (
                <IconSettings className={ICON_DIM}/>
            ),
        },
        {
            label: "Logout",
            href: "/authentication",
            icon: (
                <IconArrowLeft className={ICON_DIM}/>
            ),
            onClick: logout,
        },

    ];

    return (
        <Sidebar open={open} setOpen={setOpen}>
            <SidebarBody className="justify-between gap-10">
                <div className="flex flex-1 flex-col overflow-x-hidden">
                    {open ? <Logo/> : <LogoIcon/>}
                    <div className="mt-8 flex flex-col gap-2">
                        {links.map((link, idx) => (
                            <SidebarLink key={idx} link={link}/>
                        ))}
                    </div>

                    {/* SIDEBAR SEPARATOR */}
                    <div className="h-px w-full bg-neutral-300 dark:bg-neutral-600 my-2"/>

                    {/* SERVERS LOGO + LIST */}
                    {open ? <LogoServer/> : <LogoServerIcon/>}
                    <div className="mt-3 flex flex-col gap-1 h-40 overflow-y-auto">
                        {servers.map((s, idx) => (
                            <SidebarLink key={idx} link={{
                                label: `${s.name_serv}`,
                                href: `/servers/${s.id_serv}`,
                                icon: <IconHash className={ICON_DIM}/>,
                            }}/>
                        ))}
                        <SidebarLink
                            link={{
                                label: "Join Server",
                                href: "/join",
                                //icon: <IconSquarePlus className={ICON_DIM}/>,
                                icon: <IconSquarePlus className="h-5 w-5 text-green-500"/>
                            }}
                        />
                    </div>

                    {/* CHANNEL IN A SERVER */}
                    {serverId && (
                        <div>
                            {/* SIDEBAR SEPARATOR */}
                            <div className="h-px w-full bg-neutral-300 dark:bg-neutral-600 my-2"/>

                            {/* SERVERS CHANNEL + LIST */}
                            {open ? <LogoChannel/> : <LogoChannelIcon/>}
                            <div className=" mt-3 flex flex-col gap-2 h-40 overflow-y-auto">
                                {channels.map((c, idx) => (
                                    <SidebarLink key={idx} link={{
                                        label: `${c.name}`,
                                        href: `/servers/${serverId}/channel`,
                                        icon: <IconHash className={ICON_DIM}/>,
                                    }}/>
                                ))}
                            </div>
                        </div>
                    )}
                </div>

                {/* USER */}
                <div>
                    <SidebarLink
                        link={{
                            label: `${user?.username} [${user?.status}]`,
                            href: `/profile/${user?.id}`,
                            icon: (
                                <img
                                    src="https://assets.aceternity.com/manu.png"
                                    className="h-7 w-7 shrink-0 rounded-full"
                                    width={50}
                                    height={50}
                                    alt="Avatar"
                                />
                            ),
                        }}
                    />
                </div>
            </SidebarBody>
        </Sidebar>
    );
}

export const Logo = () => {
    return (
        <a
            href="#"
            className="relative z-20 flex items-center space-x-2 py-1 text-sm font-normal text-black">
            <div
                className="h-5 w-6 shrink-0 rounded-tl-lg rounded-tr-sm rounded-br-lg rounded-bl-sm bg-black dark:bg-white"/>
            <motion.span
                initial={{opacity: 0}}
                animate={{opacity: 1}}
                className="font-medium whitespace-pre text-black dark:text-white">
                Talky
            </motion.span>
        </a>
    );
};
export const LogoIcon = () => {
    return (
        <a
            href="#"
            className="relative z-20 flex items-center space-x-2 py-1 text-sm font-normal text-black">
            <div
                className="h-5 w-6 shrink-0 rounded-tl-lg rounded-tr-sm rounded-br-lg rounded-bl-sm bg-black dark:bg-white"/>
        </a>
    );
};

export const LogoServer = () => {
    return (
        <a
            href="/servers"
            className="mt-2 relative z-20 flex items-center space-x-2 py-1 text-sm font-normal text-black">
            <IconServer className={ICON_DIM}/>
            <motion.span
                initial={{opacity: 0}}
                animate={{opacity: 1}}
                className="font-medium whitespace-pre text-black dark:text-white">
                Servers
            </motion.span>

        </a>

    )
}

export const LogoServerIcon = () => {
    return (
        <a
            href="#"
            className="mt-2 relative z-20 flex items-center space-x-2 py-1 text-sm font-normal text-black">
            <IconServer className="h-5 w-5 shrink-0 text-neutral-700 dark:text-neutral-200"/>
        </a>
    );
};

export const LogoChannel = () => {
    return (
        <a
            href="#"
            className="mt-2 relative z-20 flex items-center space-x-2 py-1 text-sm font-normal text-black">
            <IconSpeakerphone className={ICON_DIM}/>
            <motion.span
                initial={{opacity: 0}}
                animate={{opacity: 1}}
                className="font-medium whitespace-pre text-black dark:text-white">
                Channels
            </motion.span>
        </a>
    )
}

export const LogoChannelIcon = () => {
    return (
        <a
            href="#"
            className="mt-2 relative z-20 flex items-center space-x-2 py-1 text-sm font-normal text-black">
            <IconSpeakerphone className={ICON_DIM}/>
        </a>
    );
};




