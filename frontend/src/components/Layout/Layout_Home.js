import './Layout_Home.css'
import NavigationPanel from "../NavigationPanel/NavigationPanel";
import ActivityPanel from "../ActivityPanel/ActivityPanel";


const Layout_Home = ({children}) => {
    return (
        <div className='LayoutHome'>
            <header>
                meNexus
            </header>
            <NavigationPanel color={"black"}/>
            <main className='MainContentHome'>
                {children}
            </main>
            <ActivityPanel color={"white"}/>
        </div>
    );
}

export default Layout_Home;


