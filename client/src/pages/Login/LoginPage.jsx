import LoginLayout from "../../layouts/LoginLayout.jsx";
import LoginForm from "../../components/LoginForm/LoginForm.jsx";

const LoginPage = () => {
    return(
        <LoginLayout>
            <div className="register__main-content">
                <h1>Welcome to meNexus</h1>
                <p>Login to your account to get started!</p>
                <LoginForm/>
            </div>
        </LoginLayout>
    );
}

export default LoginPage;