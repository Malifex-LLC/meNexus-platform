// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {Link} from "react-router-dom";

/* ---------------------------------------------------------------- helpers */

function makeDescription(a) {
    const actor = (a.actorId);

    switch (a.verb) {
        case 'POSTED':
            return (
                <>
                    <Link to={`/profile/${actor}`}/><span className={'text-brand cursor-pointer'}>@{actor}</span> posted in
                    <Link to={`/post/${a.containerId}`}/><span className={'text-accent cursor-pointer'}> {a.containerId}</span>
                </>
            );

        case 'COMMENTED':
            return (
                <>
                    <Link to={`/profile/${actor}`}/><span className={'text-brand cursor-pointer'}>@{actor}</span> commented on a
                    <Link to={`/post/${a.objectId}`}/><span className={'text-accent cursor-pointer'}> post</span> in <Link to={`/post/${a.containerId}`}/><span className={'text-accent cursor-pointer'}> {a.containerId}</span>
                </>
            );

        case 'FOLLOWED':
            return (
                <>
                    <Link to={`/profile/${actor}`}/><span className={'text-brand cursor-pointer'}>@{actor}</span> followed <Link to={`/profile/${a.objectId}`}/><span className={'text-brand cursor-pointer'}>@{a.objectId}</span>
                </>
            );

        case 'UNFOLLOWED':
            return (
                <>
                    <span className={'text-foreground'}>@{actor}</span> unfollowed <Link to={`/profile/${a.objectId}`}/><span className={'text-brand cursor-pointer'}>@{a.objectId}</span>
                </>
            );

        case 'JOINED':
            return (
                <>
                    <span className={'text-foreground'}>@{actor}</span> joined
                    <Link to={`/synapse/${a.containerId}`}/><span className={'text-accent cursor-pointer'}> {a.objectId}</span>
                </>
            );

        case 'LEFT':
            return (
                <>
                    <span className={'text-foreground'}>@{actor}</span> left
                    <Link to={`/synapse/${a.containerId}`}/><span className={'text-accent cursor-pointer'}> {a.objectId}</span>
                </>
            );

        default:
            return (
                <>
                    {actor} did something
                </>
            );
    }
}

/* -------------------------------------------------------------- component */
const Activity = ({ activity }) => {
    const description = makeDescription(activity);
    const date        = new Date(activity.timestamp); //

    return (
        <div className="p-4 py-5 rounded-xl text-xl md:text-xs xl:text-md 2xl:text-xl
                    bg-surface/40 text-foreground shadow-2xl">
            {description}
            <span className="block text-xs opacity-60 mt-1">
        {date.toLocaleString()}
      </span>
        </div>
    );
};

export default Activity;
