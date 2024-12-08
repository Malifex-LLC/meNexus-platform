import './HomeLayout.css'
import Header from "../Header/Header.jsx";
import Navigation from "../Navigation/Navigation.jsx";
import Activity from "../Activity/Activity.jsx";



const HomeLayout = ({children}) => {
    return (
        <div className='LayoutHome'>
            <Header color={'black'}/>
            <Navigation color={'black'}/>
            <main className='MainContentHome'>
                {children}
            </main>
            <Activity color={"white"}/>
        </div>
    );
}

export default HomeLayout;


