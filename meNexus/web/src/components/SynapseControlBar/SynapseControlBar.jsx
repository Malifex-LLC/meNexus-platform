import {Link, useLocation} from "react-router-dom";
import {RxDashboard} from "react-icons/rx";
import {FaNetworkWired} from "react-icons/fa6";
import {FaEnvelope} from "react-icons/fa";
import {IoPerson, IoSettings} from "react-icons/io5";
import { IoMdChatboxes } from "react-icons/io";
import { MdOutlineFeed } from "react-icons/md";



const SynapseControlBar = ({synapseMetadata}) => {
    const location = useLocation();
    const isActive = (path) => location.pathname.startsWith(path) ? "text-brand" : "text-foreground";
    return (
        <div className="w-full p-4 mt-10  gap-6 bg-header-bg text-foreground ">
            <div className={''}>
                {synapseMetadata ? (
                    <header className="pt-4 text-foreground">
                        <h1 className="text-3xl text-brand font-bold">{synapseMetadata.metadata.name}</h1>
                        <p className="text-lg text-foreground">
                            {synapseMetadata.metadata.description}
                        </p>
                        <p className="text-xs ">
                            publicKey: {synapseMetadata.identity.publicKey}
                        </p>
                    </header>
                ) : (
                    <div className="p-4">Loading synapse info…</div>
                )}
            </div>


        </div>
    )
}

export default SynapseControlBar;