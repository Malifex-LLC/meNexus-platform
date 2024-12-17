import './LoginLayout.css'
import {Link} from "react-router-dom";
import Header from "../../components/Header/Header.jsx";

const LoginLayout = ({children}) => {
    return (
        <div className='login-layout'>
            <Header/>
            <main className='login-layout__main-content'>
                {children}
            </main>
            <div className='login-layout__register-redirect'>
                Don't have an account? <Link to="/register">Register!</Link>
            </div>
        </div>
    );
}

export default LoginLayout;