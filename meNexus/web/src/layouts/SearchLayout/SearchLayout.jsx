// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";

const SearchLayout = ({children}) => {
    return (
        <div className="search-layout h-full pt-17   bg-background text-foreground">
            <Header />
            <main className="search-layout__main-content ">
                {children}
            </main>
        </div>
    )
};

export default SearchLayout;