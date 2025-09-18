// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const RegisterLayout = ({children}) => {
    return (
        <div className='flex flex-col w-screen h-screen items-center justify-center  bg-background'>
            <main className='flex mx-2'>
                {children}
            </main>
        </div>
    );
}

export default RegisterLayout;