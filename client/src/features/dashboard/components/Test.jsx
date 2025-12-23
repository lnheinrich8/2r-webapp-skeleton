import { useState, useEffect } from 'react';
import axios from 'axios';

import API_BASE_URL from '../../shared/utils/api';

// Styling
import './test.css';

const Test = () => {

    const [testData, setTestData] = useState(null);

    const handleTestButton = async () => {
        let email = "lnheinrich8@gmail.com";
        try {
            const res = await axios.get(
                `${API_BASE_URL}/user/getbyemail/${email}`,
                { withCredentials: true }
            );

            setTestData(res.data);
            console.log(res.data);
        } catch (err) {
            const serverError = err.response?.data?.error || "Unknown error occurred during getting user";
            console.error(`Fetching user by email failed: ${serverError}`);
        }
    }

    return (
        <div>
            <button className="test-button" onClick={handleTestButton}>Test</button>
        </div>
    )
}

export default Test;
