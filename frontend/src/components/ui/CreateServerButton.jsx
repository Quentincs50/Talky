import {IconCircleCheck, IconCircleX} from "@tabler/icons-react";
import {useState} from "react";

const CreateServerButton = ({createServer, serverName, setServerName}) => {
    const [isOpen, setIsOpen] = useState(false);

    const handleSubmit = (e) => {
        createServer(e);
        setIsOpen(false);
    };

    return (
        <>
            {/* BOUTON */}
            <button
                onClick={() => setIsOpen(true)}
                className="w-60 transform rounded-lg bg-black px-6 py-2 font-medium text-white transition-all duration-300 hover:-translate-y-0.5 hover:bg-gray-800 dark:bg-white dark:text-black dark:hover:bg-gray-200">
                Create a server
            </button>

            {/* MODAL */}
            {isOpen && (
                <>
                    {/* FOND BLUR */}
                    <div
                        className="fixed inset-0 z-40 bg-black/50 backdrop-blur-sm"
                        onClick={() => setIsOpen(false)}
                    />

                    {/* CONTENT */}
                    <div
                        className="fixed left-1/2 top-1/2 z-50 w-96 -translate-x-1/2 -translate-y-1/2 rounded-xl bg-white p-6 shadow-xl dark:bg-neutral-800">
                        <h2 className="mb-4 text-lg font-semibold text-black dark:text-white">
                            Create a server
                        </h2>
                        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
                            <input
                                type="text"
                                placeholder="Enter a name for your server"
                                value={serverName}
                                onChange={(e) => setServerName(e.target.value)}
                                className="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none focus:border-indigo-500 dark:border-neutral-600 dark:bg-neutral-700 dark:text-white"
                            />
                            <div className="flex justify-end gap-2">
                                <button
                                    type="button"
                                    onClick={() => setIsOpen(false)}
                                    className="flex items-center gap-1 rounded-lg px-4 py-2 text-red-500 hover:bg-red-100 bg-red-200 transition-colors dark:hover:bg-red-500/10">
                                    Cancel
                                    <IconCircleX className="h-4 w-4"/>
                                </button>
                                <button
                                    type="submit"
                                    className="flex items-center gap-1 rounded-lg bg-green-200 px-4 py-2 text-green-500 hover:bg-green-300 transition-colors">
                                    Create
                                    <IconCircleCheck className="h-4 w-4"/>
                                </button>
                            </div>
                        </form>
                    </div>
                </>
            )}
        </>
    );
};

export default CreateServerButton;