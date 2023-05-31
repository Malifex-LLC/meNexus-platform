import Layout_Login from "../Layout/Layout_Login";
import LoginWindow from "../LoginWindow/LoginWindow";
import React, { useState, useEffect } from 'react';
import axios from "axios";


const Login = () => {

    return(
        <Layout_Login>
            <LoginWindow/>
        </Layout_Login>



    );
}

export default Login;
