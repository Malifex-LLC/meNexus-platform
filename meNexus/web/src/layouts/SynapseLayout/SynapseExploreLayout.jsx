import Header from "../../components/Header/Header.jsx";

const SynapseExploreLayout =({ children }) => {
    return (
        <div className='h-screen flex flex-col bg-background'>
            <Header />
            <div
                className={`lg:flex flex-col pt-17 px-8 overflow-y-auto flex-1 w-full lg:col-span-6`}>
                {children}
            </div>

        </div>
    );
};

export default SynapseExploreLayout;