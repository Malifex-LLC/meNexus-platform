// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {IoLocationSharp} from "react-icons/io5";
import {formatDateNoTime} from '../../utils/dateUtils.js'

const UserAbout = ({user}) => {

    return (
        <div className={`flex flex-col p-4 bg-surface rounded-xl shadow-lg border border-border`}>
            <h1 className={`text-3xl text-brand font-montserrat`}>About</h1>
            <div className={`flex flex-row mt-4 gap-4 px-2`}>
                <h2 className={`text-lg text-primary font-montserrat`}>Bio: </h2>
                <p className={`text-lg text-foreground font-inter`}>{user.bio}</p>
            </div>
            <div className={`flex flex-row mt-4 gap-4 px-2`} >
                <h2 className={`flex text-lg text-primary font-montserrat`}>Location: </h2>
                <p className="font-intert">{user.location}</p>
            </div>
            <div className={`flex flex-row mt-4 gap-4 px-2`} >
                <h2 className={`flex text-lg text-primary font-montserrat`}>Joined: </h2>
                <p className="font-inter">{formatDateNoTime(user.createdAt)}</p>
            </div>

        </div>
    );
}

export default UserAbout;