import './LoginLayoutPKI.css'
import {Link} from "react-router-dom";
import Header from "../../components/Header/Header.jsx";

const LoginLayoutPKI = ({children}) => {
    return (
        <div className='login-layout'>

            <main className='login-layout__main-content'>
                {children}
            </main>
            <div className='login-layout__register-redirect'>
                Don't have an account? <Link to="/register">Register!</Link>
            </div>
        </div>
    );
}

export default LoginLayoutPKI;