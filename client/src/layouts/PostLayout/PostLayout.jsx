import Header from "../../components/Header/Header.jsx";

const PostLayout = ({children}) => {
    return (
        <div className='post-layout flex flex-col w-screen h-screen items-center justify-center  bg-background'>
            <div className='sticky top-0 z-50 border-b border-border'>
                <Header />
            </div>
            <main className='post-layout__main-content  '>
                {children}
            </main>
        </div>
    );
}

export default PostLayout;