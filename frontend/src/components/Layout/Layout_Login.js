import './Layout_Login.css'

const Layout_Login = ({children}) => {
    return (
        <div className='LayoutLogin'>
            <header>
                enkitheymystik.com
            </header>
            <main className='MainContentLogin'>
                {children}
            </main>
        </div>
    );
}

export default Layout_Login;


