import './LoginLayout.css'


const LoginLayout = ({children}) => {
    return (
        <div className='login-layout'>
            <header>
                meNexus
            </header>
            <main className='login-layout__main-content'>
                {children}
            </main>
        </div>
    );
}

export default LoginLayout;


