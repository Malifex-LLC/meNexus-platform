import LoginLayoutPKI from "../../layouts/LoginLayout/LoginLayoutPKI.jsx";
import LoginFormPKI from "../../components/LoginForm/LoginFormPKI.jsx";

const LoginPagePKI = () => {
    return(
        <LoginLayoutPKI>
            <div className="register__main-content">
                <h1>Welcome to meNexus</h1>
                <p>Login to your account to get started!</p>
                <LoginFormPKI/>
            </div>
        </LoginLayoutPKI>
    );
}

export default LoginPagePKI;