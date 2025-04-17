import LoginLayout from "../../layouts/LoginLayout/LoginLayout.jsx";
import LoginForm from "../../components/LoginForm/LoginForm.jsx";

const LoginPage = () => {
    return(
        <LoginLayout>
            <div className="flex flex-col text-xl items-center w-md mb-4 bg-surface text-foreground rounded-2xl">
                <h1 className={`text-5xl text-center font-bold p-4`}>Welcome to meNexus</h1>
                <p className={`text-xl text-center font-light p-4`}>Login to your account to get started!</p>
                <LoginForm/>
            </div>
        </LoginLayout>
    );
}

export default LoginPage;