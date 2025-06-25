import {Link} from "react-router-dom";

const LoginLayoutPKI = ({children}) => {
    return (
        <div className='login-layout flex flex-col w-screen h-screen items-center justify-center  bg-background'>

            <main className='login-layout__main-content mb-8'>
                {children}
            </main>
            <div className='login-layout__register-redirect text-xl text-foreground'>
                Don't have an account? <Link to="/register" className={`text-brand`}>Register!</Link>
            </div>
        </div>
    );
}

export default LoginLayoutPKI;