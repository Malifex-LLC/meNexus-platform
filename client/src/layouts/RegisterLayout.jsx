import './RegisterLayout.css'


const RegisterLayout = ({children}) => {
    return (
        <div className='register-layout'>
            <header>
                enkitheymystik.com
            </header>
            <main className='register-layout__main-content'>
                {children}
            </main>
        </div>
    );
}

export default RegisterLayout;


