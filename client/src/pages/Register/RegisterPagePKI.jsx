import RegisterFormPKI from "../../components/RegisterForm/RegisterFormPKI.jsx";
import RegisterLayout from '../../layouts/RegisterLayout/RegisterLayout.jsx'

const RegisterPage = () => {
    return (
        <RegisterLayout>
            <div className="register__main-content">
                <h1>Welcome to meNexus</h1>
                <p>Create your account to get started!</p>
                <RegisterFormPKI />
            </div>
        </RegisterLayout>
    );
};

export default RegisterPage;