const UserBadges = () => {
    const badges = [
        { name: "Early Adopter", color: "bg-accent" },
        { name: "Builder", color: "bg-brand" },
        { name: "Open Source", color: "bg-secondary" },
    ];

    return (
        <div className="flex flex-col bg-surface rounded-xl shadow-lg p-6 w-full ">
            <h2 className="text-2xl font-semibold text-primary mb-2">Badges</h2>
            <div className="flex gap-2 flex-wrap">
                {badges.map((badge, i) => (
                    <span
                        key={i}
                        className={`text-sm px-3 py-1 rounded-full text-background font-semibold ${badge.color}`}
                    >
                        {badge.name}
                    </span>
                ))}
            </div>
        </div>
    );
};

export default UserBadges;
