import { Link } from "react-router-dom";
import { useMemo } from "react";

const makeDescription = ({ activity, actor, targetUser=null, targetSynapse=null, mode }) => {
    switch (activity.type) {
        case 'POSTED':
            return (
                <div>
                    <Link to={`/profile/${actor.handle}`}>
                        <span className="text-brand cursor-pointer hover:underline">@{actor.handle}</span>
                    </Link> created a
                    {mode === 'SYNAPSE' && (
                        <div>
                            <Link to={`/post/${activity.object_id}`}>
                                <span className="text-accent cursor-pointer hover:underline"> post</span>
                            </Link>
                        </div>
                    )}
                    {mode === 'GLOBAL' && (
                        <>
                            <Link to={`/post/${activity.object_id}`}>
                                <span className="text-secondary cursor-pointer hover:underline"> post</span>
                            </Link>
                            <> in </>
                            <Link to={`/synapse/${activity.context_id}`}>
                                <span className="text-accent cursor-pointer hover:underline">
                                    {targetSynapse?.metadata?.name || 'Unknown Synapse'}
                                </span>
                            </Link>
                        </>
                    )}
                </div>
            );
        case 'COMMENTED':
            return (
                <div>
                    <Link to={`/profile/${actor.handle}`}>
                        <span className="text-brand cursor-pointer hover:underline">@{actor.handle}</span>
                    </Link> commented on a
                    {mode === 'SYNAPSE' && (
                        <Link to={`/post/${activity.object_id}`}>
                            <span className="text-accent cursor-pointer hover:underline"> post</span>
                        </Link>
                    )}
                    {mode === 'GLOBAL' && (
                        <>
                            <Link to={`/post/${activity.object_id}`}>
                                <span className="text-secondary cursor-pointer hover:underline"> post</span>
                            </Link>
                            <> in </>
                            <Link to={`/synapse/${activity.context_id}`}>
                                <span className="text-accent cursor-pointer hover:underline">
                                    {targetSynapse?.metadata?.name || 'Unknown Synapse'}
                                </span>
                            </Link>
                        </>
                    )}
                </div>
            );
        case 'FOLLOWED':
            return (
                <div>
                    <Link to={`/profile/${actor.handle}`}>
                        <span className="text-brand cursor-pointer hover:underline">@{actor.handle}</span>
                    </Link> followed{' '}
                    <Link to={`/profile/${targetUser?.handle}`}>
                        <span className="text-accent cursor-pointer">@{targetUser?.handle}</span>
                    </Link>
                </div>
            );
        case 'UNFOLLOWED':
            return (
                <div>
                    <Link to={`/profile/${actor.handle}`}>
                        <span className="text-brand cursor-pointer hover:underline">@{actor.handle}</span>
                    </Link> unfollowed{' '}
                    <Link to={`/profile/${targetUser?.handle}`}>
                        <span className="text-accent cursor-pointer">@{targetUser?.handle}</span>
                    </Link>
                </div>
            );
        case 'JOINED':
            return (
                <div>
                    <Link to={`/profile/${actor.handle}`}>
                        <span className="text-brand cursor-pointer hover:underline">@{actor.handle}</span>
                    </Link> joined{' '}
                    <Link to={`/synapse/${activity.context_id}`}>
                        <span className="text-accent cursor-pointer hover:underline">
                            {targetSynapse?.metadata?.name || 'Unknown Synapse'}
                        </span>
                    </Link>
                </div>
            );
        case 'LEFT':
            return (
                <div>
                    <Link to={`/profile/${actor.handle}`}>
                        <span className="text-brand cursor-pointer hover:underline">@{actor.handle}</span>
                    </Link> left{' '}
                    <Link to={`/synapse/${activity.context_id}`}>
                        <span className="text-accent cursor-pointer hover:underline">
                            {targetSynapse?.metadata?.name || 'Unknown Synapse'}
                        </span>
                    </Link>
                </div>
            );
        default:
            return <div>{actor.handle} did something</div>;
    }
};

const Activity = ({ activity, mode, getUserById, getSynapseById }) => {
    const actor = useMemo(() => getUserById?.(activity.actor_public_key), [activity.actor_public_key, getUserById]);
    const targetUser = useMemo(() => {
        if (activity.type === 'FOLLOWED' || activity.type === 'UNFOLLOWED') {
            return getUserById?.(activity.target_id);
        }
        return null;
    }, [activity.type, activity.target_id, getUserById]);

    const targetSynapse = useMemo(() => {
        if (activity.context_type === 'SYNAPSE') return getSynapseById?.(activity.context_id);
        return null;
    }, [activity.context_type, activity.context_id, getSynapseById]);

    const description = useMemo(() => {
        if (!actor) return null;
        return makeDescription({ activity, actor, targetUser, targetSynapse, mode });
    }, [actor, targetUser, targetSynapse, activity, mode]);

    const date = new Date(activity.published);
    if (!description) return <div>Loading...</div>;

    return (
        <div className="flex flex-col w-full p-4 py-5 rounded-xl text-lg bg-background/70 text-foreground shadow-2xl">
            {description}
            <span className="block text-xs opacity-60 mt-1">{date.toLocaleString()}</span>
        </div>
    );
};

export default Activity;
