import LoginLayoutPKI from "../../layouts/LoginLayout/LoginLayoutPKI.jsx";
import LoginForm from "../../components/LoginForm/LoginForm.jsx";

const LoginPage = () => {
    return(
        <LoginLayoutPKI>
            <div className="register__main-content">
                <h1>Welcome to meNexus</h1>
                <p>Login to your account to get started!</p>
                <LoginForm/>
            </div>
        </LoginLayoutPKI>
    );
}

export default LoginPage;