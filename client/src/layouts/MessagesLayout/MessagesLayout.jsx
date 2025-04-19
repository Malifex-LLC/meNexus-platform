import Header from '../../components/Header/Header.jsx'
import Navigation from "../../components/Navigation/Navigation.jsx";

const MessagesLayout = ({children}) => {

    return (
        <div className="messages-layout flex flex-col md:flex-row h-screen  pt-17 bg-background  ">
            <Header/>
            <div className="messages-layout__conversation-menu mb-8  ">
                {children[0] }
            </div>
            <div className="messages-layout__conversation-content flex-1 overflow-hidden md:mr-16 lg:mr-32">
                {children[1]}
            </div>
        </div>
    );
}

export default MessagesLayout;