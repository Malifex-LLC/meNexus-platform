import './RegisterLayout.css'
import {Link} from "react-router-dom";
import Header from "../components/Header/Header.jsx";

const RegisterLayout = ({children}) => {
    return (
        <div className='register-layout'>
            <Header />
            <main className='register-layout__main-content'>
                {children}
            </main>
            <div className='register-layout__login-redirect'>
                Already have an account? <Link to="/login">Login!</Link>
            </div>
        </div>
    );
}

export default RegisterLayout;