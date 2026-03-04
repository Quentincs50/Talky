import { phone } from "../../assets/index.js";

const HeaderConv = ({ people }) => {
    return (
        <div className="bg-white border-b border-gray-200 px-6 py-4">
            <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                    <div className="w-12 h-12 rounded-full bg-gray-300 overflow-hidden flex-shrink-0">
                        {people.avatar ? (
                            <img
                                src={people.avatar}
                                alt={people.name}
                                className="w-full h-full object-cover"
                            />
                        ) : (
                            <div className="w-full h-full flex items-center justify-center bg-indigo-500 text-white text-lg font-semibold">
                                {people.name?.charAt(0).toUpperCase()}
                            </div>
                        )}
                    </div>

                    {/* Nom + Status */}
                    <div>
                        <h3 className="text-lg font-semibold text-gray-900">
                            {people.name}
                        </h3>
                        <div className="flex items-center gap-2">
                            <div className={`w-2 h-2 rounded-full ${
                                people.status === 'online' ? 'bg-green-500' : 'bg-gray-400'
                            }`} />
                            <p className="text-sm text-gray-600">
                                {people.status}
                            </p>
                        </div>
                    </div>
                </div>

                <button className="p-3 rounded-full hover:bg-gray-100 transition-colors">
                    <img
                        src={phone}
                        alt="call"
                        className="w-6 h-6"
                    />
                </button>
            </div>
        </div>
    );
};

export default HeaderConv;