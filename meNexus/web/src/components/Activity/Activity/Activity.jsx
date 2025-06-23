const Activity = ({description}) => {
    return (
        <div className={`p-4 py-5 rounded-xl text-xl md:text-xs xl:text-md 2xl:text-xl bg-surface text-foreground
        shadow-2xl`}>
            {description}
        </div>
    );
};

export default Activity;