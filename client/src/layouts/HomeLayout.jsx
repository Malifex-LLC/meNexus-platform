import './HomeLayout.css'
import Header from "../components/Header/Header.jsx";
import Navigation from "../components/Navigation/Navigation.jsx";
import Activity from "../components/Activity/Activity.jsx";


const HomeLayout = ({children}) => {
    return (
        <div className='LayoutHome'>
            <Header/>
            <Navigation/>
            <main className='MainContentHome'>
                {children}
            </main>
            <Activity color={"white"}/>
        </div>
    );
}

export default HomeLayout;


