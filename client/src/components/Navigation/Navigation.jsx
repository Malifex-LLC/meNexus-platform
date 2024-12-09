import './Navigation.css'
import {NavLink} from "react-router-dom";


const Navigation = () => {
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
            </ul>
        </nav>
    )
};

export default Navigation;