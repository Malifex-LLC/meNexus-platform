// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {Link} from "react-router-dom";

const RegisterLayout = ({children}) => {
    return (
        <div className='register-layout flex flex-col w-screen h-screen items-center justify-center  bg-background'>
            <main className='register-layout__main-content'>
                {children}
            </main>
            <div className='register-layout__login-redirect text-xl text-foreground'>
                Already have an account? <Link to="/login" className={'text-brand'}>Login!</Link>
            </div>
        </div>
    );
}

export default RegisterLayout;