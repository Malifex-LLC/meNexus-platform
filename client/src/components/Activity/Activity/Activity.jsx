const Activity = ({description}) => {
    return (
        <div className={`p-4 rounded-2xl text-xl bg-surface text-foreground`}>
            {description}
        </div>
    );
};

export default Activity;