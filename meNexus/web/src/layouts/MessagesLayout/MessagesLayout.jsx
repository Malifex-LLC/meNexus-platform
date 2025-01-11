import './MessagesLayout.css'
import Header from '../../components/Header/Header.jsx'
import Navigation from "../../components/Navigation/Navigation.jsx";

const MessagesLayout = ({children}) => {

    return (
        <div className="messages-layout">
            <Header/>
            <Navigation/>
            <div className="messages-layout__conversation-menu">
                {children[0] }
            </div>
            <div className="messages-layout__conversation-content">
                {children[1]}
            </div>
        </div>
    );
}

export default MessagesLayout;