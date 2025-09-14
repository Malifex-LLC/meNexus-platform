import {FaGithub, FaGlobe} from "react-icons/fa";

const UserLinks = () => {
    const links = [
        {link: "https://malifex.dev", label: "Malifex", icon: <FaGlobe/>},
        {link: "https://github.com/Malifex-LLC/meNexus-platform", label: "GitHub", icon: <FaGithub/>},
    ]
    return (
        <div className="bg-surface rounded-xl shadow-lg p-4 border border-border">
            <h2 className="text-2xl font-semibold text-brand mb-2 font-montserrat">Links</h2>
            <div className="flex flex-col gap-2 font-inter">
                {links.map((link, index) => (
                    <span key={index} className="flex items-center px-2 py-2 gap-2 text-md font-semibold text-foreground hover:text-brand">
                        <span className={'text-primary'}>{link.icon}</span>
                        <a href={link.link} target={"_blank"} className={'font-normal text-foreground hover:text-brand'}>{link.link}</a>
                    </span>
                ))}
            </div>
        </div>
    )
}

export default UserLinks;