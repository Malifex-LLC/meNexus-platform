import './Navigation.css'
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
        <nav className='navigation' aria-label="Primary Navigation">
            <ul>
                <li>
                    <NavLink to="/home" activeClassName="active">Home</NavLink>
                </li>
                <li>
                    <NavLink to="/profile" activeClassName="active">Profile</NavLink>
                </li>
                <li>
                    <NavLink to="/messages" activeClassName="active">Messages</NavLink>
                </li>
                <li>
                    <NavLink to="/settings" activeClassName="active">Settings</NavLink>
                </li>
                <li>
                    <NavLink to="/login" onClick={handleLogout} activeClassName="active">Logout</NavLink>
                </li>
            </ul>
        </nav>
    )
};

export default Navigation;