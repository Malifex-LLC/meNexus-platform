import './HomeLayout.css'
import Header from "../components/Header/Header.jsx";
import Navigation from "../components/Navigation/Navigation.jsx";
import Activity from "../components/Activity/Activity.jsx";


const HomeLayout = ({ children }) => {
    return (
        <div className='home-layout'>
            <Header />
            <Navigation />
            <main className='home-layout__main-content'>
                {children}
            </main>
            <Activity />
        </div>
    );
}

export default HomeLayout;