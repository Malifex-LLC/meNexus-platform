import * as React from 'react';
import {Link} from "react-router-dom";
import './NavDropdown.css'

interface NavDropdownProps {
    label: string;
    title: string;
    links: { label: string; to: string }[];
    activeMenu: string | null;
    setActiveMenu: (label: string | null) => void;
}

const NavDropdown: React.FC<NavDropdownProps> = ({ label, title, links, activeMenu, setActiveMenu }) => {
    return (
        <li
            className="nav__list-item relative text-md md:text-2xl font-light font-['Manrope', sans-serif]
            justify-center items-center"
            onMouseEnter={() => setActiveMenu(label)}
            onMouseLeave={() => setActiveMenu(null)}
        >
            <div className="nav__label cursor-pointer item-center justify-center p-0 m-0">{label}</div>
            {activeMenu === label && (
                <div className="nav__dropdown flex flex-row
                absolute
                mt-6
                w-[60vw] max-w-sm md:w-auto
                bg-header-bg opacity-90 rounded-lg shadow-lg z-[100] text-foreground text-3xl ">
                    <div className="dropdown__column px-4 py-2 mx-4 mb-4 items-start">
                        <h1 className="text-brand">{title}</h1>
                        <ul>
                            {links.map(({ label, to }) => (
                                <li
                                    key={label}
                                    className="list-none p-2 text-left text-xl cursor-pointer whitespace-nowrap
                                     hover:bg-brand  transition-colors duration-100"
                                >
                                    <Link to={to} className="block no-underline text-inherit w-full">
                                        {label}
                                    </Link>
                                </li>
                            ))}
                        </ul>
                    </div>
                </div>
            )}
        </li>
    );
};

export default NavDropdown;
