// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {IoLocationSharp} from "react-icons/io5";

const AboutUserPanel = ({user}) => {
    return (
        <div className={`flex flex-col p-4 bg-surface rounded-xl shadow-lg`}>
            <h1 className={`text-3xl text-brand`}>About {user.displayName}</h1>
            <div className={`mt-4`}>
                <div className={`flex flex-row mt-4 gap-4`}>
                    <h1 className={`text-xl text-primary`}>Bio:</h1>
                    <p className={`text-lg text-foreground`}>{user.bio}</p>
                </div>
                <div className={`flex flex-row mt-4 gap-4`}>
                    <h1 className={`text-xl text-primary`}>Location:</h1>
                    <div className={`text-md flex items-center gap-2`} >
                        <IoLocationSharp />
                        <p className="user-profile__location ">{user.location}</p>
                    </div>                </div>

            </div>
        </div>
    );
}

export default AboutUserPanel;