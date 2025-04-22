import Activity from "../Activity/Activity.jsx";

const activityData = [
    {description: `Activity feed is coming soon!`},


]

const ActivityFeed = () => {
    return (
        <div className={`flex flex-col p-4  h-screen  bg-background text-foreground `}>
            <div className={`self-center text-3xl text-md lg:text-3xl pb-8`}>
                <h1>Activity</h1>
            </div>
            <div className={`self-center w-full h-1 mb-8 bg-border`}/>
            <div className={`w-full text-foreground`}>
                {activityData.map((activity, index) => (
                    <div key={index} className={`mb-4`}>
                        <Activity description={activity.description} />
                    </div>
                ))}

            </div>
        </div>
    )
};

export default ActivityFeed;