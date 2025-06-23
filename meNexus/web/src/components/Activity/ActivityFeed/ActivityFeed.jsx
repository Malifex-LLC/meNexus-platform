import Activity from "../Activity/Activity.jsx";

const activityData = [
    {description: `Activity feed is coming soon!`},


]

const ActivityFeed = () => {
    return (
        <div className={`flex flex-col p-4  h-screen  bg-background text-foreground `}>
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