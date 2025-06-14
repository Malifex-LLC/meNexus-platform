import { NavLink } from "react-router-dom";
import {useNavigate} from "react-router-dom";
import useLogout from '../../api/hooks/useLogout.js'

const Navigation = () => {
    const { logout } = useLogout();
    const navigate = useNavigate(); // React Router navigate

    const handleLogout = (e) => {
        e.preventDefault();
        logout();
        navigate('/login');
    }

    return (
        <nav className='navigation flex h-screen w-sm text-2xl p-8 px-12
        bg-background '>
            <ul className={`text-white`}>
                <li className={`pt-4 hover:text-brand`}>
                    <NavLink to="/home" activeClassName="active">Home</NavLink>
                </li>
                <li className={`pt-4 hover:text-brand`}>
                    <NavLink to="/profile" activeClassName="active">Profile</NavLink>
                </li>
                <li className={`pt-4 hover:text-brand`}>
                    <NavLink to="/messages" activeClassName="active">Messages</NavLink>
                </li>
                <li className={`pt-4 hover:text-brand`}>
                    <NavLink to="/settings" activeClassName="active">Settings</NavLink>
                </li>
                <li className={`pt-4 hover:text-brand`}>
                    <NavLink to="/login" onClick={handleLogout} activeClassName="active">Logout</NavLink>
                </li>
            </ul>
        </nav>
    )
};

export default Navigation;