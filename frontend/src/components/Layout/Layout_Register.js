import './Layout_Register.css'

const Layout_Register = ({children}) => {
    return (
        <div className='LayoutRegister'>
            <header>
                enkitheymystik.com
            </header>
            <main className='MainContentLogin'>
                {children}
            </main>
        </div>
    );
}

export default Layout_Register;


