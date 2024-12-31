import './MessagesLayout.css'
import Header from '../../components/Header/Header.jsx'
import Navigation from "../../components/Navigation/Navigation.jsx";

const MessagesLayout = ({children}) => {
    return (
        <div className="messages-layout">
            <Header/>
            <Navigation/>
            <div className="messages-layout__main-content">
                {children}
            </div>
        </div>
    );
}

export default MessagesLayout;